use crate::dom::program::app_context::WeakContext;
#[cfg(feature = "with-raf")]
use crate::dom::request_animation_frame;
#[cfg(feature = "with-ric")]
use crate::dom::request_idle_callback;
use crate::dom::DomNode;
use crate::dom::SkipDiff;
use crate::dom::SkipPath;
use crate::dom::{document, now, IdleDeadline, Measurements, Modifier};
use crate::dom::{util::body, AnimationFrameHandle, Application, DomPatch, IdleCallbackHandle};
use crate::html::{self, attributes::class, text};
use crate::vdom;
use crate::vdom::diff;
use crate::vdom::diff_recursive;
use crate::vdom::Patch;
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
use wasm_bindgen::{JsCast, JsValue};
use web_sys;

pub(crate) use app_context::AppContext;
pub use mount_procedure::{MountAction, MountProcedure, MountTarget};

mod app_context;
mod mount_procedure;

/// Program handle the lifecycle of the APP
pub struct Program<APP>
where
    APP: Application,
{
    pub(crate) app_context: AppContext<APP>,

    /// the first element of the app view, where the patch is generated is relative to
    pub root_node: Rc<RefCell<Option<DomNode>>>,

    /// the actual DOM element where the APP is mounted to.
    pub(crate) mount_node: Rc<RefCell<Option<DomNode>>>,

    /// Pending patches that hasn't been applied to the DOM yet
    /// for optimization purposes to avoid sluggishness of the app, when a patch
    /// can not be run in 1 execution due to limited remaining time deadline
    /// it will be put into the pending patches to be executed on the next run.
    pub(crate) pending_patches: Rc<RefCell<VecDeque<DomPatch>>>,

    /// store the Closure used in request_idle_callback calls
    pub(crate) idle_callback_handles: Rc<RefCell<Vec<IdleCallbackHandle>>>,
    /// store the Closure used in request_animation_frame calls
    pub(crate) animation_frame_handles: Rc<RefCell<Vec<AnimationFrameHandle>>>,

    /// keep track of the time when the dom is last updated
    pub(crate) last_update: Rc<RefCell<Option<f64>>>,
}

pub struct WeakProgram<APP>
where
    APP: Application,
{
    pub(crate) app_context: WeakContext<APP>,
    pub(crate) root_node: Weak<RefCell<Option<DomNode>>>,
    mount_node: Weak<RefCell<Option<DomNode>>>,
    pending_patches: Weak<RefCell<VecDeque<DomPatch>>>,
    idle_callback_handles: Weak<RefCell<Vec<IdleCallbackHandle>>>,
    animation_frame_handles: Weak<RefCell<Vec<AnimationFrameHandle>>>,
    last_update: Weak<RefCell<Option<f64>>>,
}

impl<APP> WeakProgram<APP>
where
    APP: Application,
{
    ///
    pub fn upgrade(&self) -> Option<Program<APP>> {
        let app_context = self.app_context.upgrade()?;
        let root_node = self.root_node.upgrade()?;
        let mount_node = self.mount_node.upgrade()?;
        let pending_patches = self.pending_patches.upgrade()?;
        let idle_callback_handles = self.idle_callback_handles.upgrade()?;
        let animation_frame_handles = self.animation_frame_handles.upgrade()?;
        let last_update = self.last_update.upgrade()?;
        Some(Program {
            app_context,
            root_node,
            mount_node,
            pending_patches,
            idle_callback_handles,
            animation_frame_handles,
            last_update,
        })
    }
}

impl<APP> Clone for WeakProgram<APP>
where
    APP: Application,
{
    fn clone(&self) -> Self {
        WeakProgram {
            app_context: self.app_context.clone(),
            root_node: Weak::clone(&self.root_node),
            mount_node: Weak::clone(&self.mount_node),
            pending_patches: Weak::clone(&self.pending_patches),
            idle_callback_handles: Weak::clone(&self.idle_callback_handles),
            animation_frame_handles: Weak::clone(&self.animation_frame_handles),
            last_update: Weak::clone(&self.last_update),
        }
    }
}

impl<APP> Program<APP>
where
    APP: Application,
{
    ///
    pub fn downgrade(&self) -> WeakProgram<APP> {
        WeakProgram {
            app_context: AppContext::downgrade(&self.app_context),
            root_node: Rc::downgrade(&self.root_node),
            mount_node: Rc::downgrade(&self.mount_node),
            pending_patches: Rc::downgrade(&self.pending_patches),
            idle_callback_handles: Rc::downgrade(&self.idle_callback_handles),
            animation_frame_handles: Rc::downgrade(&self.animation_frame_handles),
            last_update: Rc::downgrade(&self.last_update),
        }
    }
}

impl<APP> Clone for Program<APP>
where
    APP: Application,
{
    fn clone(&self) -> Self {
        Program {
            app_context: self.app_context.clone(),
            root_node: Rc::clone(&self.root_node),
            mount_node: Rc::clone(&self.mount_node),
            pending_patches: Rc::clone(&self.pending_patches),
            idle_callback_handles: Rc::clone(&self.idle_callback_handles),
            animation_frame_handles: Rc::clone(&self.animation_frame_handles),
            last_update: Rc::clone(&self.last_update),
        }
    }
}

impl<APP> Program<APP>
where
    APP: Application,
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

impl<APP> Program<APP>
where
    APP: Application,
{
    /// Create an Rc wrapped instance of program, initializing DomUpdater with the initial view
    /// and root node, but doesn't mount it yet.
    pub fn new(app: APP) -> Self {
        Self::from_rc_app(Rc::new(RefCell::new(app)))
    }

    /// create a program from Rc<RefCell<APP>>
    pub fn from_rc_app(app: Rc<RefCell<APP>>) -> Self {
        let app_view = app.borrow().view();
        log::info!("app_view: {}", app_view.render_to_string());
        Program {
            app_context: AppContext {
                app,
                current_vdom: Rc::new(RefCell::new(app_view)),
                pending_msgs: Rc::new(RefCell::new(VecDeque::new())),
                pending_cmds: Rc::new(RefCell::new(VecDeque::new())),
            },
            root_node: Rc::new(RefCell::new(None)),
            mount_node: Rc::new(RefCell::new(None)),
            pending_patches: Rc::new(RefCell::new(VecDeque::new())),
            idle_callback_handles: Rc::new(RefCell::new(vec![])),
            animation_frame_handles: Rc::new(RefCell::new(vec![])),
            last_update: Rc::new(RefCell::new(None)),
        }
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

    #[allow(unused)]
    /// create initial dom node generated
    /// from template and patched by the difference of vdom_template and current app view.
    fn create_initial_view(&self) -> DomNode {
        let current_view = self.app_context.current_vdom();
        let real_view = current_view.unwrap_template_ref();
        self.create_dom_node(None, &real_view)
    }

    /// each element and it's descendant in the vdom is created into
    /// an actual DOM node.
    pub fn mount(&mut self, mount_node: &web_sys::Node, mount_procedure: MountProcedure) {
        let mount_node = DomNode::from(mount_node.clone());
        *self.mount_node.borrow_mut() = Some(mount_node);
        self.pre_mount();

        #[cfg(feature = "use-template")]
        let created_node = self.create_initial_view_with_template();
        #[cfg(not(feature = "use-template"))]
        let created_node = self.create_initial_view();

        let mount_node: DomNode = match mount_procedure.target {
            MountTarget::MountNode => self
                .mount_node
                .borrow()
                .as_ref()
                .expect("mount node")
                .clone(),
            MountTarget::ShadowRoot => {
                /*
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
                */
                todo!("shadow onhold!..")
            }
        };

        match mount_procedure.action {
            MountAction::Append => {
                mount_node.append_children([created_node.clone()]);
            }
            MountAction::ClearAppend => {
                log::info!("doing a clear append..");
                mount_node.clear_children();
                mount_node.append_children([created_node.clone()]);
            }
            MountAction::Replace => {
                mount_node.replace_node(created_node.clone());
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
        let skip_diff = view.skip_diff();

        let dom_patches = if let Some(skip_diff) = skip_diff {
            let current_vdom = self.app_context.current_vdom();
            let real_current_vdom = current_vdom.unwrap_template_ref();
            let real_view = view.unwrap_template_ref();
            let patches =
                self.create_patches_with_skip_diff(&real_current_vdom, &real_view, &skip_diff);
            //log::info!("patches: {:#?}",patches);
            self.convert_patches(
                self.root_node
                    .borrow()
                    .as_ref()
                    .expect("must have a root node"),
                &patches,
            )
            .expect("must convert patches")
        } else {
            self.create_dom_patch(&view)
        };

        let total_patches = dom_patches.len();

        // update the last DOM node tree with this new view
        self.queue_dom_patches(dom_patches).expect("must not error");
        // set the current dom
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
                // TODO: maybe return early here, but do a dispatch_multiple([])
            }
        }

        // tell the app about the performance measurement and only if there was patches applied
        #[cfg(feature = "with-measure")]
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

    pub(crate) fn create_patches_with_skip_diff<'a>(
        &self,
        old_vdom: &'a vdom::Node<APP::MSG>,
        new_vdom: &'a vdom::Node<APP::MSG>,
        skip_diff: &SkipDiff,
    ) -> Vec<Patch<'a, APP::MSG>> {
        use crate::vdom::TreePath;
        assert!(!old_vdom.is_template(), "old vdom should not be a template");
        assert!(!new_vdom.is_template(), "new vdom should not be a template");
        diff_recursive(
            &old_vdom,
            &new_vdom,
            &SkipPath::new(TreePath::root(), skip_diff.clone()),
        )
    }

    fn create_dom_patch(&self, new_vdom: &vdom::Node<APP::MSG>) -> Vec<DomPatch> {
        let current_vdom = self.app_context.current_vdom();
        log::info!("current_vdom: {}", current_vdom.render_to_string());
        log::info!("    new_vdom: {}", new_vdom.render_to_string());
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
            log::info!("Setting the new root node..");
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

    /// This is called when an event is triggered in the html DOM.
    /// The sequence of things happening here:
    /// - The app component update is executed.
    /// - The returned Cmd from the component update is then emitted.
    /// - The view is reconstructed with the new state of the app.
    /// - The dom is updated with the newly reconstructed view.
    fn dispatch_inner(&mut self, deadline: Option<IdleDeadline>) {
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
        let created_node = self.create_dom_node(None, &style_node);

        let head = document().head().expect("must have a head");
        let head_node: web_sys::Node = head.unchecked_into();
        let dom_head = DomNode::from(head_node);
        dom_head.append_children([created_node]);
    }

    /// inject style element to the mount node
    pub fn inject_style_to_mount(&mut self, style: &str) {
        let style_node = html::tags::style([], [text(style)]);
        let created_node = self.create_dom_node(None, &style_node);

        self.mount_node
            .borrow_mut()
            .as_mut()
            .expect("mount node")
            .append_children([created_node]);
    }

    /// dispatch multiple MSG
    pub fn dispatch_multiple(&mut self, msgs: impl IntoIterator<Item = APP::MSG>) {
        self.app_context.push_msgs(msgs);
        self.dispatch_inner_with_priority_ric();
    }

    /// dispatch a single msg
    pub fn dispatch(&mut self, msg: APP::MSG) {
        self.dispatch_multiple([msg])
    }
}

impl<APP> Program<APP>
where
    APP: Application,
{
    /// patch the DOM to reflect the App's view
    ///
    /// Note: This is in another function so as to allow tests to use this shared code
    #[cfg(feature = "test-fixtures")]
    pub fn update_dom_with_vdom(
        &mut self,
        new_vdom: vdom::Node<APP::MSG>,
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
