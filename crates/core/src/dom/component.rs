use crate::html::attributes::{class, classes, Attribute};
use crate::vdom::AttributeName;
use crate::{dom::Effects, vdom::Node};
use std::any::TypeId;
use std::rc::Rc;
use crate::vdom::Leaf;

pub use stateful_component::{register_template, stateful_component, StatefulComponent, StatefulModel};
#[cfg(feature = "custom_element")]
pub use web_component::{register_web_component, WebComponent, WebComponentWrapper};

mod stateful_component;
#[cfg(feature = "custom_element")]
mod web_component;

/// A component has a view and can update itself.
///
/// The update function returns an effect which can contain
/// follow ups and effects. Follow ups are executed on the next
/// update loop of this component, while the effects are executed
/// on the parent component that mounts it.
pub trait Component<MSG, XMSG>
where
    MSG: 'static,
{
    /// init the component
    fn init(&mut self) -> Effects<MSG, XMSG> {
        Effects::none()
    }

    /// Update the model of this component and return
    /// follow up and/or effects that will be executed on the next update loop
    fn update(&mut self, msg: MSG) -> Effects<MSG, XMSG>;

    /// the view of the component
    fn view(&self) -> Node<MSG>;

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
    fn class_ns(class_name: &str) -> Attribute<MSG>
    where
        Self: Sized,
    {
        class(Self::prefix_class(class_name))
    }

    /// create namespaced class names to pair that evaluates to true
    fn classes_ns_flag(pair: impl IntoIterator<Item = (impl ToString, bool)>) -> Attribute<MSG>
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

pub struct StatelessModel<MSG>{
    /// the view of this stateless model
    pub view: Box<Node<MSG>>,
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
            type_id: self.type_id.clone(),
            attrs: self.attrs.clone(),
            children: self.children.clone(),
        }
    }
}

/// create a stateless component node
pub fn component<COMP, MSG, MSG2>(
    app: COMP,
    attrs: impl IntoIterator<Item = Attribute<MSG>>,
    children: impl IntoIterator<Item = Node<MSG>>,
) -> Node<MSG>
where
    COMP: Component<MSG, MSG2> + 'static,
    MSG: 'static,
{
    let type_id = TypeId::of::<COMP>();
    let view = app.view();
    let (_template, vdom_template) = register_template(type_id, &view);
    Node::Leaf(Leaf::StatelessComponent(StatelessModel {
        view: Box::new(view),
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

        impl Component<Msg, ()> for AwesomeEditor {
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
