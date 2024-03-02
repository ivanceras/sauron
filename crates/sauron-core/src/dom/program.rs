use crate::dom::program::app_context::WeakContext;
#[cfg(feature = "with-raf")]
use crate::dom::request_animation_frame;
#[cfg(feature = "with-ric")]
use crate::dom::request_idle_callback;
#[cfg(feature = "pre-diff")]
use crate::dom::PreDiff;
use crate::dom::{document, now, IdleDeadline, Measurements, Modifier};
use crate::dom::{util::body, AnimationFrameHandle, Application, DomPatch, IdleCallbackHandle};
use crate::html::{self, attributes::class, text};
use crate::vdom;
use crate::vdom::diff;
use crate::vdom::KEY;
use app_context::AppContext;
use mt_dom::{diff_recursive, TreePath};
use std::collections::hash_map::DefaultHasher;
use std::collections::BTreeMap;
use std::collections::VecDeque;
use std::hash::{Hash, Hasher};
use std::mem::ManuallyDrop;
use std::{
    any::TypeId,
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
    rc::Weak,
};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{self, Element, Node};

mod app_context;

pub(crate) type EventClosures = Vec<Closure<dyn FnMut(web_sys::Event)>>;
pub(crate) type Closures = Vec<Closure<dyn FnMut()>>;

/// Program handle the lifecycle of the APP
pub struct Program<APP, MSG>
where
    MSG: 'static,
{
    pub(crate) app_context: AppContext<APP, MSG>,

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
    pub(crate) event_closures: Rc<RefCell<EventClosures>>,
    /// generic closures that has no argument
    pub closures: Rc<RefCell<Closures>>,
    last_update: Rc<RefCell<Option<f64>>>,
}

pub struct WeakProgram<APP, MSG>
where
    MSG: 'static,
{
    pub(crate) app_context: WeakContext<APP, MSG>,
    pub(crate) root_node: Weak<RefCell<Option<Node>>>,
    mount_node: Weak<RefCell<Node>>,
    pub node_closures: Weak<RefCell<ActiveClosure>>,
    mount_procedure: MountProcedure,
    pending_patches: Weak<RefCell<VecDeque<DomPatch<MSG>>>>,
    idle_callback_handles: Weak<RefCell<Vec<IdleCallbackHandle>>>,
    animation_frame_handles: Weak<RefCell<Vec<AnimationFrameHandle>>>,
    pub(crate) event_closures: Weak<RefCell<EventClosures>>,
    pub(crate) closures: Weak<RefCell<Closures>>,
    last_update: Weak<RefCell<Option<f64>>>,
}

/// Closures that we are holding on to to make sure that they don't get invalidated after a
/// VirtualNode is dropped.
///
/// The usize is a unique identifier that is associated with the DOM element that this closure is
/// attached to.
pub type ActiveClosure =
    BTreeMap<usize, BTreeMap<&'static str, Closure<dyn FnMut(web_sys::Event)>>>;

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

impl<APP, MSG> WeakProgram<APP, MSG>
where
    MSG: 'static,
{
    ///
    pub fn upgrade(&self) -> Option<Program<APP, MSG>> {
        if let Some(app_context) = self.app_context.upgrade() {
            if let Some(root_node) = self.root_node.upgrade() {
                if let Some(mount_node) = self.mount_node.upgrade() {
                    if let Some(node_closures) = self.node_closures.upgrade() {
                        if let Some(pending_patches) = self.pending_patches.upgrade() {
                            if let Some(idle_callback_handles) =
                                self.idle_callback_handles.upgrade()
                            {
                                if let Some(animation_frame_handles) =
                                    self.animation_frame_handles.upgrade()
                                {
                                    if let Some(event_closures) = self.event_closures.upgrade() {
                                        if let Some(closures) = self.closures.upgrade(){
                                            if let Some(last_update) = self.last_update.upgrade() {
                                                return Some(Program {
                                                    app_context,
                                                    root_node,
                                                    mount_node,
                                                    node_closures,
                                                    mount_procedure: self.mount_procedure,
                                                    pending_patches,
                                                    idle_callback_handles,
                                                    animation_frame_handles,
                                                    event_closures,
                                                    closures,
                                                    last_update,
                                                });
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        None
    }
}

impl<APP, MSG> Clone for WeakProgram<APP, MSG>
where
    MSG: 'static,
{
    fn clone(&self) -> Self {
        WeakProgram {
            app_context: self.app_context.clone(),
            root_node: Weak::clone(&self.root_node),
            mount_node: Weak::clone(&self.mount_node),
            node_closures: Weak::clone(&self.node_closures),
            mount_procedure: self.mount_procedure,
            pending_patches: Weak::clone(&self.pending_patches),
            idle_callback_handles: Weak::clone(&self.idle_callback_handles),
            animation_frame_handles: Weak::clone(&self.animation_frame_handles),
            event_closures: Weak::clone(&self.event_closures),
            closures: Weak::clone(&self.closures),
            last_update: Weak::clone(&self.last_update),
        }
    }
}

impl<APP, MSG> Program<APP, MSG>
where
    MSG: 'static,
{
    ///
    pub fn downgrade(&self) -> WeakProgram<APP, MSG> {
        WeakProgram {
            app_context: AppContext::downgrade(&self.app_context),
            root_node: Rc::downgrade(&self.root_node),
            mount_node: Rc::downgrade(&self.mount_node),
            node_closures: Rc::downgrade(&self.node_closures),
            mount_procedure: self.mount_procedure,
            pending_patches: Rc::downgrade(&self.pending_patches),
            idle_callback_handles: Rc::downgrade(&self.idle_callback_handles),
            animation_frame_handles: Rc::downgrade(&self.animation_frame_handles),
            event_closures: Rc::downgrade(&self.event_closures),
            closures: Rc::downgrade(&self.closures),
            last_update: Rc::downgrade(&self.last_update),
        }
    }
}

impl<APP, MSG> Clone for Program<APP, MSG>
where
    MSG: 'static,
{
    fn clone(&self) -> Self {
        Program {
            app_context: self.app_context.clone(),
            root_node: Rc::clone(&self.root_node),
            mount_node: Rc::clone(&self.mount_node),
            node_closures: Rc::clone(&self.node_closures),
            mount_procedure: self.mount_procedure,
            pending_patches: Rc::clone(&self.pending_patches),
            idle_callback_handles: Rc::clone(&self.idle_callback_handles),
            animation_frame_handles: Rc::clone(&self.animation_frame_handles),
            event_closures: Rc::clone(&self.event_closures),
            closures: Rc::clone(&self.closures),
            last_update: Rc::clone(&self.last_update),
        }
    }
}

impl<APP, MSG> Drop for Program<APP, MSG>
where
    MSG: 'static,
{
    fn drop(&mut self) {
        // program is dropped
    }
}

impl<APP, MSG> Program<APP, MSG>
where
    MSG: 'static,
{
    /// get a reference to the APP
    pub fn app(&self) -> Ref<'_, APP> {
        self.app_context.app.borrow()
    }

    /// get a mutable reference to the APP
    pub fn app_mut(&self) -> RefMut<'_, APP> {
        self.app_context.app.borrow_mut()
    }
}

impl<APP, MSG> Program<APP, MSG>
where
    MSG: 'static,
    APP: Application<MSG>,
{
    /// Create an Rc wrapped instance of program, initializing DomUpdater with the initial view
    /// and root node, but doesn't mount it yet.
    pub fn new(
        app: APP,
        mount_node: &web_sys::Node,
        action: MountAction,
        target: MountTarget,
    ) -> Self {
        Program {
            app_context: AppContext::new(app),
            root_node: Rc::new(RefCell::new(None)),
            mount_node: Rc::new(RefCell::new(mount_node.clone())),
            node_closures: Rc::new(RefCell::new(ActiveClosure::new())),
            mount_procedure: MountProcedure { action, target },
            pending_patches: Rc::new(RefCell::new(VecDeque::new())),
            idle_callback_handles: Rc::new(RefCell::new(vec![])),
            animation_frame_handles: Rc::new(RefCell::new(vec![])),
            event_closures: Rc::new(RefCell::new(vec![])),
            closures: Rc::new(RefCell::new(vec![])),
            last_update: Rc::new(RefCell::new(None)),
        }
    }

    /// executed after the program has been mounted
    fn after_mounted(&mut self) {
        // call the init of the component
        let cmd = self.app_context.init_app();
        cmd.emit(self.clone());

        // inject the app's dynamic style after the emitting the init function and it's effects
        self.inject_dynamic_style();
    }

    fn app_hash() -> u64 {
        let type_id = TypeId::of::<APP>();
        let mut hasher = DefaultHasher::new();
        type_id.hash(&mut hasher);
        hasher.finish()
    }

    fn inject_stylesheet(&mut self) {
        let static_style = self.app_context.static_style();
        if !static_style.is_empty() {
            let class_names = format!("static {}", Self::app_hash());
            self.inject_style(class_names, &static_style);
        }
    }

    fn inject_dynamic_style(&mut self) {
        let dynamic_style = self.app_context.dynamic_style();
        if !dynamic_style.is_empty() {
            let class_names = format!("dynamic {}", Self::app_hash());
            self.inject_style(class_names, &dynamic_style);
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
    pub fn append_to_mount(app: APP, mount_node: &web_sys::Node) -> ManuallyDrop<Self> {
        let mut program = Self::new(app, mount_node, MountAction::Append, MountTarget::MountNode);
        program.mount();
        ManuallyDrop::new(program)
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
    pub fn replace_mount(app: APP, mount_node: &web_sys::Node) -> ManuallyDrop<Self> {
        let mut program = Self::new(
            app,
            mount_node,
            MountAction::Replace,
            MountTarget::MountNode,
        );
        program.mount();
        ManuallyDrop::new(program)
    }

    /// clear the existing children of the mount before mounting the app
    pub fn clear_append_to_mount(app: APP, mount_node: &web_sys::Node) -> ManuallyDrop<Self> {
        let mut program = Self::new(
            app,
            mount_node,
            MountAction::ClearAppend,
            MountTarget::MountNode,
        );
        program.mount();
        ManuallyDrop::new(program)
    }

    /// clear the existing children of the mount before mounting the app
    pub fn clear_mount(app: APP, mount_node: &web_sys::Node) -> ManuallyDrop<Self> {
        Self::clear_append_to_mount(app, mount_node)
    }

    /// clear the existing children of the document body before mounting the app
    pub fn clear_mount_to_body(app: APP) -> ManuallyDrop<Self> {
        Self::clear_append_to_mount(app, &body())
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
    pub fn mount_to_body(app: APP) -> ManuallyDrop<Self> {
        Self::append_to_mount(app, &body())
    }

    /// executed right before the app is mounted to the dom
    pub fn pre_mount(&mut self) {
        self.inject_stylesheet();
    }

    /// each element and it's descendant in the vdom is created into
    /// an actual DOM node.
    pub fn mount(&mut self) {
        self.pre_mount();
        let created_node = self.create_dom_node(&self.app_context.current_vdom());

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
                Self::append_child_and_dispatch_mount_event(&mount_node, &created_node);
            }
            MountAction::ClearAppend => {
                Self::clear_children(&mount_node);
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
        *self.root_node.borrow_mut() = Some(created_node);
        self.after_mounted();
    }

    #[cfg(feature = "with-ric")]
    fn dispatch_pending_msgs_with_ric(&mut self) -> Result<(), JsValue> {
        let program = Program::downgrade(&self);
        let handle = request_idle_callback(move |deadline| {
            let mut program = program.upgrade().expect("must upgrade");
            program
                .dispatch_pending_msgs(Some(deadline))
                .expect("must execute")
        })
        .expect("must execute");
        self.idle_callback_handles.borrow_mut().push(handle);
        Ok(())
    }

    /// executes pending msgs by calling the app update method with the msgs
    /// as parameters.
    /// If there is no deadline specified all the pending messages are executed
    fn dispatch_pending_msgs(&mut self, deadline: Option<IdleDeadline>) -> Result<(), JsValue> {
        if !self.app_context.has_pending_msgs() {
            return Ok(());
        }
        let mut did_complete = true;
        while self.app_context.dispatch_pending_msg() {
            // break only if a deadline is supplied
            if let Some(deadline) = &deadline {
                if deadline.did_timeout() {
                    did_complete = false;
                    break;
                }
            }
        }
        if !did_complete {
            #[cfg(feature = "with-ric")]
            self.dispatch_pending_msgs_with_ric()
                .expect("must complete");
        }
        Ok(())
    }

    /// update the browser DOM to reflect the APP's  view
    pub fn update_dom(
        &mut self,
        modifier: &Modifier,
        treepath: Option<Vec<TreePath>>,
    ) -> Result<Measurements, JsValue> {
        let t1 = now();
        // a new view is created due to the app update
        let view = self.app_context.view();
        let t2 = now();

        let node_count = view.node_count();

        // update the last DOM node tree with this new view
        let total_patches = self
            .update_dom_with_vdom(view, treepath)
            .expect("must not error");
        let t3 = now();
        if let Some(last_update) = self.last_update.borrow().as_ref() {
            let frame_time = 1000.0 / 60.0; // 1s in 60 frames
            let time_delta = t3 - last_update;
            let remaining = frame_time - time_delta;
            if time_delta < frame_time {
                log::warn!(
                    "update is {} too soon!... time_delta: {}",
                    remaining,
                    time_delta
                );
            }
        }
        *self.last_update.borrow_mut() = Some(t3);

        let strong_count = self.app_context.strong_count();
        let weak_count = self.app_context.weak_count();
        let root_node_count = Rc::strong_count(&self.root_node);
        assert_eq!(strong_count, root_node_count);
        let measurements = Measurements {
            name: modifier.measurement_name.to_string(),
            node_count,
            build_view_took: t2 - t1,
            total_patches,
            dom_update_took: t3 - t2,
            total_time: t3 - t1,
            strong_count,
            weak_count,
        };

        if measurements.total_time > 16.0 {
            #[cfg(all(feature = "with-measure", feature = "with-debug"))]
            {
                log::warn!("dispatch took {}ms", measurements.total_time.round());
            }
        }
        Ok(measurements)
    }

    fn create_dom_patch(
        &self,
        new_vdom: &vdom::Node<MSG>,
        treepath: Option<Vec<TreePath>>,
    ) -> Vec<DomPatch<MSG>> {
        let current_vdom = self.app_context.current_vdom();
        let patches = if let Some(treepath) = treepath {
            log::debug!("using treepath from pre_eval: {treepath:?}");
            let patches = treepath
                .into_iter()
                .flat_map(|path| {
                    let new_node = path.find_node_by_path(new_vdom).expect("new_node");
                    let old_node = path.find_node_by_path(&current_vdom).expect("old_node");
                    log::debug!("new_node: {new_node:#?}");
                    log::debug!("old_node: {old_node:#?}");
                    diff_recursive(
                        &old_node,
                        &new_node,
                        &path,
                        &KEY,
                        &|_old, _new| false,
                        &|_old, _new| false,
                    )
                })
                .collect::<Vec<_>>();
            log::info!("patches: {patches:#?}");
            patches
        } else {
            log::debug!("using classic diff...");
            let patches = diff(&current_vdom, &new_vdom);
            patches
        };
        #[cfg(all(feature = "with-debug", feature = "log-patches"))]
        {
            log::debug!("There are {} patches", patches.len());
            log::debug!("patches: {patches:#?}");
        }
        let dom_patches = self
            .convert_patches(&patches)
            .expect("must convert patches");
        dom_patches
    }

    /// patch the DOM to reflect the App's view
    ///
    /// Note: This is in another function so as to allow tests to use this shared code
    pub fn update_dom_with_vdom(
        &mut self,
        new_vdom: vdom::Node<MSG>,
        treepath: Option<Vec<TreePath>>,
    ) -> Result<usize, JsValue> {
        let dom_patches = self.create_dom_patch(&new_vdom, treepath);
        let total_patches = dom_patches.len();
        self.pending_patches.borrow_mut().extend(dom_patches);

        #[cfg(feature = "with-raf")]
        self.apply_pending_patches_with_raf().expect("raf");

        #[cfg(not(feature = "with-raf"))]
        self.apply_pending_patches().expect("raf");

        self.app_context.set_current_dom(new_vdom);
        Ok(total_patches)
    }

    #[cfg(feature = "with-raf")]
    fn apply_pending_patches_with_raf(&mut self) -> Result<(), JsValue> {
        let program = Program::downgrade(&self);
        let handle = request_animation_frame(move || {
            let mut program = program.upgrade().expect("must upgrade");
            program.apply_pending_patches().expect("must not error");
        })
        .expect("must execute");
        self.animation_frame_handles.borrow_mut().push(handle);
        Ok(())
    }

    /// apply the pending patches into the DOM
    fn apply_pending_patches(&mut self) -> Result<(), JsValue> {
        if self.pending_patches.borrow().is_empty() {
            return Ok(());
        }
        let dom_patches: Vec<DomPatch<MSG>> = self.pending_patches.borrow_mut().drain(..).collect();
        for dom_patch in dom_patches {
            self.apply_dom_patch(dom_patch)
                .expect("must apply dom patch");
        }
        Ok(())
    }

    /// execute DOM changes in order to reflect the APP's view into the browser representation
    fn dispatch_dom_changes(&mut self, modifier: &Modifier, treepath: Option<Vec<TreePath>>) {
        #[allow(unused_variables)]
        let measurements = self
            .update_dom(modifier, treepath)
            .expect("must update dom");

        #[cfg(feature = "with-measure")]
        // tell the app about the performance measurement and only if there was patches applied
        if modifier.log_measurements && measurements.total_patches > 0 {
            let cmd_measurement = self.app_context.measurements(measurements);
            cmd_measurement.emit(self.clone());
        }
    }

    #[cfg(feature = "with-ric")]
    fn dispatch_inner_with_ric(&self) {
        let program = Program::downgrade(&self);
        let handle = request_idle_callback(move |deadline| {
            if let Some(mut program) = program.upgrade() {
                program.dispatch_inner(Some(deadline));
            } else {
                log::warn!("unable to upgrade program.. maybe try again next time..");
            }
        })
        .expect("must execute");
        self.idle_callback_handles.borrow_mut().push(handle);
    }

    #[allow(unused)]
    #[cfg(feature = "with-raf")]
    fn dispatch_inner_with_raf(&self) {
        let program = Program::downgrade(&self);
        let handle = request_animation_frame(move || {
            let mut program = program.upgrade().expect("must upgrade");
            program.dispatch_inner(None);
        })
        .expect("must execute");
        self.animation_frame_handles.borrow_mut().push(handle);
    }

    fn dispatch_inner_with_priority_ric(&self) {
        #[cfg(feature = "with-ric")]
        self.dispatch_inner_with_ric();
        #[cfg(not(feature = "with-ric"))]
        {
            #[cfg(feature = "with-raf")]
            self.dispatch_inner_with_raf();

            #[cfg(not(feature = "with-raf"))]
            {
                let program = Program::downgrade(&self);
                wasm_bindgen_futures::spawn_local(async move {
                    if let Some(mut program) = program.upgrade() {
                        program.dispatch_inner(None);
                    } else {
                        log::warn!(
                            "unable to upgrade program here, in dispatch_inner_with_priority_ric"
                        );
                    }
                })
            }
        }
    }

    /// clone the app
    #[cfg(feature = "pre-diff")]
    #[allow(unsafe_code)]
    pub fn app_clone(&self) -> ManuallyDrop<APP> {
        unsafe{
            let app: APP = std::mem::transmute_copy(&*self.app_context.app.borrow());
            //TODO: We are creating a copy of the app everytime,
            // as dropping the app will error in the runtime
            // This might be leaking the memory
            ManuallyDrop::new(app)
        }
        // An alternative to transmute_copy is to just plainly clone the app
        //let borrowed_app = self.app_context.app.borrow();
        //borrowed_app.clone()
    }

    /// This is called when an event is triggered in the html DOM.
    /// The sequence of things happening here:
    /// - The app component update is executed.
    /// - The returned Cmd from the component update is then emitted.
    /// - The view is reconstructed with the new state of the app.
    /// - The dom is updated with the newly reconstructed view.
    fn dispatch_inner(&mut self, deadline: Option<IdleDeadline>) {
        #[cfg(feature = "pre-diff")]
        let old_app = self.app_clone();
        self.dispatch_pending_msgs(deadline)
            .expect("must dispatch msgs");
        // ensure that all pending msgs are all dispatched already
        if self.app_context.has_pending_msgs() {
            self.dispatch_pending_msgs(None)
                .expect("must dispatch all pending msgs");
        }
        if self.app_context.has_pending_msgs() {
            panic!("Can not proceed until previous pending msgs are dispatched..");
        }

        #[cfg(feature = "pre-diff")]
        let treepath = self.app().pre_diff(&old_app).map(|eval| {
            log::debug!("eval: {eval:#?}");
            PreDiff::traverse(&eval)
        });

        #[cfg(not(feature = "pre-diff"))]
        let treepath = None;

        let cmd = self.app_context.batch_pending_cmds();

        if !self.pending_patches.borrow().is_empty() {
            log::error!(
                "BEFORE DOM updates there are still Remaining pending patches: {}",
                self.pending_patches.borrow().len()
            );
        }

        if cmd.modifier.should_update_view {
            self.dispatch_dom_changes(&cmd.modifier, treepath);
        }

        // Ensure all pending patches are applied before emiting the Cmd from update
        if !self.pending_patches.borrow().is_empty() {
            self.apply_pending_patches()
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

        cmd.emit(self.clone());
    }

    /// Inject a style to the global document
    fn inject_style(&mut self, class_names: String, style: &str) {
        let style_node = html::tags::style([class(class_names)], [text(style)]);
        let created_node = self.create_dom_node(&style_node);

        let head = document().head().expect("must have a head");
        head.append_child(&created_node).expect("must append style");
    }

    /// inject style element to the mount node
    pub fn inject_style_to_mount(&mut self, style: &str) {
        let style_node = html::tags::style([], [text(style)]);
        let created_node = self.create_dom_node(&style_node);

        self.mount_node
            .borrow_mut()
            .append_child(&created_node)
            .expect("could not append child to mount shadow");
    }

    /// dispatch multiple MSG
    pub fn dispatch_multiple(&mut self, msgs: impl IntoIterator<Item = MSG>) {
        self.app_context.push_msgs(msgs);
        self.dispatch_inner_with_priority_ric();
    }

    /// dispatch a single msg
    pub fn dispatch(&mut self, msg: MSG) {
        self.dispatch_multiple([msg])
    }
}
