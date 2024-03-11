#[cfg(feature = "use-template")]
use crate::dom::component::register_template;
use crate::dom::dom_node;
use crate::dom::program::app_context::WeakContext;
#[cfg(feature = "with-raf")]
use crate::dom::request_animation_frame;
#[cfg(feature = "with-ric")]
use crate::dom::request_idle_callback;
use crate::dom::template;
#[cfg(feature = "skip_diff")]
use crate::dom::SkipDiff;
use crate::dom::{document, now, IdleDeadline, Measurements, Modifier};
use crate::dom::{util::body, AnimationFrameHandle, Application, DomPatch, IdleCallbackHandle};
use crate::html::{self, attributes::class, text};
use crate::vdom;
use crate::vdom::diff;
use crate::vdom::{diff_recursive, TreePath};
use indexmap::IndexMap;
use std::collections::hash_map::DefaultHasher;
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

pub(crate) use app_context::AppContext;
pub use mount_procedure::{MountAction, MountProcedure, MountTarget};

mod app_context;
mod mount_procedure;

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
    pub(crate) mount_node: Rc<RefCell<Option<Node>>>,

    /// The closures that are currently attached to all the nodes used in the Application
    /// We keep these around so that they don't get dropped (and thus stop working);
    pub node_closures: Rc<RefCell<ActiveClosure>>,

    /// Pending patches that hasn't been applied to the DOM yet
    /// for optimization purposes to avoid sluggishness of the app, when a patch
    /// can not be run in 1 execution due to limited remaining time deadline
    /// it will be put into the pending patches to be executed on the next run.
    pub(crate) pending_patches: Rc<RefCell<VecDeque<DomPatch>>>,

    /// store the Closure used in request_idle_callback calls
    pub(crate) idle_callback_handles: Rc<RefCell<Vec<IdleCallbackHandle>>>,
    /// store the Closure used in request_animation_frame calls
    pub(crate) animation_frame_handles: Rc<RefCell<Vec<AnimationFrameHandle>>>,

    /// event listener closures
    pub(crate) event_closures: Rc<RefCell<EventClosures>>,
    /// generic closures that has no argument
    pub closures: Rc<RefCell<Closures>>,
    pub(crate) last_update: Rc<RefCell<Option<f64>>>,
}

pub struct WeakProgram<APP, MSG>
where
    MSG: 'static,
{
    pub(crate) app_context: WeakContext<APP, MSG>,
    pub(crate) root_node: Weak<RefCell<Option<Node>>>,
    mount_node: Weak<RefCell<Option<Node>>>,
    pub node_closures: Weak<RefCell<ActiveClosure>>,
    pending_patches: Weak<RefCell<VecDeque<DomPatch>>>,
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
    IndexMap<usize, micromap::Map<&'static str, Closure<dyn FnMut(web_sys::Event)>, 5>>;

impl<APP, MSG> WeakProgram<APP, MSG>
where
    MSG: 'static,
{
    ///
    pub fn upgrade(&self) -> Option<Program<APP, MSG>> {
        let app_context = self.app_context.upgrade()?;
        let root_node = self.root_node.upgrade()?;
        let mount_node = self.mount_node.upgrade()?;
        let node_closures = self.node_closures.upgrade()?;
        let pending_patches = self.pending_patches.upgrade()?;
        let idle_callback_handles = self.idle_callback_handles.upgrade()?;
        let animation_frame_handles = self.animation_frame_handles.upgrade()?;
        let event_closures = self.event_closures.upgrade()?;
        let closures = self.closures.upgrade()?;
        let last_update = self.last_update.upgrade()?;
        Some(Program {
            app_context,
            root_node,
            mount_node,
            node_closures,
            pending_patches,
            idle_callback_handles,
            animation_frame_handles,
            event_closures,
            closures,
            last_update,
        })
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
            pending_patches: Rc::clone(&self.pending_patches),
            idle_callback_handles: Rc::clone(&self.idle_callback_handles),
            animation_frame_handles: Rc::clone(&self.animation_frame_handles),
            event_closures: Rc::clone(&self.event_closures),
            closures: Rc::clone(&self.closures),
            last_update: Rc::clone(&self.last_update),
        }
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
    pub fn new(app: APP) -> Self {
        let app_view = app.view();
        #[cfg(feature = "use-template")]
        let (template, vdom_template) = register_template(TypeId::of::<APP>(), &app_view);
        let program = Program {
            app_context: AppContext {
                app: Rc::new(RefCell::new(app)),
                #[cfg(feature = "use-template")]
                template: template,
                #[cfg(feature = "use-template")]
                vdom_template: Rc::new(vdom_template),
                current_vdom: Rc::new(RefCell::new(app_view)),
                pending_msgs: Rc::new(RefCell::new(VecDeque::new())),
                pending_cmds: Rc::new(RefCell::new(VecDeque::new())),
            },
            root_node: Rc::new(RefCell::new(None)),
            mount_node: Rc::new(RefCell::new(None)),
            node_closures: Rc::new(RefCell::new(ActiveClosure::new())),
            pending_patches: Rc::new(RefCell::new(VecDeque::new())),
            idle_callback_handles: Rc::new(RefCell::new(vec![])),
            animation_frame_handles: Rc::new(RefCell::new(vec![])),
            event_closures: Rc::new(RefCell::new(vec![])),
            closures: Rc::new(RefCell::new(vec![])),
            last_update: Rc::new(RefCell::new(None)),
        };
        program
    }

    /// executed after the program has been mounted
    fn after_mounted(&mut self) {
        // call the init of the component
        let init_cmd = self.app_context.init_app();

        // this call may or may not trigger dispatch
        // as the initi app of Application
        // may just return Cmd::none which doesn't trigger
        // dispatching / redraw
        init_cmd.emit(self.clone());

        // inject the app's dynamic style after the emitting the init function and it's effects
        self.inject_dynamic_style();

        // first dispatch call to ensure the template is patched with the
        // new app real view
        //self.dispatch_multiple([]);
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
    pub fn mount_node(&self) -> Option<web_sys::Node> {
        if let Some(mount_node) = self.mount_node.borrow().as_ref() {
            Some(mount_node.clone())
        } else {
            None
        }
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
        let mut program = Self::new(app);
        program.mount(mount_node, MountProcedure::append());
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
        let mut program = Self::new(app);
        program.mount(mount_node, MountProcedure::replace());
        ManuallyDrop::new(program)
    }

    /// clear the existing children of the mount before mounting the app
    pub fn clear_append_to_mount(app: APP, mount_node: &web_sys::Node) -> ManuallyDrop<Self> {
        let mut program = Self::new(app);
        program.mount(mount_node, MountProcedure::clear_append());
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

    /// create initial dom node generated
    /// from template and patched by the difference of vdom_template and current app view.
    fn create_initial_view(&self) -> web_sys::Node {
        #[cfg(feature = "use-template")]
        {
            let app_view = self.app_context.app.borrow().view();
            let dom_template = self.app_context.template.clone();
            let vdom_template = &self.app_context.vdom_template;
            let patches = diff(vdom_template, &app_view);
            let dom_patches = self
                .convert_patches(&dom_template, &patches)
                .expect("convert patches");
            //log::info!("first time patches {}: {patches:#?}", patches.len());
            let new_template_node = self
                .apply_dom_patches(dom_patches)
                .expect("template patching");
            //log::info!("new template node: {:?}", new_template_node);
            dom_template
        }
        #[cfg(not(feature = "use-template"))]
        {
            self.create_dom_node(&self.app_context.current_vdom())
        }
    }

    /// each element and it's descendant in the vdom is created into
    /// an actual DOM node.
    pub fn mount(&mut self, mount_node: &web_sys::Node, mount_procedure: MountProcedure) {
        *self.mount_node.borrow_mut() = Some(mount_node.clone());
        self.pre_mount();

        let created_node = self.create_initial_view();

        let mount_node: web_sys::Node = match mount_procedure.target {
            MountTarget::MountNode => self
                .mount_node
                .borrow()
                .as_ref()
                .expect("mount node")
                .clone(),
            MountTarget::ShadowRoot => {
                let mount_element: web_sys::Element = self
                    .mount_node
                    .borrow()
                    .as_ref()
                    .expect("mount node")
                    .clone()
                    .unchecked_into();
                mount_element
                    .attach_shadow(&web_sys::ShadowRootInit::new(web_sys::ShadowRootMode::Open))
                    .expect("unable to attached shadow");
                let mount_shadow = mount_element.shadow_root().expect("must have a shadow");

                *self.mount_node.borrow_mut() = Some(mount_shadow.unchecked_into());
                self.mount_node
                    .borrow()
                    .as_ref()
                    .expect("mount_node")
                    .clone()
            }
        };

        match mount_procedure.action {
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
                *self.mount_node.borrow_mut() = Some(created_node.clone())
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
            log::info!("did not complete pending msgs in time.. dispatching the rest");
            #[cfg(feature = "with-ric")]
            self.dispatch_pending_msgs_with_ric()
                .expect("must complete");
        }
        Ok(())
    }

    /// execute DOM changes in order to reflect the APP's view into the browser representation
    pub fn update_dom(&mut self, modifier: &Modifier) -> Result<(), JsValue> {
        let t1 = now();
        // a new view is created due to the app update
        let view = self.app_context.view();
        let t2 = now();

        let node_count = view.node_count();

        let dom_patches = self.create_dom_patch(&view);
        let total_patches = dom_patches.len();
        // update the last DOM node tree with this new view
        self.queue_dom_patches(dom_patches).expect("must not error");
        /// set the current dom
        self.app_context.set_current_dom(view);
        let t3 = now();

        let strong_count = self.app_context.strong_count();
        let weak_count = self.app_context.weak_count();
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

        #[cfg(all(feature = "with-measure", feature = "with-debug"))]
        if let Some(last_update) = self.last_update.borrow().as_ref() {
            let frame_time = (1000_f64 / 60_f64).floor(); // 1s in 60 frames
            let time_delta = t3 - last_update;
            let _remaining = frame_time - time_delta;
            if time_delta < frame_time {
                //log::warn!("update is {remaining} too soon!... time_delta: {time_delta}, frame_time: {frame_time}");
                // return early here
                return Ok(measurements);
            }
        }

        let total = dom_node::total_time_spent();
        //log::info!("total: {:#?}", total);
        //log::info!("average: {:#?}", total.average());
        //log::info!("percentile: {:#?}", total.percentile());

        #[cfg(feature = "with-measure")]
        // tell the app about the performance measurement and only if there was patches applied
        if modifier.log_measurements && measurements.total_patches > 0 {
            let cmd_measurement = self.app_context.measurements(measurements);
            cmd_measurement.emit(self.clone());
        }

        *self.last_update.borrow_mut() = Some(t3);
        Ok(())
    }

    /// patch the DOM to reflect the App's view
    ///
    /// Note: This is in another function so as to allow tests to use this shared code
    pub fn queue_dom_patches(&mut self, dom_patches: Vec<DomPatch>) -> Result<(), JsValue> {
        self.pending_patches.borrow_mut().extend(dom_patches);

        #[cfg(feature = "with-raf")]
        self.apply_pending_patches_with_raf().expect("raf");

        #[cfg(not(feature = "with-raf"))]
        self.apply_pending_patches().expect("raf");

        Ok(())
    }

    fn create_dom_patch(&self, new_vdom: &vdom::Node<MSG>) -> Vec<DomPatch> {
        let current_vdom = self.app_context.current_vdom();
        let patches = diff(&current_vdom, new_vdom);

        #[cfg(all(feature = "with-debug", feature = "log-patches"))]
        {
            log::debug!("There are {} patches", patches.len());
            log::debug!("patches: {patches:#?}");
        }

        self.convert_patches(
            self.root_node
                .borrow()
                .as_ref()
                .expect("must have a root node"),
            &patches,
        )
        .expect("must convert patches")
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
        let dom_patches: Vec<DomPatch> = self.pending_patches.borrow_mut().drain(..).collect();
        let new_root_node = self.apply_dom_patches(dom_patches)?;

        //Note: it is important that root_node points to the original mutable reference here
        // since it can be replaced with a new root Node(the top-level node of the view) when patching
        // if what we are replacing is a root node:
        // we replace the root node here, so that's reference is updated
        // to the newly created node
        if let Some(new_root_node) = new_root_node {
            #[cfg(feature = "with-debug")]
            log::info!("the root_node is replaced with {:?}", &self.root_node);
            *self.root_node.borrow_mut() = Some(new_root_node);
        }
        Ok(())
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
                let program = Program::downgrade(self);
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
    #[cfg(feature = "skip_diff")]
    #[allow(unsafe_code)]
    pub fn app_clone(&self) -> ManuallyDrop<APP> {
        unsafe {
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
        #[cfg(feature = "skip_diff")]
        let old_app = self.app_clone();

        self.dispatch_pending_msgs(deadline)
            .expect("must dispatch msgs");
        // ensure that all pending msgs are all dispatched already
        #[cfg(feature = "ensure-check")]
        if self.app_context.has_pending_msgs() {
            log::info!(
                "There are still: {} pending msgs",
                self.app_context.pending_msgs_count()
            );
            self.dispatch_pending_msgs(None)
                .expect("must dispatch all pending msgs");
        }
        #[cfg(feature = "ensure-check")]
        if self.app_context.has_pending_msgs() {
            panic!("Can not proceed until previous pending msgs are dispatched..");
        }
        #[cfg(feature = "skip_diff")]
        {
            let skip_diff = self.app().skip_diff(&old_app);
            log::info!("skip_diff: {skip_diff:#?}");
        }

        let cmd = self.app_context.batch_pending_cmds();

        if !self.pending_patches.borrow().is_empty() {
            log::error!(
                "BEFORE DOM updates there are still Remaining pending patches: {}",
                self.pending_patches.borrow().len()
            );
        }

        if cmd.modifier.should_update_view {
            self.update_dom(&cmd.modifier).expect("must update dom");
        }

        // Ensure all pending patches are applied before emiting the Cmd from update
        #[cfg(feature = "ensure-check")]
        if !self.pending_patches.borrow().is_empty() {
            self.apply_pending_patches()
                .expect("applying pending patches..");
        }

        #[cfg(feature = "ensure-check")]
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
            .as_mut()
            .expect("mount node")
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


impl<APP, MSG> Program<APP, MSG>
where
    MSG: 'static,
    APP: Application<MSG>,
{
    /// patch the DOM to reflect the App's view
    ///
    /// Note: This is in another function so as to allow tests to use this shared code
    #[cfg(feature = "test-fixtures")]
    pub fn update_dom_with_vdom(
        &mut self,
        new_vdom: vdom::Node<MSG>,
    ) -> Result<usize, JsValue> {
        let dom_patches = self.create_dom_patch(&new_vdom);
        let total_patches = dom_patches.len();
        self.pending_patches.borrow_mut().extend(dom_patches);

        #[cfg(feature = "with-raf")]
        self.apply_pending_patches_with_raf().expect("raf");

        #[cfg(not(feature = "with-raf"))]
        self.apply_pending_patches().expect("raf");

        self.app_context.set_current_dom(new_vdom);
        Ok(total_patches)
    }
}

