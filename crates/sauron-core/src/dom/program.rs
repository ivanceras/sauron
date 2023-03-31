#[cfg(feature = "with-measure")]
use crate::dom::Measurements;
use crate::vdom;
use crate::{Application, Cmd, Dispatch};
use std::{any::TypeId, cell::RefCell, collections::BTreeMap, rc::Rc};
use wasm_bindgen::{closure::Closure, JsCast};
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen::JsValue;
use std::collections::VecDeque;
use crate::dom::created_node::ActiveClosure;
use crate::DomPatch;
use web_sys::{self,Element,Node};
use crate::vdom::{diff,AttributeValue, Attribute};
use mt_dom::TreePath;
use crate::CreatedNode;
use crate::dom::created_node;



/// Holds the user App and the dom updater
/// This is passed into the event listener and the dispatch program
/// will be called after the event is triggered.
pub struct Program<APP, MSG>
where
    MSG: 'static,
{
    /// holds the user application
    pub app: Rc<RefCell<APP>>,
    /// The MSG that hasn't been applied to the APP yet
    pub pending_msgs: Rc<RefCell<VecDeque<MSG>>>,

    /// pending cmds that hasn't been emited yet
    pub pending_cmds: Rc<RefCell<VecDeque<Cmd<APP,MSG>>>>,

    /// the current vdom representation
    pub current_vdom: Rc<RefCell<vdom::Node<MSG>>>,
    /// the first element of the app view, where the patch is generated is relative to
    pub root_node: Rc<RefCell<Option<Node>>>,

    /// the actual DOM element where the APP is mounted to.
    pub mount_node: Node,

    /// The closures that are currently attached to elements in the page.
    ///
    /// We keep these around so that they don't get dropped (and thus stop working);
    ///
    pub active_closures: Rc<RefCell<ActiveClosure>>,
    /// after mounting or update dispatch call, the element will be focused
    pub focused_node: Rc<RefCell<Option<Node>>>,

    /// if the mount node is replaced by the root_node
    pub replace: bool,

    /// whether or not to use shadow root of the mount_node
    pub use_shadow: bool,

    /// Pending patches that hasn't been applied to the DOM yet
    /// for optimization purposes to avoid sluggishness of the app, when a patch
    /// can not be run in 1 execution due to limited remaining time deadline
    /// it will be put into the pending patches to be executed on the next run.
    pub pending_patches: Rc<RefCell<VecDeque<DomPatch<MSG>>>>,
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
            mount_node: self.mount_node.clone(),
            active_closures: Rc::clone(&self.active_closures),
            focused_node: Rc::clone(&self.focused_node),
            replace: self.replace,
            use_shadow: self.use_shadow,
            pending_patches: Rc::clone(&self.pending_patches),
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
        replace: bool,
        use_shadow: bool,
    ) -> Self {
        let view = app.view();
        Program {
            app: Rc::new(RefCell::new(app)),
            pending_msgs: Rc::new(RefCell::new(VecDeque::new())),
            pending_cmds: Rc::new(RefCell::new(VecDeque::new())),
            current_vdom: Rc::new(RefCell::new(view)),
            root_node: Rc::new(RefCell::new(None)),
            mount_node: mount_node.clone(),
            active_closures: Rc::new(RefCell::new(ActiveClosure::new())),
            focused_node: Rc::new(RefCell::new(None)),
            replace,
            use_shadow,
            pending_patches: Rc::new(RefCell::new(VecDeque::new())),
        }
    }

    /// executed after the program has been mounted
    fn after_mounted(&self) {
        // call the init of the component
        let cmds = self.app.borrow_mut().init();
        // then emit the cmds, so it starts executing initial calls such (ie: fetching data,
        // listening to events (resize, hashchange)
        cmds.emit(self);

        // inject the style style after call the init of the app as
        // it may be modifying the app state including the style
        let style = self.app.borrow().style();
        if !style.trim().is_empty() {
            let type_id = TypeId::of::<APP>();
            Self::inject_style(type_id, &style);
        }
    }

    /// return the node where the app is mounted into
    pub fn mount_node(&self) -> web_sys::Node {
        self.mount_node.clone()
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
        let program = Self::new(app, mount_node, true, false);
        program.mount();
        program
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
        let program = Self::new(app, mount_node, false, false);
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
        Self::append_to_mount(app, &crate::body())
    }

    /// each element and it's descendant in the vdom is created into
    /// an actual DOM node.
    pub fn mount(&self)
    {
        let created_node = CreatedNode::create_dom_node(
            self,
            &self.current_vdom.borrow(),
            &mut self.focused_node.borrow_mut(),
        );

        //TODO: maybe remove replace the mount
        if self.replace {
            let mount_element: &Element = self.mount_node.unchecked_ref();
            mount_element
                .replace_with_with_node_1(&created_node.node)
                .expect("Could not append child to mount");
        } else if self.use_shadow {
            let mount_element: &web_sys::Element =
                self.mount_node.unchecked_ref();
            mount_element
                .attach_shadow(&web_sys::ShadowRootInit::new(
                    web_sys::ShadowRootMode::Open,
                ))
                .expect("unable to attached shadow");
            let mount_shadow =
                mount_element.shadow_root().expect("must have a shadow");

            let mount_shadow_node: &web_sys::Node =
                mount_shadow.unchecked_ref();

            mount_shadow_node
                .append_child(&created_node.node)
                .expect("could not append child to mount shadow");
        } else {
            self.mount_node
                .append_child(&created_node.node)
                .expect("Could not append child to mount");
        }
        *self.root_node.borrow_mut() = Some(created_node.node);
        *self.active_closures.borrow_mut() = created_node.closures;
        self.set_focus_element();
        self.after_mounted();
    }


    /// update the attributes at the mounted element
    pub fn update_mount_attributes(
        &self,
        attributes_value: BTreeMap<String, String>,
    ) {
        let mount_node = self.mount_node();
        let mount_element: &web_sys::Element = mount_node.unchecked_ref();
        for (attr, value) in attributes_value.iter() {
            mount_element
                .set_attribute(attr, value)
                .expect("unable to set attribute in the mount element");
        }
    }

    /// executes pending msgs by calling the app update method with the msgs
    /// as parameters
    ///
    /// TODO: maybe call the apply_pending_patches here to apply the some patches
    async fn dispatch_pending_msgs(&self, deadline: f64) ->Result<(), JsValue>{
        if self.pending_msgs.borrow().is_empty(){
            log::info!("no pending msgs... returning early..");
            return Ok(())
        }
        let mut i = 0;
        let t1 = crate::now();
        while let Some(pending_msg) = self.pending_msgs.borrow_mut().pop_front(){
            #[cfg(all(feature = "with-measure", feature = "with-debug"))]
            log::debug!("Executing pending msg item {}", i);
            let cmd = self.app.borrow_mut().update(pending_msg).await;
            self.pending_cmds.borrow_mut().push_back(cmd);
            let t2 = crate::now();
            let elapsed = t2 - t1;
            if elapsed > deadline{
                log::warn!("elapsed time: {}ms", elapsed);
                log::warn!("we should be breaking at {}..", i);
                break;
            }
            i += 1;
        }
        Ok(())
    }

    /// Diff the current virtual dom with the new virtual dom that is being passed in.
    ///
    /// Then use that diff to patch the real DOM in the user's browser so that they are
    /// seeing the latest state of the application.
    ///
    /// Return the total number of patches applied
    pub async fn update_dom(
        &self,
        new_vdom: vdom::Node<MSG>,
    ) -> Result<usize, JsValue>
    {

        let total_patches = self.patch_dom(
            &new_vdom
        )?;

        *self.current_vdom.borrow_mut() = new_vdom;

        self.set_focus_element();
        Ok(total_patches)
    }

    /// replace the current vdom with the `new_vdom`.
    pub fn set_current_dom(&self, new_vdom: vdom::Node<MSG>) {

        let created_node = CreatedNode::create_dom_node(
            self,
            &new_vdom,
            &mut self.focused_node.borrow_mut(),
        );
        self.mount_node
            .append_child(&created_node.node)
            .expect("Could not append child to mount");

        *self.root_node.borrow_mut() = Some(created_node.node);
        *self.active_closures.borrow_mut() = created_node.closures;

        *self.current_vdom.borrow_mut() = new_vdom;
    }

    /// Apply all of the patches to our old root node in order to create the new root node
    /// that we desire.
    /// This is usually used after diffing two virtual nodes.
    ///
    pub fn patch_dom(
        &self,
        new_vdom: &vdom::Node<MSG>,
    ) -> Result<usize, JsValue>
    {
        let current_vdom = self.current_vdom.borrow();
        let patches = diff(&current_vdom, new_vdom);
        // move the diff into patch function.
        let total_patches = patches.len();

        #[cfg(feature = "with-debug")]
        log::debug!("patches: {:#?}", patches);

        #[cfg(all(feature = "with-measure", feature = "with-debug"))]
        let t1 = crate::now();
        let nodes_to_find: Vec<(&TreePath, Option<&&'static str>)> = patches
            .iter()
            .map(|patch| (patch.path(), patch.tag()))
            .collect();

        let mut paths = vec![];
        for patch in patches.iter() {
            paths.push(patch.path());
        }

        let nodes_to_patch = created_node::find_all_nodes(self.root_node.borrow().as_ref().expect("must have a root node"), &nodes_to_find);
        #[cfg(all(feature = "with-measure", feature = "with-debug"))]
        let t2 = crate::now();

        #[cfg(all(feature = "with-measure", feature = "with-debug"))]
        log::info!("Took {}ms to find all the nodes", t2 - t1);

        for patch in patches.iter() {
            let patch_path = patch.path();
            let patch_tag = patch.tag();
            if let Some(target_node) = nodes_to_patch.get(patch_path) {
                //TODO: tests are panicking here!, so has to comment out the checking of tag names
                // check the tag here if it matches
                let target_element: &Element = target_node.unchecked_ref();
                if let Some(tag) = patch_tag {
                    let target_tag = target_element.tag_name().to_lowercase();
                    if target_tag != **tag {
                        panic!(
                            "expecting a tag: {:?}, but found: {:?}",
                            tag, target_tag
                        );
                    }
                }

                #[cfg(all(feature = "with-measure", feature = "with-debug"))]
                let t3 = crate::now();

                //TODO: push this into a vecqueue for executing the patches
                //taking into account deadline remaining time
                let dom_patch =
                    DomPatch::from_patch(self, target_node, &mut self.focused_node.borrow_mut(), patch);

                #[cfg(all(feature = "with-measure", feature = "with-debug"))]
                let t4 = crate::now();

                #[cfg(all(feature = "with-measure", feature = "with-debug"))]
                log::info!("Creating dom_patch took {}ms", t4 - t3);

                self.pending_patches.borrow_mut().push_back(dom_patch);
            } else {
                unreachable!("Getting here means we didn't find the element of next node that we are supposed to patch, patch_path: {:?}, with tag: {:?}", patch_path, patch_tag);
            }
        }


        self.apply_pending_patches()
            .expect("must not error");

        Ok(total_patches)
    }

    /// apply the pending patches into the DOM
    fn apply_pending_patches(
        &self,
    ) -> Result<(), JsValue>
    {
        if self.pending_patches.borrow().is_empty(){
            log::info!("No pending patches... returning..");
            return Ok(())
        }
        let deadline = 100.0;
        let t1 = crate::now();
        #[cfg(feature = "with-debug")]
        let mut cnt = 0;
        while let Some(dom_patch) = self.pending_patches.borrow_mut().pop_front() {
            #[cfg(feature = "with-debug")]
            log::debug!("Executing pending patch item {}", cnt);
            let t2 = crate::now();
            self.apply_dom_patch(dom_patch).expect("must apply dom patch");
            let elapsed = t2 - t1;
            if elapsed > deadline {
                log::info!("breaking here...");
                break;
            }
            #[cfg(feature = "with-debug")]
            {
                cnt += 1;
            }
        }
        #[cfg(all(feature = "with-measure", feature = "with-debug"))]
        let t3 = crate::now();
        #[cfg(all(feature = "with-measure", feature = "with-debug"))]
        log::info!("Pending patches took {}ms", t3 - t1);
        Ok(())
    }

    fn apply_dom_patch(&self, dom_patch: DomPatch<MSG>) -> Result<(), JsValue>{
        match dom_patch {
            DomPatch::InsertBeforeNode {
                target_node, nodes, ..
            } => {
                // we inser the node before this target element
                let target_element: &Element = target_node.unchecked_ref();
                if let Some(parent_target) = target_element.parent_node() {
                    for for_insert in nodes {
                        parent_target
                            .insert_before(
                                &for_insert.node,
                                Some(target_element),
                            )
                            .expect("must remove target node");

                        self.active_closures.borrow_mut().extend(for_insert.closures);
                    }
                } else {
                    panic!("unable to get parent node of the target element: {:?} for patching: {:#?}", target_element, nodes);
                }
            }

            DomPatch::InsertAfterNode {
                target_node, nodes, ..
            } => {
                // we insert the node before this target element
                let target_element: &Element = target_node.unchecked_ref();
                for for_insert in nodes.into_iter().rev() {
                    let created_element: &Element = for_insert
                        .node
                        .dyn_ref()
                        .expect("only elements is supported for now");
                    target_element
                        .insert_adjacent_element("afterend", created_element)
                        .expect("must remove target node");
                    self.active_closures.borrow_mut().extend(for_insert.closures);
                }
            }
            DomPatch::AppendChildren {
                target_node,
                children,
                ..
            } => {
                let target_element: &Element = target_node.unchecked_ref();
                for child in children.into_iter() {
                    target_element.append_child(&child.node)?;
                    self.active_closures.borrow_mut().extend(child.closures);
                }
            }

            DomPatch::AddAttributes {
                target_node, attrs, ..
            } => {
                let target_element: &Element = target_node.unchecked_ref();
                let attrs: Vec<&Attribute<MSG>> =
                    attrs.iter().map(|a| a).collect();
                CreatedNode::set_element_attributes(
                   self,
                    &mut self.active_closures.borrow_mut(),
                    target_element,
                    &attrs,
                );
            }
            DomPatch::RemoveAttributes {
                target_node, attrs, ..
            } => {
                let target_element: &Element = target_node.unchecked_ref();
                for attr in attrs.iter() {
                    for att_value in attr.value() {
                        match att_value {
                            AttributeValue::Simple(_) => {
                                CreatedNode::remove_element_attribute(
                                    target_element,
                                    attr,
                                )?;
                            }
                            // it is an event listener
                            AttributeValue::EventListener(_) => {
                                CreatedNode::remove_event_listener_with_name(
                                    attr.name(),
                                    target_element,
                                    &mut self.active_closures.borrow_mut(),
                                )?;
                            }
                            AttributeValue::FunctionCall(_)
                            | AttributeValue::Style(_)
                            | AttributeValue::Empty => (),
                        }
                    }
                }
            }

            // This also removes the associated closures and event listeners to the node being replaced
            // including the associated closures of the descendant of replaced node
            // before it is actully replaced in the DOM
            //
            DomPatch::ReplaceNode {
                patch_path,
                target_node,
                replacement,
            } => {
                let target_element: &Element = target_node.unchecked_ref();
                // FIXME: performance bottleneck here
                // Each element and it's descendant is created. Each call to dom to create the element
                // has a cost of ~1ms due to bindings in wasm-bindgen, multiple call of 1000 elements can accumulate to 1s time.
                //
                // Possible fix: stringify and process the patch in plain javascript code.
                // That way, all the code is done at once.
                if target_element.node_type() == Node::ELEMENT_NODE {
                    CreatedNode::remove_event_listeners(
                        target_element,
                        &mut self.active_closures.borrow_mut(),
                    )?;
                }
                target_element
                    .replace_with_with_node_1(&replacement.node)
                    .expect("must replace node");

                //Note: it is important that root_node points to the original mutable reference here
                // since it can be replaced with a new root Node(the top-level node of the view) when patching
                // if what we are replacing is a root node:
                // we replace the root node here, so that's reference is updated
                // to the newly created node
                if patch_path.path.is_empty() {
                    *self.root_node.borrow_mut() = Some(replacement.node);
                    #[cfg(feature = "with-debug")]
                    log::info!(
                        "the root_node is replaced with {:?}",
                        &self.root_node
                    );
                }
                self.active_closures.borrow_mut().extend(replacement.closures);
            }
            DomPatch::RemoveNode { target_node, .. } => {
                let target_element: &Element = target_node.unchecked_ref();
                let parent_target = target_element
                    .parent_node()
                    .expect("must have a parent node");
                parent_target
                    .remove_child(target_element)
                    .expect("must remove target node");
                if target_element.node_type() == Node::ELEMENT_NODE {
                    CreatedNode::remove_event_listeners(
                        target_element,
                        &mut self.active_closures.borrow_mut(),
                    )?;
                }
            }
        }
        Ok(())
    }

    fn set_focus_element(&self) {
        if let Some(focused_node) = &*self.focused_node.borrow() {
            let focused_element: &Element = focused_node.unchecked_ref();
            CreatedNode::set_element_focus(focused_element);
        }
    }


    /// execute DOM changes in order to reflect the APP's view into the browser representation
    async fn dispatch_dom_changes(&self, _log_measurements: bool, _measurement_name: &str, _msg_count: usize, _t1: f64) {
        #[cfg(feature = "with-measure")]
        let t2 = crate::now();

        // a new view is created due to the app update
        let view = self.app.borrow().view();
        #[cfg(all(feature = "with-measure", feature="with-debug"))]
        let t25 = crate::now();
        #[cfg(all(feature = "with-measure", feature="with-debug"))]
        log::info!("view took: {}ms", t25 - t2);

        #[cfg(feature = "with-measure")]
        let node_count = view.node_count();
        #[cfg(feature = "with-measure")]
        let t3 = crate::now();

        #[cfg(all(feature = "with-measure", feature="with-debug"))]
        log::info!("view and node count took: {}ms", t3 - t2);

        // update the last DOM node tree with this new view
        let _total_patches =
            self.update_dom(view).await.expect("must not error");
        #[cfg(feature = "with-measure")]
        let t4 = crate::now();

        #[cfg(feature = "with-measure")]
        {
            let dispatch_duration = t4 - _t1;
            #[cfg(all(feature = "with-measure", feature="with-debug"))]
             log::info!("dispatch took: {}ms", dispatch_duration);
            // 60fps is 16.667 ms per frame.
            if dispatch_duration > 16.0 {
                log::warn!("dispatch took: {}ms", dispatch_duration);
            }
        }

        #[cfg(feature = "with-measure")]
        if _log_measurements && _total_patches > 0 {
            let measurements = Measurements {
                name: _measurement_name.to_string(),
                msg_count: _msg_count,
                view_node_count: node_count,
                update_dispatch_took: t2 - _t1,
                build_view_took: t3 - t2,
                total_patches: _total_patches,
                dom_update_took: t4 - t3,
                total_time: t4 - _t1,
            };
            // tell the app on app performance measurements
            let cmd_measurement =
                self.app.borrow().measurements(measurements).no_render();
            cmd_measurement.emit(self);
        }
    }

    /// This is called when an event is triggered in the html DOM.
    /// The sequence of things happening here:
    /// - The app component update is executed.
    /// - The returned Cmd from the component update is then emitted.
    /// - The view is reconstructed with the new state of the app.
    /// - The dom is updated with the newly reconstructed view.
    ///
    ///
    /// TODO: split this function into 2.
    /// - update the app with msgs (use a request_idle_callback)
    /// - compute the view and update the dom (use request_animation_frame )
    async fn dispatch_inner(&self, deadline: f64) {
        let t1 = crate::now();

        let msg_count = 0;
        self.dispatch_pending_msgs(deadline).await.expect("must dispatch msgs");

        while let Some(cmd) = self.pending_cmds.borrow_mut().pop_front(){
            if cmd.modifier.should_update_view {
                let log_measurements = cmd.modifier.log_measurements;
                let measurement_name = &cmd.modifier.measurement_name;
                self.dispatch_dom_changes(log_measurements, measurement_name, msg_count, t1).await;
            }
            cmd.emit(self);
        }
    }

    /// Inject a style to the global document
    fn inject_style(type_id: TypeId, style: &str) {
        use wasm_bindgen::JsCast;
        dbg!(&type_id);
        let type_id = format!("{:?}", type_id);

        let document = crate::document();
        let html_style = document
            .create_element("style")
            .expect("must be able to create style element");
        html_style
            .set_attribute("class", &type_id)
            .expect("must set attribute");
        let html_style: web_sys::Node = html_style.unchecked_into();
        html_style.set_text_content(Some(style));
        let head = document.head().expect("must have a head");
        head.append_child(&html_style).expect("must append style");
    }

    /// inject style element to the mount node
    pub fn inject_style_to_mount<DSP>(&self, style: &str)
    {
        let style_node =
            crate::html::tags::style([], [crate::html::text(style)]);
        let created_node =
            CreatedNode::create_dom_node(self, &style_node, &mut None);
        if self.use_shadow {
            let mount_element: &web_sys::Element =
                self.mount_node.unchecked_ref();
            let mount_shadow =
                mount_element.shadow_root().expect("must have a shadow");

            let mount_shadow_node: &web_sys::Node =
                mount_shadow.unchecked_ref();

            mount_shadow_node
                .append_child(&created_node.node)
                .expect("could not append child to mount shadow");
        } else {
            panic!("injecting style to non shadow mount is not supported");
        }
    }


}

/// This will be called when the actual event is triggered.
/// Defined in the DomUpdater::create_closure_wrap function
impl<APP, MSG> Dispatch<MSG> for Program<APP, MSG>
where
    MSG: 'static,
    APP: Application<MSG> + 'static,
{
    ///TODO: store the msgs in a queue
    /// and an executor to execute those msgs
    /// with an alloted time of say 10ms to execute.
    /// if there is no more time, then dom patching should be commence and resume.
    fn dispatch_multiple(&self, msgs: Vec<MSG>) {
        self.pending_msgs.borrow_mut().extend(msgs);
        let program = self.clone();
        #[cfg(feature = "with-ric")]
        let _handle = crate::dom::util::request_idle_callback_with_deadline(move|deadline: f64|{
            log::info!("deadline: {:.2}", deadline);
            let program = program.clone();
            spawn_local(async move{
                program.dispatch_inner(deadline).await;
            });
        }).expect("must execute");

        #[cfg(not(feature = "with-ric"))]
        spawn_local(async move{
            program.dispatch_inner(10.0).await;
        });
    }



    fn dispatch(&self, msg: MSG) {
        self.dispatch_multiple(vec![msg])
    }

    fn dispatch_with_delay(&self, msg: MSG, timeout: i32) ->i32 {
        let program_clone = self.clone();
        crate::dom::util::delay_exec(
            Closure::once(move || {
                program_clone.dispatch(msg);
            }),
            timeout,
        )
    }
}
