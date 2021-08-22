use crate::{dom::dom_updater::DomUpdater, Application, Dispatch};
use std::any::TypeId;
use std::{cell::RefCell, rc::Rc};
#[cfg(feature = "with-request-animation-frame")]
use wasm_bindgen::closure::Closure;
use web_sys::Node;

use crate::dom::Measurements;

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
    pub fn new(app: APP, root_node: &Node) -> Self {
        let dom_updater: DomUpdater<MSG> =
            DomUpdater::new(app.view(), root_node);
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

    /// get the real DOM node where this app is mounted to.
    #[allow(unused)]
    fn root_node(&self) -> web_sys::Node {
        self.dom_updater.borrow().root_node()
    }

    /// Creates an Rc wrapped instance of Program and replace the root_node with the app view
    pub fn replace_mount(app: APP, root_node: &Node) -> Self {
        let program = Self::new(app, root_node);
        program.start_replace_mount();
        program.after_mounted();
        program
    }

    ///  Instantiage an app and append the view to the root_node
    pub fn append_to_mount(app: APP, root_node: &Node) -> Self {
        let program = Self::new(app, root_node);
        program.start_append_to_mount();
        program.after_mounted();
        program
    }

    /// Instantiate the app and then append it to the document body
    pub fn mount_to_body(app: APP) -> Self {
        Self::append_to_mount(app, &crate::body())
    }

    /// Replace the body of the document with the app
    pub fn replace_body(app: APP) -> Self {
        Self::replace_mount(app, &crate::body())
    }
    fn start_append_to_mount(&self) {
        self.dom_updater.borrow_mut().append_to_mount(self)
    }

    fn start_replace_mount(&self) {
        self.dom_updater.borrow_mut().replace_mount(self)
    }

    /// This is called when an event is triggered in the html DOM.
    /// The sequence of things happening here:
    /// - The app component update is executed.
    /// - The returned Cmd from the component update is then emitted.
    /// - The view is reconstructed with the new state of the app.
    /// - The dom is updated with the newly reconstructed view.
    fn dispatch_inner(&self, msg: MSG) {
        let t1 = crate::now();
        // update the app and emit the cmd returned from the update
        let cmd = self.app.borrow_mut().update(msg);

        if cmd.should_update_view {
            //trace!("Executing cmd..");
            let t2 = crate::now();

            #[cfg(feature = "with-measure")]
            log::trace!("app update took: {}ms", t2 - t1);

            // a new view is created due to the app update
            let view = self.app.borrow().view();
            let node_count = view.node_count();
            let t3 = crate::now();

            #[cfg(feature = "with-measure")]
            log::trace!("creating app view took: {}ms", t3 - t2);

            // update the last DOM node tree with this new view
            self.dom_updater.borrow_mut().update_dom(self, view);
            let t4 = crate::now();
            #[cfg(feature = "with-measure")]
            log::trace!("dom update took: {}ms", t4 - t3);

            #[cfg(feature = "with-measure")]
            {
                let dispatch_duration = t4 - t1;
                // 60fps is 16.667 ms per frame.
                if dispatch_duration > 16.0 {
                    log::warn!("dispatch took: {}ms", dispatch_duration);
                } else {
                    log::trace!("dispatch took: {}ms", dispatch_duration);
                }
            }

            if cmd.log_measurements {
                let measurements = Measurements {
                    view_node_count: node_count,
                    update_dispatch_took: t2 - t1,
                    build_view_took: t3 - t2,
                    dom_update_took: t4 - t3,
                    total_time: t4 - t1,
                };
                // tell the app on app performance measurements
                let mut cmd_measurement =
                    self.app.borrow().measurements(measurements);
                cmd_measurement.should_update_view = false;
                cmd_measurement.emit(self);
            } else {
                #[cfg(any(feature = "with-debug", feature = "with-measure"))]
                log::info!("skipped logging measurements");
            }
        } else {
            #[cfg(any(feature = "with-debug", feature = "with-measure"))]
            log::info!("dom update is skipped here");
        }
        cmd.emit(self);
    }

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
}

/// This will be called when the actual event is triggered.
/// Defined in the DomUpdater::create_closure_wrap function
impl<APP, MSG> Dispatch<MSG> for Program<APP, MSG>
where
    MSG: 'static,
    APP: Application<MSG> + 'static,
{
    #[cfg(feature = "with-request-animation-frame")]
    fn dispatch(&self, msg: MSG) {
        let program_clone = self.clone();
        let closure_raf: Closure<dyn FnMut() + 'static> =
            Closure::once(move || {
                program_clone.dispatch_inner(msg);
            });
        crate::dom::util::request_animation_frame_for_closure(&closure_raf);
        closure_raf.forget();
    }

    #[cfg(not(feature = "with-request-animation-frame"))]
    fn dispatch(&self, msg: MSG) {
        self.dispatch_inner(msg)
    }
}
