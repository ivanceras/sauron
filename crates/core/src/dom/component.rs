use crate::dom::SkipDiff;
use crate::html::attributes::{class, classes, Attribute};
use crate::vdom::AttributeName;
use crate::vdom::Leaf;
use crate::{dom::Effects, vdom::Node};
use std::any::TypeId;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

use crate::dom::template;
pub use stateful_component::{stateful_component, StatefulComponent, StatefulModel};
#[cfg(feature = "custom_element")]
pub use web_component::{register_web_component, WebComponent, WebComponentWrapper};

mod stateful_component;
#[cfg(feature = "custom_element")]
mod web_component;

thread_local! {
    static TEMPLATE_LOOKUP: RefCell<HashMap<TypeId, web_sys::Node>> = RefCell::new(HashMap::new());
}

/// if the template is already registered, return the dom template
/// if not, create the dom template and add it
pub fn register_template<MSG>(type_id: TypeId, vdom_template: &Node<MSG>) -> web_sys::Node {
    if let Some(template) = lookup_template(type_id) {
        template
    } else {
        let template = template::create_dom_node_without_listeners(&vdom_template);
        add_template(type_id, &template);
        template
    }
}

pub fn add_template(type_id: TypeId, template: &web_sys::Node) {
    TEMPLATE_LOOKUP.with_borrow_mut(|map| {
        if map.contains_key(&type_id) {
            // already added
        } else {
            map.insert(type_id, template.clone());
        }
    })
}

/// lookup for the template
pub fn lookup_template(type_id: TypeId) -> Option<web_sys::Node> {
    TEMPLATE_LOOKUP.with_borrow(|map| {
        if let Some(existing) = map.get(&type_id) {
            Some(existing.clone_node_with_deep(true).expect("deep clone"))
        } else {
            None
        }
    })
}

/// A component has a view and can update itself.
///
/// The update function returns an effect which can contain
/// follow ups and effects. Follow ups are executed on the next
/// update loop of this component, while the effects are executed
/// on the parent component that mounts it.
pub trait Component {
    ///
    type MSG: 'static;
    ///
    type XMSG: 'static;

    /// init the component
    fn init(&mut self) -> Effects<Self::MSG, Self::XMSG> {
        Effects::none()
    }

    /// Update the model of this component and return
    /// follow up and/or effects that will be executed on the next update loop
    fn update(&mut self, msg: Self::MSG) -> Effects<Self::MSG, Self::XMSG>;

    /// the view of the component
    fn view(&self) -> Node<Self::MSG>;

    /// optional logical code when to skip diffing some particular node
    /// by comparing field values of app and its old values
    fn skip_diff(&self) -> Option<SkipDiff> {
        None
    }

    ///
    fn template(&self) -> Option<Node<Self::MSG>> {
        None
    }

    /// component can have static styles
    fn stylesheet() -> Vec<String>
    where
        Self: Sized,
    {
        vec![]
    }

    /// specify which attribute names are observed for this Component
    fn observed_attributes() -> Vec<AttributeName> {
        vec![]
    }

    /// in addition, component can contain dynamic style
    /// which can change when the model is updated
    fn style(&self) -> Vec<String> {
        vec![]
    }

    /// return the component name
    /// defaults to the struct simplified name
    fn component_name() -> String
    where
        Self: Sized,
    {
        extract_simple_struct_name::<Self>()
    }

    /// prefix the class bane
    fn prefix_class(class_name: &str) -> String
    where
        Self: Sized,
    {
        let component_name = Self::component_name();
        if class_name.is_empty() {
            component_name
        } else {
            format!("{component_name}__{class_name}")
        }
    }

    /// create a classname prepended with this component name
    fn class_ns(class_name: &str) -> Attribute<Self::MSG>
    where
        Self: Sized,
    {
        class(Self::prefix_class(class_name))
    }

    /// create namespaced class names to pair that evaluates to true
    fn classes_ns_flag(
        pair: impl IntoIterator<Item = (impl ToString, bool)>,
    ) -> Attribute<Self::MSG>
    where
        Self: Sized,
    {
        let class_list = pair.into_iter().filter_map(|(class, flag)| {
            if flag {
                Some(Self::prefix_class(&class.to_string()))
            } else {
                None
            }
        });

        classes(class_list)
    }

    /// create a selector class prepended with this component name
    fn selector_ns(class_name: &str) -> String
    where
        Self: Sized,
    {
        let component_name = Self::component_name();
        if class_name.is_empty() {
            format!(".{component_name}")
        } else {
            format!(".{component_name}__{class_name}")
        }
    }

    /// create namesspaced selector from multiple classnames
    fn selectors_ns(class_names: impl IntoIterator<Item = impl ToString>) -> String
    where
        Self: Sized,
    {
        let selectors: Vec<String> = class_names
            .into_iter()
            .map(|class_name| Self::selector_ns(&class_name.to_string()))
            .collect();
        selectors.join(" ")
    }
}

pub(crate) fn extract_simple_struct_name<T: ?Sized>() -> String {
    let type_name = std::any::type_name::<T>();
    let name = if let Some(first) = type_name.split(['<', '>']).next() {
        first
    } else {
        type_name
    };
    name.rsplit("::")
        .next()
        .map(|s| s.to_string())
        .expect("must have a name")
}

/// Contains necessary information for creating template
/// of the Component of this type_id
pub struct StatelessModel<MSG> {
    /// the view of this stateless model
    pub view: Box<Node<MSG>>,
    /// skip_diff
    pub skip_diff: Rc<Option<SkipDiff>>,
    /// the vdom template of this component
    pub vdom_template: Box<Node<MSG>>,
    /// component type id
    pub type_id: TypeId,
    /// attributes of this model
    pub attrs: Vec<Attribute<MSG>>,
    /// external children component
    pub children: Vec<Node<MSG>>,
}

impl<MSG> StatelessModel<MSG> {
    /// mape the msg of this Leaf such that `Leaf<MSG>` becomes `Leaf<MSG2>`
    pub fn map_msg<F, MSG2>(self, cb: F) -> StatelessModel<MSG2>
    where
        F: Fn(MSG) -> MSG2 + Clone + 'static,
        MSG2: 'static,
        MSG: 'static,
    {
        StatelessModel {
            type_id: self.type_id,
            view: Box::new(self.view.map_msg(cb.clone())),
            skip_diff: self.skip_diff.clone(),
            vdom_template: Box::new(self.vdom_template.map_msg(cb.clone())),
            attrs: self
                .attrs
                .into_iter()
                .map(|a| a.map_msg(cb.clone()))
                .collect(),
            children: self
                .children
                .into_iter()
                .map(|c| c.map_msg(cb.clone()))
                .collect(),
        }
    }
}

impl<MSG> Clone for StatelessModel<MSG> {
    fn clone(&self) -> Self {
        Self {
            view: self.view.clone(),
            skip_diff: self.skip_diff.clone(),
            vdom_template: self.vdom_template.clone(),
            type_id: self.type_id.clone(),
            attrs: self.attrs.clone(),
            children: self.children.clone(),
        }
    }
}

impl<MSG> PartialEq for StatelessModel<MSG>{
    fn eq(&self, other: &Self) -> bool {
            self.view == other.view
            && self.skip_diff == other.skip_diff
            && self.vdom_template == other.vdom_template
            && self.type_id == other.type_id
            && self.attrs == other.attrs
            && self.children == other.children
    }
}

/// create a stateless component node
pub fn component<COMP>(
    app: &COMP,
    attrs: impl IntoIterator<Item = Attribute<COMP::MSG>>,
    children: impl IntoIterator<Item = Node<COMP::MSG>>,
) -> Node<COMP::MSG>
where
    COMP: Component + 'static,
{
    let type_id = TypeId::of::<COMP>();
    let view = app.view();
    let skip_diff = app.skip_diff();

    let vdom_template = app.template().expect("must have a template");

    let _template = register_template(type_id, &vdom_template);
    Node::Leaf(Leaf::StatelessComponent(StatelessModel {
        view: Box::new(view),
        skip_diff: Rc::new(skip_diff),
        vdom_template: Box::new(vdom_template),
        type_id,
        attrs: attrs.into_iter().collect(),
        children: children.into_iter().collect(),
    }))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::html::*;
    use std::marker::PhantomData;

    #[test]
    fn test_extract_component_name() {
        enum Msg {}
        struct AwesomeEditor {}

        impl Component for AwesomeEditor {
            type MSG = Msg;
            type XMSG = ();

            fn update(&mut self, _msg: Msg) -> Effects<Msg, ()> {
                Effects::none()
            }
            fn view(&self) -> Node<Msg> {
                div([], [])
            }
        }

        let name = extract_simple_struct_name::<AwesomeEditor>();
        println!("name: {name}");
        assert_eq!("AwesomeEditor", name);
    }

    #[test]
    fn test_name_with_generics() {
        struct ComplexEditor<XMSG> {
            _phantom2: PhantomData<XMSG>,
        }

        enum Xmsg {}

        let name = extract_simple_struct_name::<ComplexEditor<Xmsg>>();
        println!("name: {name}");
        assert_eq!("ComplexEditor", name);
    }

    #[test]
    fn test_name_with_2_generics() {
        struct ComplexEditor<MSG, XMSG> {
            _phantom1: PhantomData<MSG>,
            _phantom2: PhantomData<XMSG>,
        }

        enum Msg {}
        enum Xmsg {}

        let name = extract_simple_struct_name::<ComplexEditor<Msg, Xmsg>>();
        println!("name: {name}");
        assert_eq!("ComplexEditor", name);
    }
}
