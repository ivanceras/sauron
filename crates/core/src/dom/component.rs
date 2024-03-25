use crate::html::attributes::{class, classes, Attribute};
use crate::vdom::AttributeName;
use crate::vdom::Leaf;
use crate::{dom::Effects, vdom::Node};
use derive_where::derive_where;
use std::any::TypeId;
use std::cell::RefCell;
use std::collections::HashMap;
use crate::dom::DomNode;

pub use stateful_component::{stateful_component, StatefulComponent, StatefulModel};
#[cfg(feature = "custom_element")]
pub use web_component::{register_web_component, WebComponent, WebComponentWrapper};

mod no_listener;
mod stateful_component;
#[cfg(feature = "custom_element")]
mod web_component;

thread_local! {
    static TEMPLATE_LOOKUP: RefCell<HashMap<TypeId, DomNode>> = RefCell::new(HashMap::new());
}

/// if the template is already registered, return the dom template
/// if not, create the dom template and add it
pub fn register_template<MSG>(type_id: TypeId, vdom_template: &Node<MSG>) -> DomNode {
    if let Some(template) = lookup_template(type_id) {
        template
    } else {
        let template = no_listener::create_dom_node_no_listeners(None, &vdom_template);
        add_template(type_id, &template);
        template
    }
}

pub fn add_template(type_id: TypeId, template: &DomNode) {
    TEMPLATE_LOOKUP.with_borrow_mut(|map| {
        if map.contains_key(&type_id) {
            // already added
        } else {
            map.insert(
                type_id,
                //template.clone_node_with_deep(true).expect("deep clone"),
                template.deep_clone().expect("deep clone"),
            );
        }
    })
}

/// lookup for the template
pub fn lookup_template(type_id: TypeId) -> Option<DomNode> {
    TEMPLATE_LOOKUP.with_borrow(|map| {
        if let Some(existing) = map.get(&type_id) {
            //Some(existing.clone_node_with_deep(true).expect("deep clone"))
            Some(existing.deep_clone().expect("deep clone"))
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
        let class_names:Vec<&str> = class_name.split(" ").collect();
        let prefixed_classes = class_names.iter().map(|c|Self::prefix_class(c)).collect::<Vec<_>>().join(" ");
        class(prefixed_classes)
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
#[derive_where(Debug)]
pub struct StatelessModel<MSG> {
    /// the view of this stateless model
    pub view: Box<Node<MSG>>,
    /// component type id
    pub type_id: TypeId,
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
        }
    }
}

impl<MSG> Clone for StatelessModel<MSG> {
    fn clone(&self) -> Self {
        Self {
            view: self.view.clone(),
            type_id: self.type_id.clone(),
        }
    }
}

impl<MSG> PartialEq for StatelessModel<MSG> {
    fn eq(&self, other: &Self) -> bool {
        self.view == other.view
            && self.type_id == other.type_id
    }
}

/// create a stateless component node
pub fn component<COMP>(
    app: &COMP,
) -> Node<COMP::MSG>
where
    COMP: Component + 'static,
{
    let type_id = TypeId::of::<COMP>();
    let app_view = app.view();
    Node::Leaf(Leaf::StatelessComponent(StatelessModel {
        view: Box::new(app_view),
        type_id,
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
        assert_eq!("AwesomeEditor", name);
    }

    #[test]
    fn test_name_with_generics() {
        struct ComplexEditor<XMSG> {
            _phantom2: PhantomData<XMSG>,
        }

        enum Xmsg {}

        let name = extract_simple_struct_name::<ComplexEditor<Xmsg>>();
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
        assert_eq!("ComplexEditor", name);
    }
}
