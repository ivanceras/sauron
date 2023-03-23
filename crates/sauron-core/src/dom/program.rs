#![allow(unused)]
#[cfg(feature = "with-measure")]
use crate::dom::Measurements;
use crate::dom::util;
use crate::vdom::Node;
use crate::{dom::dom_updater::DomUpdater, Application, Cmd, Dispatch};
use std::{any::TypeId, cell::RefCell, collections::BTreeMap, rc::Rc};
use wasm_bindgen::{closure::Closure, JsCast};
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen::JsValue;
use once_cell::sync::OnceCell;
use std::collections::VecDeque;


pub struct AppWrapper<APP,MSG>{
    pub app: APP,
    pending_updates: VecDeque<MSG>,
}

impl<APP,MSG> AppWrapper<APP,MSG>
    where MSG: 'static,
        APP: Application<MSG> + 'static,
    {

    fn new(app: APP) -> Self{
        Self{
            app,
            pending_updates: VecDeque::new(),
        }
    }

    ///TODO: spawn the updates here
    async fn dispatch_updates(&mut self, deadline: f64) -> Cmd<APP, MSG>{
        let t1 = crate::now();
        let mut all_cmd = vec![];
        let mut i = 0;
        while let Some(pending_msg) = self.pending_updates.pop_front(){
            #[cfg(all(feature = "with-measure", feature = "with-debug"))]
            log::debug!("Executing pending msg item {}", i);
            let c = self.app.update(pending_msg).await;
            all_cmd.push(c);
            let t2 = crate::now();
            let elapsed = t2 - t1;
            if elapsed > deadline{
                log::warn!("elapsed time: {}ms", elapsed);
                log::warn!("we should be breaking at {}..", i);
                break;
            }
            i += 1;
        }
        Cmd::batch(all_cmd)
    }

    fn init(&mut self) -> Cmd<APP,MSG>{
        self.app.init()
    }

    fn style(&self) -> String {
        self.app.style()
    }

    fn view(&self) -> Node<MSG>{
        self.app.view()
    }

    fn add_pending(&mut self, msgs: impl IntoIterator<Item = MSG>) {
        self.pending_updates.extend(msgs)
    }

    #[cfg(feature = "with-measure")]
    fn measurements(&self, measurements: Measurements) -> Cmd<APP,MSG>{
        self.app.measurements(measurements)
    }
}

/// Holds the user App and the dom updater
/// This is passed into the event listener and the dispatch program
/// will be called after the event is triggered.
pub struct Program<APP, MSG>
where
    MSG: 'static,
{
    /// holds the user application
    // Note: This needs to be in Rc<RefCell<_>> to allow interior mutability
    // from a non-mutable reference
    pub app_wrap: Rc<RefCell<AppWrapper<APP, MSG>>>,
    /// The dom_updater responsible to updating the actual document in the browser
    pub dom_updater: Rc<RefCell<DomUpdater<MSG>>>,
}

impl<APP, MSG> Clone for Program<APP, MSG>
where
    MSG: 'static,
{
    fn clone(&self) -> Self {
        Program {
            app_wrap: Rc::clone(&self.app_wrap),
            dom_updater: Rc::clone(&self.dom_updater),
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
        let dom_updater: DomUpdater<MSG> =
            DomUpdater::new(app.view(), mount_node, replace, use_shadow);
        Program {
            app_wrap: Rc::new(RefCell::new(AppWrapper::new(app))),
            dom_updater: Rc::new(RefCell::new(dom_updater)),
        }
    }

    /// executed after the program has been mounted
    fn after_mounted(&self) {
        // call the init of the component
        let cmds = self.app_wrap.borrow_mut().init();
        // then emit the cmds, so it starts executing initial calls such (ie: fetching data,
        // listening to events (resize, hashchange)
        cmds.emit(self);

        // inject the style style after call the init of the app as
        // it may be modifying the app state including the style
        let style = self.app_wrap.borrow().style();
        if !style.trim().is_empty() {
            let type_id = TypeId::of::<APP>();
            Self::inject_style(type_id, &style);
        }
    }

    /// return the node where the app is mounted into
    pub fn mount_node(&self) -> web_sys::Node {
        self.dom_updater.borrow().mount_node()
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

    /// Do the actual mounting of the view to the specified mount node
    pub fn mount(&self) {
        self.dom_updater.borrow_mut().mount(self);
        self.after_mounted();
    }

    /// explicity update the dom
    pub fn update_dom(&self) {
        let view = self.app_wrap.borrow().view();
        // update the last DOM node tree with this new view
        let mut dom_updater = self.dom_updater.borrow_mut();
        let _total_patches =
            dom_updater
            .update_dom(self, view);
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

    async fn dispatch_updates(&self, deadline: f64) -> Cmd<APP, MSG>{
        self.app_wrap.borrow_mut().dispatch_updates(deadline).await
    }

    async fn dispatch_dom_changes(&self, _log_measurements: bool, _measurement_name: &str, _msg_count: usize, _t1: f64) {
        #[cfg(feature = "with-measure")]
        let t2 = crate::now();

        // a new view is created due to the app update
        let view = self.app_wrap.borrow().view();
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
            self.dom_updater.borrow_mut().update_dom(self, view).await.expect("must not error");
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
                self.app_wrap.borrow().measurements(measurements).no_render();
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
        let cmd = self.dispatch_updates(deadline).await;

        if cmd.modifier.should_update_view {
            let log_measurements = cmd.modifier.log_measurements;
            let measurement_name = &cmd.modifier.measurement_name;
            self.dispatch_dom_changes(log_measurements, measurement_name, msg_count, t1).await;
        }
        cmd.emit(self);
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

    /// Inject a style to the mount node
    pub fn inject_style_to_mount(&self, style: &str) {
        self.dom_updater.borrow().inject_style_to_mount(self, style);
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
        self.app_wrap.borrow_mut().add_pending(msgs);
        let program = self.clone();
        #[cfg(feature = "with-ric")]
        let handle = util::request_idle_callback(move|v: JsValue|{
            let deadline = if let Ok(deadline) = v.dyn_into::<web_sys::IdleDeadline>() {
                deadline.time_remaining()
            } else {
                panic!("must have a deadline");
            };
            log::info!("deadline: {:.2}", deadline);
            let program = program.clone();
            spawn_local(async move{
                program.dispatch_inner(deadline).await;
            });
        });

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
