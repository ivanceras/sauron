#[cfg(feature = "with-measure")]
use crate::dom::Measurements;
use crate::Cmd;
use crate::{dom::dom_updater::DomUpdater, Application, Dispatch};
use std::any::TypeId;
use std::collections::BTreeMap;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::Node;

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
    pub app: Rc<RefCell<APP>>,
    /// The dom_updater responsible to updating the actual document in the browser
    pub dom_updater: Rc<RefCell<DomUpdater<MSG>>>,
}

impl<APP, MSG> Clone for Program<APP, MSG>
where
    MSG: 'static,
{
    fn clone(&self) -> Self {
        Program {
            app: Rc::clone(&self.app),
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
        mount_node: &Node,
        replace: bool,
        use_shadow: bool,
    ) -> Self {
        let dom_updater: DomUpdater<MSG> =
            DomUpdater::new(app.view(), mount_node, replace, use_shadow);
        Program {
            app: Rc::new(RefCell::new(app)),
            dom_updater: Rc::new(RefCell::new(dom_updater)),
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
    pub fn replace_mount(app: APP, mount_node: &Node) -> Self {
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
    pub fn append_to_mount(app: APP, mount_node: &Node) -> Self {
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
        let view = self.app.borrow().view();
        // update the last DOM node tree with this new view
        let _total_patches =
            self.dom_updater.borrow_mut().update_dom(self, view);
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
    fn dispatch_inner(&self, msgs: Vec<MSG>) {
        #[cfg(feature = "with-measure")]
        let t1 = crate::now();
        #[cfg(feature = "with-measure")]
        let msg_count = msgs.len();
        // update the app and emit the cmd returned from the update
        let all_cmd = msgs
            .into_iter()
            .map(|msg| self.app.borrow_mut().update(msg));
        let cmd = Cmd::batch(all_cmd);

        if cmd.modifier.should_update_view {
            #[cfg(feature = "with-measure")]
            let t2 = crate::now();

            // a new view is created due to the app update
            let view = self.app.borrow().view();

            #[cfg(feature = "with-measure")]
            let node_count = view.node_count();
            #[cfg(feature = "with-measure")]
            let t3 = crate::now();

            // update the last DOM node tree with this new view
            let _total_patches =
                self.dom_updater.borrow_mut().update_dom(self, view);
            #[cfg(feature = "with-measure")]
            let t4 = crate::now();

            #[cfg(feature = "with-measure")]
            {
                let dispatch_duration = t4 - t1;
                // 60fps is 16.667 ms per frame.
                if dispatch_duration > 16.0 {
                    log::warn!("dispatch took: {}ms", dispatch_duration);
                }
            }

            #[cfg(feature = "with-measure")]
            if cmd.modifier.log_measurements && _total_patches > 0 {
                let measurements = Measurements {
                    name: cmd.modifier.measurement_name.clone(),
                    msg_count,
                    view_node_count: node_count,
                    update_dispatch_took: t2 - t1,
                    build_view_took: t3 - t2,
                    total_patches: _total_patches,
                    dom_update_took: t4 - t3,
                    total_time: t4 - t1,
                };
                // tell the app on app performance measurements
                let cmd_measurement =
                    self.app.borrow().measurements(measurements).no_render();
                cmd_measurement.emit(self);
            }
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
    fn dispatch_multiple(&self, msgs: Vec<MSG>) {
        let program_clone = self.clone();
        let closure_raf: Closure<dyn FnMut() + 'static> =
            Closure::once(move || {
                program_clone.dispatch_inner(msgs);
            });
        crate::dom::util::request_animation_frame_for_closure(closure_raf);
    }

    fn dispatch(&self, msg: MSG) {
        self.dispatch_multiple(vec![msg])
    }

    fn dispatch_with_delay(&self, msg: MSG, timeout: i32) -> Option<i32> {
        let program_clone = self.clone();
        let window = crate::window();
        let closure_delay: Closure<dyn FnMut() + 'static> =
            Closure::once(move || {
                program_clone.dispatch(msg);
            });

        let timeout_id = window
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                closure_delay.as_ref().unchecked_ref(),
                timeout,
            )
            .expect("should register the setTimeout call");

        closure_delay.forget();
        Some(timeout_id)
    }
}
