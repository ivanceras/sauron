use crate::dom::DomAttr;
use crate::dom::DomAttrValue;
use crate::html::attributes::{class, classes, Attribute};
use crate::vdom::AttributeName;
use crate::vdom::AttributeValue;
use crate::vdom::Leaf;
use crate::{dom::Effects, vdom::Node};
use std::any::Any;
use std::any::TypeId;
use std::cell::Cell;
use std::collections::BTreeMap;
use std::rc::Rc;

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

/// A Container have children that is set from the parent component
///
/// It can update its Mode and returns follow ups and/or effects on the next
/// update loop.
///
/// The view in the container is set by the parent component. The container itself
/// can not listen to events on its view
pub trait Container<MSG, XMSG>
where
    MSG: 'static,
{
    /// init the container
    fn init(&mut self) -> Effects<MSG, XMSG> {
        Effects::none()
    }
    /// update the model of this component and return follow ups and/or effects
    /// that will be executed on the next update loop.
    fn update(&mut self, msg: MSG) -> Effects<MSG, XMSG>;

    /// The container presents the children passed to it from the parent.
    /// The container can decide how to display the children components here, but
    /// the children nodes here can not trigger Msg that can update this component
    fn view(&self, content: impl IntoIterator<Item = Node<XMSG>>) -> Node<MSG>;

    /// optionally a Container can specify its own css style
    fn stylesheet() -> Vec<String> {
        vec![]
    }

    /// dynamic style
    fn style(&self) -> Vec<String> {
        vec![]
    }

    /// containers can append children
    fn append_child(&mut self, child: Node<XMSG>);

    /// return the component name
    /// defaults to the struct simplified name
    fn component_name() -> String {
        extract_simple_struct_name::<Self>()
    }

    /// prefix the class bane
    fn prefix_class(class_name: &str) -> String {
        let component_name = Self::component_name();
        if class_name.is_empty() {
            component_name
        } else {
            format!("{component_name}__{class_name}")
        }
    }

    /// create a classname prepended with this component name
    fn class_ns(class_name: &str) -> Attribute<MSG> {
        class(Self::prefix_class(class_name))
    }

    /// create namespaced class names to pair that evaluates to true
    fn classes_ns_flag(pair: impl IntoIterator<Item = (impl ToString, bool)>) -> Attribute<MSG> {
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
    fn selector_ns(class_name: &str) -> String {
        let component_name = Self::component_name();
        if class_name.is_empty() {
            format!(".{component_name}")
        } else {
            format!(".{component_name}__{class_name}")
        }
    }

    /// create namesspaced selector from multiple classnames
    fn selectors_ns(class_names: impl IntoIterator<Item = impl ToString>) -> String {
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

/// A component that can be used directly in the view without mapping
pub trait StatefulComponent {
    /// create the stateful component with this attributes
    fn build(
        atts: impl IntoIterator<Item = DomAttr>,
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
        attr_name: AttributeName,
        old_value: DomAttrValue,
        new_value: DomAttrValue,
    ) where
        Self: Sized;

    fn template(&self) -> web_sys::Node;

    /// remove the attribute with this name
    fn remove_attribute(&mut self, attr_name: AttributeName);

    /// append a child into this component
    fn append_child(&mut self, child: web_sys::Node);

    /// remove a child in this index
    fn remove_child(&mut self, index: usize);

    /// the component is attached to the dom
    fn connected_callback(&mut self);
    /// the component is removed from the DOM
    fn disconnected_callback(&mut self);

    /// the component is moved or attached to the dom
    fn adopted_callback(&mut self);
}

thread_local!(static COMPONENT_ID_COUNTER: Cell<usize> = Cell::new(1));

pub fn create_component_unique_identifier() -> usize {
    COMPONENT_ID_COUNTER.with(|x| {
        let val = x.get();
        x.set(val + 1);
        val
    })
}

/// create a stateful component node
pub fn component<COMP, MSG>(
    attrs: impl IntoIterator<Item = Attribute<MSG>>,
    children: impl IntoIterator<Item = Node<MSG>>,
) -> Node<MSG>
where
    COMP: StatefulComponent + 'static,
{
    // make a global registry here
    // store the COMP in the global registry
    // and when the program encounter the component with the type id
    // it will be retrieved from the global registry
    let type_id = TypeId::of::<COMP>();
    log::info!(
        "type_id: {type_id:?}, type_name: {}",
        std::any::type_name::<COMP>()
    );
    let comp = COMP::build([], []);
    Node::Leaf(Leaf::Component {
        comp: Rc::new(comp),
        type_id,
        attrs: attrs.into_iter().collect(),
        children: children.into_iter().collect(),
    })
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
