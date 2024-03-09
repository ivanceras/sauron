use crate::dom::dom_node::DomNode;
use crate::dom::events::on_mount;
use crate::dom::program::ActiveClosure;
use crate::dom::program::AppContext;
use crate::dom::program::MountProcedure;
use crate::dom::template;
use crate::dom::Application;
use crate::dom::Cmd;
use crate::dom::Component;
use crate::dom::DomAttr;
use crate::dom::DomAttrValue;
use crate::dom::Program;
use crate::vdom;
use crate::vdom::Attribute;
use crate::vdom::AttributeName;
use crate::vdom::Leaf;
use crate::vdom::LeafComponent;
use crate::vdom::Node;
use std::any::TypeId;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::rc::Rc;
use wasm_bindgen::JsValue;

thread_local! {
    static TEMPLATE_LOOKUP: RefCell<HashMap<TypeId, web_sys::Node>> = RefCell::new(HashMap::new());
}

pub fn register_template<APP, MSG>(app: &APP) -> (web_sys::Node, vdom::Node<MSG>)
where
    APP: Application<MSG>,
    MSG: 'static,
{
    let type_id = TypeId::of::<APP>();
    let view = app.view();
    let vdom_template = template::build_vdom_template(&view);
    let template = TEMPLATE_LOOKUP.with_borrow_mut(|map| {
        if let Some(existing) = map.get(&type_id) {
            log::info!("An existing template...");
            existing.clone_node_with_deep(true).expect("deep clone")
        } else {
            log::warn!("Adding a new template for: {:?}", type_id);
            let template = template::build_template(&view);
            log::warn!("{}", template.render_to_string());
            map.insert(type_id, template.clone());
            template
        }
    });
    if cfg!(feature = "use-template") {
        (template, vdom_template)
    } else {
        (template, view)
    }
}

/// A component that can be used directly in the view without mapping
pub trait StatefulComponent {
    /// create the stateful component with this attributes
    fn build(
        attrs: impl IntoIterator<Item = DomAttr>,
        children: impl IntoIterator<Item = web_sys::Node>,
    ) -> Self
    where
        Self: Sized;

    /// This will be invoked when a component is used as a custom element
    /// and the attributes of the custom-element has been modified
    ///
    /// if the listed attributes in the observed attributes are modified
    fn attribute_changed(
        &mut self,
        attr_name: &str,
        old_value: DomAttrValue,
        new_value: DomAttrValue,
    ) where
        Self: Sized;

    /// build the template of this Component
    fn template(&self) -> web_sys::Node;

    /// remove the attribute with this name
    fn remove_attribute(&mut self, _attr_name: AttributeName) {}

    /// append a child into this component
    fn append_child(&mut self, _child: &web_sys::Node) {}

    /// remove a child in this index
    fn remove_child(&mut self, _index: usize) {}

    /// the component is attached to the dom
    fn connected_callback(&mut self) {}
    /// the component is removed from the DOM
    fn disconnected_callback(&mut self) {}

    /// the component is moved or attached to the dom
    fn adopted_callback(&mut self) {}
}

impl<COMP, MSG> Application<MSG> for COMP
where
    COMP: Component<MSG, ()> + StatefulComponent + 'static,
    MSG: 'static,
{
    fn init(&mut self) -> Cmd<Self, MSG> {
        Cmd::from(<Self as Component<MSG, ()>>::init(self))
    }

    fn update(&mut self, msg: MSG) -> Cmd<Self, MSG> {
        let effects = <Self as Component<MSG, ()>>::update(self, msg);
        Cmd::from(effects)
    }

    fn view(&self) -> Node<MSG> {
        <Self as Component<MSG, ()>>::view(self)
    }

    fn stylesheet() -> Vec<String> {
        <Self as Component<MSG, ()>>::stylesheet()
    }

    fn style(&self) -> Vec<String> {
        <Self as Component<MSG, ()>>::style(self)
    }
}

/// create a stateful component node
pub fn component<COMP, MSG, MSG2>(
    app: COMP,
    attrs: impl IntoIterator<Item = Attribute<MSG>>,
    children: impl IntoIterator<Item = Node<MSG>>,
) -> Node<MSG>
where
    COMP: Component<MSG2, ()> + StatefulComponent + 'static,
    MSG: Default + 'static,
    MSG2: 'static,
{
    let type_id = TypeId::of::<COMP>();
    let attrs = attrs.into_iter().collect::<Vec<_>>();

    // Note: we can not include the children in the build function
    // as the children here contains the MSG generic
    // and we can not discard the event listeners.
    //
    // The attribute(minus events) however can be used for configurations, for setting initial state
    // of the stateful component.

    /*
    let app = COMP::build(
        attrs
            .clone()
            .into_iter()
            .map(|a| DomAttr::convert_attr_except_listener(&a)),
        [],
    );
    */

    let (template, vdom_template) = register_template(&app);

    log::info!("vdom template: {}", vdom_template.render_to_string());

    log::info!("dom  template: {}", template.render_to_string());

    let app = Rc::new(RefCell::new(app));

    let program = Program {
        app_context: AppContext {
            app: Rc::clone(&app),
            #[cfg(feature = "use-template")]
            template,
            current_vdom: Rc::new(RefCell::new(vdom_template)),
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
    let children: Vec<Node<MSG>> = children.into_iter().collect();
    let mount_event = on_mount(move |me| {
        log::info!("Component is now mounted..");
        let mut program = program.clone();
        program.mount(&me.target_node, MountProcedure::append());
        MSG::default()
    });
    let node = Node::Leaf(Leaf::Component(LeafComponent {
        comp: app,
        type_id,
        attrs: attrs.into_iter().chain([mount_event]).collect(),
        children: children.into_iter().collect(),
    }));
    node
}

impl Into<DomAttrValue> for JsValue {
    fn into(self) -> DomAttrValue {
        if let Some(v) = self.as_bool() {
            DomAttrValue::Simple(v.into())
        } else if let Some(v) = self.as_f64() {
            DomAttrValue::Simple(v.into())
        } else if let Some(v) = self.as_string() {
            DomAttrValue::Simple(v.into())
        } else if self.is_null() {
            log::info!("it is a null value");
            DomAttrValue::Empty
        } else {
            todo!("handle other conversion, other than bool, f64, strings, nulls ")
        }
    }
}
