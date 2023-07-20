use crate::dom::{document, now, Measurements};
use crate::vdom;
use crate::vdom::diff;
use crate::vdom::Patch;
use crate::dom::{Application, Cmd, util::body, DomPatch, IdleCallbackHandle, AnimationFrameHandle};
use crate::dom::dom_node::find_all_nodes;
use crate::html::{self, text,attributes::class};
use mt_dom::TreePath;
use std::collections::VecDeque;
use std::{any::TypeId, cell::RefCell, rc::Rc};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{self, Element, IdleDeadline, Node};
use std::collections::BTreeMap;
use wasm_bindgen::closure::Closure;
#[cfg(feature = "with-ric")]
use crate::dom::request_idle_callback;
#[cfg(feature = "with-raf")]
use crate::dom::request_animation_frame;


/// Program handle the lifecycle of the APP
pub struct Program<APP, MSG>
where
    MSG: 'static,
{
    /// holds the user application
    pub app: Rc<RefCell<APP>>,
    /// The MSG that hasn't been applied to the APP yet
    ///
    /// Note: MSG has to be executed in the same succession one by one
    /// since the APP's state may be affected by the previous MSG
    pending_msgs: Rc<RefCell<VecDeque<MSG>>>,

    /// pending cmds that hasn't been emited yet
    pending_cmds: Rc<RefCell<VecDeque<Cmd<APP, MSG>>>>,

    /// the current vdom representation
    current_vdom: Rc<RefCell<vdom::Node<MSG>>>,
    /// the first element of the app view, where the patch is generated is relative to
    pub(crate) root_node: Rc<RefCell<Option<Node>>>,

    /// the actual DOM element where the APP is mounted to.
    mount_node: Rc<RefCell<Node>>,

    /// The closures that are currently attached to all the nodes used in the Application
    /// We keep these around so that they don't get dropped (and thus stop working);
    pub node_closures: Rc<RefCell<ActiveClosure>>,

    /// specify how the root node is mounted into the mount node
    mount_procedure: MountProcedure,

    /// Pending patches that hasn't been applied to the DOM yet
    /// for optimization purposes to avoid sluggishness of the app, when a patch
    /// can not be run in 1 execution due to limited remaining time deadline
    /// it will be put into the pending patches to be executed on the next run.
    pending_patches: Rc<RefCell<VecDeque<DomPatch<MSG>>>>,

    /// store the Closure used in request_idle_callback calls
    idle_callback_handles: Rc<RefCell<Vec<IdleCallbackHandle>>>,
    /// store the Closure used in request_animation_frame calls
    animation_frame_handles: Rc<RefCell<Vec<AnimationFrameHandle>>>,

    /// event listener closures
    #[allow(clippy::type_complexity)]
    pub(crate) event_closures: Rc<RefCell<Vec<Closure<dyn FnMut(web_sys::Event)>>>>,
}

/// Closures that we are holding on to to make sure that they don't get invalidated after a
/// VirtualNode is dropped.
///
/// The usize is a unique identifier that is associated with the DOM element that this closure is
/// attached to.
pub type ActiveClosure = BTreeMap<usize, Vec<(&'static str, Closure<dyn FnMut(web_sys::Event)>)>>;

/// specify how the App is mounted to the DOM
#[derive(Clone, Copy)]
pub enum MountAction {
    /// append the APP's root node to the target mount node
    Append,
    /// clear any children of the target mount node then append the APP's root node
    ClearAppend,
    /// replace the target mount node with the APP's root node
    Replace,
}

/// specify whether to attach the Node in shadow_root
#[derive(Clone, Copy)]
pub enum MountTarget {
    /// attached in the mount node
    MountNode,
    /// attached to the shadow root
    ShadowRoot,
}

/// specify how the root node will be mounted to the mount node
#[derive(Clone, Copy)]
struct MountProcedure {
    action: MountAction,
    target: MountTarget,
}

impl<APP, MSG> Clone for Program<APP, MSG>
where
    MSG: 'static,
{
    fn clone(&self) -> Self {
        Program {
            app: Rc::clone(&self.app),
            pending_msgs: Rc::clone(&self.pending_msgs),
            pending_cmds: Rc::clone(&self.pending_cmds),
            current_vdom: Rc::clone(&self.current_vdom),
            root_node: Rc::clone(&self.root_node),
            mount_node: Rc::clone(&self.mount_node),
            node_closures: Rc::clone(&self.node_closures),
            mount_procedure: self.mount_procedure,
            pending_patches: Rc::clone(&self.pending_patches),
            idle_callback_handles: Rc::clone(&self.idle_callback_handles),
            animation_frame_handles: Rc::clone(&self.animation_frame_handles),
            event_closures: Rc::clone(&self.event_closures),
        }
    }
}

impl<APP, MSG> Program<APP, MSG>
where
    MSG: 'static,
    APP: Application<MSG> + 'static,
{
    /// Create an Rc wrapped instance of program, initializing DomUpdater with the initial view
    /// and root node, but doesn't mount it yet.
    pub fn new(
        app: APP,
        mount_node: &web_sys::Node,
        action: MountAction,
        target: MountTarget,
    ) -> Self {
        let view = app.view();
        Program {
            app: Rc::new(RefCell::new(app)),
            pending_msgs: Rc::new(RefCell::new(VecDeque::new())),
            pending_cmds: Rc::new(RefCell::new(VecDeque::new())),
            current_vdom: Rc::new(RefCell::new(view)),
            root_node: Rc::new(RefCell::new(None)),
            mount_node: Rc::new(RefCell::new(mount_node.clone())),
            node_closures: Rc::new(RefCell::new(ActiveClosure::new())),
            mount_procedure: MountProcedure { action, target },
            pending_patches: Rc::new(RefCell::new(VecDeque::new())),
            idle_callback_handles: Rc::new(RefCell::new(vec![])),
            animation_frame_handles: Rc::new(RefCell::new(vec![])),
            event_closures: Rc::new(RefCell::new(vec![])),
        }
    }

    /// executed after the program has been mounted
    fn after_mounted(&self) {
        // call the init of the component
        let cmds = self.app.borrow_mut().init();
        // then emit the cmds, so it starts executing initial calls such (ie: fetching data,
        // listening to events (resize, hashchange)
        cmds.into_iter().for_each(|cmd|cmd.emit(self));

        // inject the style style after call the init of the app as
        // it may be modifying the app state including the style
        let styles = self.app.borrow().style();
        if !styles.is_empty() {
            let type_id = TypeId::of::<APP>();
            let css_style = styles.join("");
            self.inject_style(type_id, &css_style);
        }
    }

    /// return the node where the app is mounted into
    pub fn mount_node(&self) -> web_sys::Node {
        self.mount_node.borrow().clone()
    }

    ///  Instantiage an app and append the view to the root_node
    /// # Example
    /// ```rust,ignore
    /// # use sauron::prelude::*;
    /// # use sauron::document;
    /// struct App{}
    /// # impl Application<()> for App{
    /// #     fn view(&self) -> Node<()>{
    /// #         text("hello")
    /// #     }
    /// #     fn update(&mut self, _: ()) -> Cmd<Self, ()> {
    /// #         Cmd::none()
    /// #     }
    /// # }
    /// let mount = document().query_selector("#app").ok().flatten().unwrap();
    /// Program::append_to_mount(App{}, &mount);
    /// ```
    pub fn append_to_mount(app: APP, mount_node: &web_sys::Node) -> Self {
        let program = Self::new(app, mount_node, MountAction::Append, MountTarget::MountNode);
        program.mount();
        program
    }

    /// Creates an Rc wrapped instance of Program and replace the root_node with the app view
    /// # Example
    /// ```rust,ignore
    /// # use sauron::prelude::*;
    /// # use sauron::document;
    /// struct App{}
    /// # impl Application<()> for App{
    /// #     fn view(&self) -> Node<()>{
    /// #         text("hello")
    /// #     }
    /// #     fn update(&mut self, _: ()) -> Cmd<Self, ()> {
    /// #         Cmd::none()
    /// #     }
    /// # }
    /// let mount = document().query_selector(".container").ok().flatten().unwrap();
    /// Program::replace_mount(App{}, &mount);
    /// ```
    pub fn replace_mount(app: APP, mount_node: &web_sys::Node) -> Self {
        let program = Self::new(
            app,
            mount_node,
            MountAction::Replace,
            MountTarget::MountNode,
        );
        program.mount();
        program
    }

    /// clear the existing children of the mount first before appending
    pub fn clear_append_to_mount(app: APP, mount_node: &web_sys::Node) -> Self {
        let program = Self::new(
            app,
            mount_node,
            MountAction::ClearAppend,
            MountTarget::MountNode,
        );
        program.mount();
        program
    }

    /// Instantiate the app and then append it to the document body
    /// # Example
    /// ```rust,ignore
    /// # use sauron::prelude::*;
    /// # use sauron::document;
    /// struct App{}
    /// # impl Application<()> for App{
    /// #     fn view(&self) -> Node<()>{
    /// #         text("hello")
    /// #     }
    /// #     fn update(&mut self, _: ()) -> Cmd<Self, ()> {
    /// #         Cmd::none()
    /// #     }
    /// # }
    /// Program::mount_to_body(App{});
    /// ```
    pub fn mount_to_body(app: APP) -> Self {
        Self::append_to_mount(app, &body())
    }

    /// each element and it's descendant in the vdom is created into
    /// an actual DOM node.
    pub fn mount(&self) {
        let created_node = self.create_dom_node(
            &self.current_vdom.borrow(),
        );

        let mount_node: web_sys::Node = match self.mount_procedure.target {
            MountTarget::MountNode => self.mount_node.borrow().clone(),
            MountTarget::ShadowRoot => {
                let mount_element: web_sys::Element =
                    self.mount_node.borrow().clone().unchecked_into();
                mount_element
                    .attach_shadow(&web_sys::ShadowRootInit::new(web_sys::ShadowRootMode::Open))
                    .expect("unable to attached shadow");
                let mount_shadow = mount_element.shadow_root().expect("must have a shadow");

                *self.mount_node.borrow_mut() = mount_shadow.unchecked_into();
                self.mount_node.borrow().clone()
            }
        };

        match self.mount_procedure.action {
            MountAction::Append => {
                Self::append_child_and_dispatch_mount_event(&mount_node,  &created_node);
            }
            MountAction::ClearAppend => {
                let mount_element: &Element = mount_node.unchecked_ref();
                log::debug!("mount_node: {:?}", mount_element.outer_html());
                let children = mount_node.child_nodes();
                log::debug!("There are {}", children.length());
                let child_nodes: Vec<Node> = (0..children.length())
                    .map(|i| children.item(i).expect("must have a child"))
                    .collect();

                child_nodes.into_iter().for_each(|child| {
                    mount_node.remove_child(&child).expect("must remove child");
                });

                Self::append_child_and_dispatch_mount_event(&mount_node, &created_node);
            }
            MountAction::Replace => {
                let mount_element: &Element = mount_node.unchecked_ref();
                mount_element
                    .replace_with_with_node_1(&created_node)
                    .expect("Could not append child to mount");
                Self::dispatch_mount_event(&created_node);
                *self.mount_node.borrow_mut() = created_node.clone()
            }
        }
        log::debug!("Root node is now set..");
        *self.root_node.borrow_mut() = Some(created_node);
        self.after_mounted();
    }

    fn dispatch_pending_msgs_with_ric(&self) {
        #[cfg(feature = "with-ric")]
        {
            let program = self.clone();
            let handle = request_idle_callback(move |deadline| {
                program
                    .dispatch_pending_msgs(Some(deadline))
                    .expect("must execute")
            })
            .expect("must execute");
            self.idle_callback_handles.borrow_mut().push(handle);
        }

        #[cfg(not(feature = "with-ric"))]
        {
            let program = self.clone();
            wasm_bindgen_futures::spawn_local(async move {
                program
                    .dispatch_pending_msgs(None)
                    .expect("must execute")
            })
        }
    }

    /// executes pending msgs by calling the app update method with the msgs
    /// as parameters.
    /// If there is no deadline specified all the pending messages are executed
    fn dispatch_pending_msgs(&self, deadline: Option<IdleDeadline>) -> Result<(), JsValue> {
        if self.pending_msgs.borrow().is_empty() {
            return Ok(());
        }
        let mut i = 0;
        let t1 = now();
        let mut did_complete = true;
        while let Some(pending_msg) = self.pending_msgs.borrow_mut().pop_front() {
            // Note: each MSG needs to be executed one by one in the same order
            // as APP's state can be affected by the previous MSG
            let cmd = self.app.borrow_mut().update(pending_msg);

            // we put the cmd in the pending_cmd queue
            self.pending_cmds.borrow_mut().push_back(cmd);

            let t2 = now();
            let elapsed = t2 - t1;
            // break only if a deadline is supplied
            if let Some(deadline) = &deadline {
                if deadline.did_timeout() {
                    log::warn!("elapsed time: {}ms", elapsed);
                    log::warn!("we should be breaking at {}..", i);
                    did_complete = false;
                    break;
                }
            }
            i += 1;
        }
        if !did_complete {
            self.dispatch_pending_msgs_with_ric();
        }
        Ok(())
    }

    /// update the browser DOM to reflect the APP's  view
    pub fn update_dom(&self) -> Result<Measurements, JsValue> {
        let t1 = now();
        // a new view is created due to the app update
        let view = self.app.borrow().view();
        let t2 = now();

        let node_count = view.node_count();

        // update the last DOM node tree with this new view
        let total_patches = self.update_dom_with_vdom(view).expect("must not error");
        let t3 = now();

        let measurements = Measurements {
            name: None,
            node_count,
            build_view_took: t2 - t1,
            total_patches,
            dom_update_took: t3 - t2,
            total_time: t3 - t1,
        };
        if measurements.total_time > 16.0 {
            #[cfg(all(feature = "with-measure", feature = "with-debug"))]
            log::warn!("dispatch took {}ms", measurements.total_time.round());
        }
        Ok(measurements)
    }

    /// patch the DOM to reflect the App's view
    pub fn update_dom_with_vdom(&self, new_vdom: vdom::Node<MSG>) -> Result<usize, JsValue> {
        let total_patches = {
            let current_vdom = self.current_vdom.borrow();
            let patches = diff(&current_vdom, &new_vdom);
            #[cfg(feature = "with-debug")]
            log::debug!("There are {} patches", patches.len());
            #[cfg(feature = "with-debug")]
            log::debug!("patches: {patches:#?}");
            let dom_patches = self
                .convert_patches(&patches)
                .expect("must convert patches");
            self.pending_patches.borrow_mut().extend(dom_patches);
            self.apply_pending_patches_using_raf();
            patches.len()
        };

        *self.current_vdom.borrow_mut() = new_vdom;
        Ok(total_patches)
    }

    /// replace the current vdom with the `new_vdom`.
    pub fn set_current_dom(&self, new_vdom: vdom::Node<MSG>) {
        let created_node =
            self.create_dom_node(&new_vdom);
        self.mount_node
            .borrow_mut()
            .append_child(&created_node)
            .expect("Could not append child to mount");

        *self.root_node.borrow_mut() = Some(created_node);
        *self.current_vdom.borrow_mut() = new_vdom;
    }

    /// get the real DOM target node and make a DomPatch object for each of the Patch
    fn convert_patches(&self, patches: &[Patch<MSG>]) -> Result<Vec<DomPatch<MSG>>, JsValue> {
        let nodes_to_find: Vec<(&TreePath, Option<&&'static str>)> = patches
            .iter()
            .map(|patch| (patch.path(), patch.tag()))
            .collect();

        let nodes_to_patch = find_all_nodes(
            self.root_node
                .borrow()
                .as_ref()
                .expect("must have a root node"),
            &nodes_to_find,
        );

        let dom_patches:Vec<DomPatch<MSG>> = patches.iter().map(|patch|{
            let patch_path = patch.path();
            let patch_tag = patch.tag();
            if let Some(target_node) = nodes_to_patch.get(patch_path) {
                let target_element: &Element = target_node.unchecked_ref();
                if let Some(tag) = patch_tag {
                    let target_tag = target_element.tag_name().to_lowercase();
                    if target_tag != **tag {
                        panic!(
                            "expecting a tag: {tag:?}, but found: {target_tag:?}"
                        );
                    }
                }
                self.convert_patch(target_element, patch)
            } else {
                unreachable!("Getting here means we didn't find the element of next node that we are supposed to patch, patch_path: {:?}, with tag: {:?}", patch_path, patch_tag);
            }
        }).collect();

        Ok(dom_patches)
    }


    fn apply_pending_patches_using_ric(&self)  {
        #[cfg(feature = "with-ric")]
        {
            let program = self.clone();
            let handle = request_idle_callback(move |deadline| {
                program
                    .apply_pending_patches(Some(deadline))
                    .expect("must not error");
            })
            .expect("must complete the remaining pending patches..");
            self.idle_callback_handles.borrow_mut().push(handle);
        }

        #[cfg(not(feature = "with-ric"))]
        {
            let program = self.clone();
            wasm_bindgen_futures::spawn_local(async move {
                program.apply_pending_patches(None).expect("must complete");
            })
        }
    }

    fn apply_pending_patches_using_raf(&self) {
        #[cfg(feature = "with-raf")]
        {
            let program = self.clone();
            let handle = request_animation_frame(move || {
                program.apply_pending_patches(None).expect("must not error");
            })
            .expect("must execute");
            self.animation_frame_handles.borrow_mut().push(handle);
        }

        #[cfg(not(feature = "with-raf"))]
        {
            let program = self.clone();
            wasm_bindgen_futures::spawn_local(async move {
                program.apply_pending_patches(None).expect("must complete");
            })
        }
    }

    /// apply the pending patches into the DOM
    fn apply_pending_patches(&self, deadline: Option<IdleDeadline>) -> Result<(), JsValue> {
        if self.pending_patches.borrow().is_empty() {
            return Ok(());
        }
        let mut did_complete = true;
        while let Some(dom_patch) = self.pending_patches.borrow_mut().pop_front() {
            self.apply_dom_patch(dom_patch)
                .expect("must apply dom patch");
            // only break if deadline is specified
            if let Some(deadline) = &deadline {
                if deadline.did_timeout() {
                    did_complete = false;
                    break;
                }
            }
        }
        if !did_complete {
            log::warn!("applying pending patches did not complete!..");
            self.apply_pending_patches_using_ric();
        }
        Ok(())
    }

    /// execute DOM changes in order to reflect the APP's view into the browser representation
    #[allow(unused_variables)]
    fn dispatch_dom_changes(&self, log_measurements: bool) {
        let measurements = self.update_dom().expect("must update dom");

        #[cfg(feature = "with-measure")]
        // tell the app about the performance measurement and only if there was patches applied
        if log_measurements && measurements.total_patches > 0 {
            let cmd_measurement = self.app.borrow().measurements(measurements).no_render();
            cmd_measurement.emit(self);
        }
    }

    fn dispatch_inner_with_ric(&self) {
        #[cfg(feature = "with-ric")]
        {
            let program = self.clone();
            let handle = request_idle_callback(move |deadline| {
                program.dispatch_inner(Some(deadline));
            })
            .expect("must execute");
            self.idle_callback_handles.borrow_mut().push(handle);
        }

        #[cfg(not(feature = "with-ric"))]
        {
            let program = self.clone();
            wasm_bindgen_futures::spawn_local(async move {
                program.dispatch_inner(None);
            })
        }
    }

    /// This is called when an event is triggered in the html DOM.
    /// The sequence of things happening here:
    /// - The app component update is executed.
    /// - The returned Cmd from the component update is then emitted.
    /// - The view is reconstructed with the new state of the app.
    /// - The dom is updated with the newly reconstructed view.
    fn dispatch_inner(&self, deadline: Option<IdleDeadline>) {
        self.dispatch_pending_msgs(deadline)
            .expect("must dispatch msgs");
        // ensure that all pending msgs are all dispatched already
        if !self.pending_msgs.borrow().is_empty() {
            self.dispatch_pending_msgs(None)
                .expect("must dispatch all pending msgs");
        }
        if !self.pending_msgs.borrow().is_empty() {
            panic!("Can not proceed until previous pending msgs are dispatched..");
        }

        let mut all_cmd = vec![];
        let mut pending_cmds = self.pending_cmds.borrow_mut();
        while let Some(cmd) = pending_cmds.pop_front() {
            all_cmd.push(cmd);
        }
        // we can execute all the cmd here at once
        let cmd = Cmd::batch(all_cmd);

        if !self.pending_patches.borrow().is_empty() {
            log::error!(
                "BEFORE DOM updates there are still Remaining pending patches: {}",
                self.pending_patches.borrow().len()
            );
        }

        if cmd.modifier.should_update_view {
            let log_measurements = cmd.modifier.log_measurements;
            self.dispatch_dom_changes(log_measurements);
        }

        // Ensure all pending patches are applied before emiting the Cmd from update
        if !self.pending_patches.borrow().is_empty() {
            self.apply_pending_patches(None)
                .expect("applying pending patches..");
        }

        if !self.pending_patches.borrow().is_empty() {
            log::error!(
                "Remaining pending patches: {}",
                self.pending_patches.borrow().len()
            );
            panic!(
                "There are still pending patches.. can not emit cmd, if all pending patches
            has not been applied yet!"
            );
        }
        cmd.emit(self);
    }

    /// Inject a style to the global document
    fn inject_style(&self, type_id: TypeId, style: &str) {
        let style_node = html::tags::style(
            [class(format!("{type_id:?}"))],
            [text(style)],
        );
        let created_node = self.create_dom_node(&style_node);

        let head = document().head().expect("must have a head");
        head.append_child(&created_node)
            .expect("must append style");
    }

    /// inject style element to the mount node
    pub fn inject_style_to_mount(&self, style: &str) {
        let style_node = html::tags::style([], [text(style)]);
        let created_node = self.create_dom_node(&style_node);

        self.mount_node
            .borrow_mut()
            .append_child(&created_node)
            .expect("could not append child to mount shadow");
    }
}

/// This will be called when the actual event is triggered.
/// Defined in the DomUpdater::create_closure_wrap function
impl<APP, MSG> Program<APP, MSG>
where
    MSG: 'static,
    APP: Application<MSG> + 'static,
{
    /// dispatch multiple MSG
    pub fn dispatch_multiple(&self, msgs: impl IntoIterator<Item = MSG>) {
        self.pending_msgs.borrow_mut().extend(msgs);
        self.dispatch_inner_with_ric();
    }

    /// dispatch a single msg
    pub fn dispatch(&self, msg: MSG) {
        self.dispatch_multiple([msg])
    }
}
