use crate::dom::Measurements;
use crate::vdom;
use crate::{Application, Cmd, Dispatch, prelude::Patch};
use std::{any::TypeId, cell::RefCell, collections::BTreeMap, rc::Rc};
use wasm_bindgen::JsCast;
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

    /// The program should be able to set and keep track of the focused element.
    /// There is only 1 focused element at any given time.
    ///
    /// after mounting or update dispatch call, the element will be focused
    pub focused_node: Rc<RefCell<Option<Node>>>,

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
        let program = Self::new(app, mount_node, false);
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

       if self.use_shadow {
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


    #[cfg(feature = "with-ric")]
    fn dispatch_pending_msgs_with_ric(&self) -> Result<(), JsValue>{
        let program = self.clone();
        crate::dom::util::request_idle_callback_with_deadline(move|deadline|{
            let program = program.clone();
            spawn_local(async move{
                program.dispatch_pending_msgs(Some(deadline)).await.expect("must execute")
            });
        }).expect("must execute");
        Ok(())
    }

    /// executes pending msgs by calling the app update method with the msgs
    /// as parameters.
    /// If there is no deadline specified all the pending messages are executed
    async fn dispatch_pending_msgs(&self, deadline: Option<f64>) ->Result<(), JsValue>{
        if self.pending_msgs.borrow().is_empty(){
            return Ok(())
        }
        let mut i = 0;
        let t1 = crate::now();
        let mut did_complete = true;
        while let Some(pending_msg) = self.pending_msgs.borrow_mut().pop_front(){
            // Note: each MSG needs to be executed one by one in the same order
            // as APP's state can be affected by the previous MSG
            let cmd = self.app.borrow_mut().update(pending_msg).await;

            // we put the cmd in the pending_cmd queue
            self.pending_cmds.borrow_mut().push_back(cmd);

            let t2 = crate::now();
            let elapsed = t2 - t1;
            // break only if a deadline is supplied
            if let Some(deadline) = deadline{
                if elapsed > deadline{
                    log::warn!("elapsed time: {}ms", elapsed);
                    log::warn!("we should be breaking at {}..", i);
                    did_complete = false;
                    break;
                }
            }
            i += 1;
        }
        if !did_complete{
            #[cfg(feature = "with-ric")]
            self.dispatch_pending_msgs_with_ric().expect("must complete");
        }
        Ok(())
    }

    /// update the browser DOM to reflect the APP's  view
    pub fn update_dom(&self) -> Result<Measurements, JsValue>{
        let t1 = crate::now();
        // a new view is created due to the app update
        let view = self.app.borrow().view();
        let t2 = crate::now();

        let node_count = view.node_count();

        // update the last DOM node tree with this new view
        let total_patches =
            self.update_dom_with_vdom(view).expect("must not error");
        let t3 = crate::now();

        let measurements = Measurements{
            name: None,
            node_count,
            build_view_took: t2 - t1,
            total_patches,
            dom_update_took: t3 - t2,
            total_time: t3 - t1,
        };
        Ok(measurements)
    }

    /// patch the DOM to reflect the App's view
    pub fn update_dom_with_vdom(
        &self,
        new_vdom: vdom::Node<MSG>,
    ) -> Result<usize, JsValue>
    {

        let total_patches = {
            let current_vdom = self.current_vdom.borrow();
            let patches = diff(&current_vdom, &new_vdom);
            let dom_patches = self.convert_patches(&patches).expect("must convert patches");
            self.pending_patches.borrow_mut().extend(dom_patches);
            self.apply_pending_patches_with_priority_raf();
            patches.len()
        };

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

    /// get the real DOM target node and make a DomPatch object for each of the Patch
    fn convert_patches(&self, patches: &[Patch<MSG>]) -> Result<Vec<DomPatch<MSG>>, JsValue>{
        let nodes_to_find: Vec<(&TreePath, Option<&&'static str>)> = patches
            .iter()
            .map(|patch| (patch.path(), patch.tag()))
            .collect();

        let nodes_to_patch = created_node::find_all_nodes(self.root_node.borrow().as_ref().expect("must have a root node"), &nodes_to_find);

        let dom_patches:Vec<DomPatch<MSG>> = patches.iter().map(|patch|{
            let patch_path = patch.path();
            let patch_tag = patch.tag();
            if let Some(target_node) = nodes_to_patch.get(patch_path) {
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
                DomPatch::from_patch(self, target_node, &mut self.focused_node.borrow_mut(), patch)
            } else {
                unreachable!("Getting here means we didn't find the element of next node that we are supposed to patch, patch_path: {:?}, with tag: {:?}", patch_path, patch_tag);
            }
        }).collect();

        Ok(dom_patches)

    }


    /// apply pending patches using raf
    /// if raf is not available, use ric
    /// if ric is not available call bare function
    fn apply_pending_patches_with_priority_raf(&self){
        #[cfg(feature = "with-raf")]
        self.apply_pending_patches_with_raf().expect("must complete");
        #[cfg(not(feature = "with-raf"))]
        {
            #[cfg(feature = "with-ric")]
            self.apply_pending_patches_with_ric().expect("must complete");
            #[cfg(not(feature = "with-ric"))]
            self.apply_pending_patches(None).expect("must complete");
        }
    }


    /// apply pending patches using ric
    /// if ric is not available, use raf
    /// if raf is not available call bare function
    fn apply_pending_patches_with_priority_ric(&self){
        #[cfg(feature = "with-ric")]
        self.apply_pending_patches_with_ric().expect("must complete");
        #[cfg(not(feature = "with-ric"))]
        {
            #[cfg(feature = "with-raf")]
            self.apply_pending_patches_with_raf().expect("must complete");
            #[cfg(not(feature = "with-raf"))]
            self.apply_pending_patches(None).expect("must complete");
        }
    }


    #[cfg(feature = "with-ric")]
    fn apply_pending_patches_with_ric(&self) -> Result<(), JsValue>{
        let program = self.clone();
        crate::dom::util::request_idle_callback_with_deadline(move|deadline|{
            program.apply_pending_patches(Some(deadline))
                .expect("must not error");
        }).expect("must complete the remaining pending patches..");
        Ok(())
    }

    #[cfg(feature = "with-raf")]
    #[allow(unused)]
    fn apply_pending_patches_with_raf(&self) -> Result<(), JsValue>{
        let program = self.clone();
        crate::dom::util::request_animation_frame(move||{
            let deadline = 10.0;
            program.apply_pending_patches(Some(deadline))
                .expect("must not error");
        }).expect("must execute");
        Ok(())
    }

    /// apply the pending patches into the DOM
    fn apply_pending_patches(
        &self,
        deadline: Option<f64>
    ) -> Result<(), JsValue>
    {
        if self.pending_patches.borrow().is_empty(){
            return Ok(())
        }
        let t1 = crate::now();
        let mut did_complete = true;
        while let Some(dom_patch) = self.pending_patches.borrow_mut().pop_front() {
            let t2 = crate::now();
            self.apply_dom_patch(dom_patch).expect("must apply dom patch");
            let elapsed = t2 - t1;
            // only break if deadline is specified
            if let Some(deadline) = deadline{
                if elapsed > deadline {
                    did_complete = false;
                    break;
                }
            }
        }
        if !did_complete{
            self.apply_pending_patches_with_priority_ric();
        }
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
    #[allow(unused)]
    fn dispatch_dom_changes(&self, log_measurements: bool) {

        let measurements = self.update_dom().expect("must update dom");


        #[cfg(feature = "with-measure")]
        // tell the app about the performance measurement and only if there was patches applied
        if log_measurements && measurements.total_patches > 0 {
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
    async fn dispatch_inner(&self, deadline: f64) {

        self.dispatch_pending_msgs(Some(deadline)).await.expect("must dispatch msgs");
        // ensure that all pending msgs are all dispatched already
        if !self.pending_msgs.borrow().is_empty(){
            self.dispatch_pending_msgs(None).await.expect("must dispatch all pending msgs");
        }
        if !self.pending_msgs.borrow().is_empty(){
            panic!("Can not proceed until previous pending msgs are dispatched..");
        }


        let mut all_cmd = vec![];
        let mut pending_cmds = self.pending_cmds.borrow_mut();
        while let Some(cmd) = pending_cmds.pop_front(){
            all_cmd.push(cmd);
        }
        // we can execute all the cmd here at once
        let cmd = Cmd::batch(all_cmd);

        if !self.pending_patches.borrow().is_empty(){
            log::error!("BEFORE DOM updates there are still Remaining pending patches: {}", self.pending_patches.borrow().len());
        }

        if cmd.modifier.should_update_view {
            let log_measurements = cmd.modifier.log_measurements;
            self.dispatch_dom_changes(log_measurements);
        }

        // Ensure all pending patches are applied before emiting the Cmd from update
        if !self.pending_patches.borrow().is_empty(){
            self.apply_pending_patches(None).expect("applying pending patches..");
        }

        if !self.pending_patches.borrow().is_empty(){
            log::error!("Remaining pending patches: {}", self.pending_patches.borrow().len());
            panic!("There are still pending patches.. can not emit cmd, if all pending patches
            has not been applied yet!");
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

    /// inject style element to the mount node
    pub fn inject_style_to_mount(&self, style: &str)
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
    /// dispatch multiple MSG
    fn dispatch_multiple(&self, msgs: Vec<MSG>) {
        self.pending_msgs.borrow_mut().extend(msgs);
        let program = self.clone();
        #[cfg(feature = "with-ric")]
        let _handle = crate::dom::util::request_idle_callback_with_deadline(move|deadline: f64|{
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

}
