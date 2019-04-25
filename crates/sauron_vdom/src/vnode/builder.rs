use crate::Callback;
use crate::Event;
use crate::{Element, Node, Text, Value};
use std::convert::AsRef;

pub struct Attribute<'a, MSG>
where
    MSG: Clone,
{
    name: &'a str,
    value: AttribValue<MSG>,
}

pub enum AttribValue<MSG>
where
    MSG: Clone,
{
    Value(Value),
    Callback(Callback<Event, MSG>),
}

impl<MSG> From<Callback<Event, MSG>> for AttribValue<MSG>
where
    MSG: Clone,
{
    fn from(cb: Callback<Event, MSG>) -> Self {
        AttribValue::Callback(cb)
    }
}

impl<T, MSG> Node<T, MSG>
where
    T: Clone,
    MSG: Clone,
{
    pub fn as_element(&mut self) -> Option<&mut Element<T, MSG>> {
        match *self {
            Node::Element(ref mut element) => Some(element),
            Node::Text(_) => None,
        }
    }

    pub fn as_element_ref(&mut self) -> Option<&Element<T, MSG>> {
        match *self {
            Node::Element(ref element) => Some(element),
            Node::Text(_) => None,
        }
    }

    /// Append children to this element
    pub fn children<C>(mut self, children: C) -> Self
    where
        C: AsRef<[Node<T, MSG>]>,
        MSG: Clone,
    {
        if let Some(element) = self.as_element() {
            for child in children.as_ref() {
                element.children.push(child.clone());
            }
        }
        self
    }

    /// add attributes to the node
    pub fn attributes<'a, A>(mut self, attributes: A) -> Self
    where
        A: AsRef<[Attribute<'a, MSG>]>,
    {
        if let Some(elm) = self.as_element() {
            elm.add_attributes_ref(attributes.as_ref());
        }
        self
    }
}

impl<T, MSG> Element<T, MSG> {
    pub fn add_attributes<'a, A>(mut self, attrs: A) -> Self
    where
        A: AsRef<[Attribute<'a, MSG>]>,
        MSG: Clone,
    {
        self.add_attributes_ref(attrs);
        self
    }

    /// add the attribute values or events callback
    /// into this element
    pub fn add_attributes_ref<'a, A>(&mut self, attrs: A) -> &mut Self
    where
        A: AsRef<[Attribute<'a, MSG>]>,
        MSG: Clone,
    {
        for a in attrs.as_ref() {
            match a.value {
                AttribValue::Value(ref v) => {
                    self.attrs.insert(a.name.to_string(), v.clone());
                }
                AttribValue::Callback(ref v) => {
                    self.events.insert(a.name.to_string(), v.clone());
                }
            }
        }
        self
    }

    pub fn add_children<C>(mut self, children: C) -> Self
    where
        C: AsRef<[Node<T, MSG>]>,
        T: Clone,
        MSG: Clone,
    {
        for c in children.as_ref() {
            self.children.push(c.clone());
        }
        self
    }

    pub fn add_event_listener(mut self, event: &str, cb: Callback<Event, MSG>) -> Self {
        self.events.insert(event.to_string(), cb);
        self
    }
}

/// Create an element
///
///```
/// use sauron_vdom::builder::*;
/// use sauron_vdom::Node;
/// use sauron_vdom::Callback;
/// use sauron_vdom::Event;
/// fn main(){
///    let old:Node<&'static str, ()> = element(
///        "div",
///        [
///            attr("class", "some-class"),
///            attr("id", "some-id"),
///            on("click", |_| {
///                println!("clicked");
///            }),
///            attr("data-id", 1111),
///            on("mouseover", |_| {
///                println!("i've been clicked");
///            }),
///        ],
///        [element("div", [], [text("Hello world!")])],
///    );
/// }
///```
#[inline]
pub fn element<'a, A, C, T, MSG>(tag: T, attrs: A, children: C) -> Node<T, MSG>
where
    C: AsRef<[Node<T, MSG>]>,
    A: AsRef<[Attribute<'a, MSG>]>,
    T: Clone,
    MSG: Clone,
{
    Node::Element(
        Element::new(tag)
            .add_children(children)
            .add_attributes(attrs),
    )
}
#[inline]
pub fn element_ns<'a, A, C, T, MSG>(tag: T, namespace: &str, attrs: A, children: C) -> Node<T, MSG>
where
    C: AsRef<[Node<T, MSG>]>,
    A: AsRef<[Attribute<'a, MSG>]>,
    T: Clone,
    MSG: Clone,
{
    Node::Element(
        Element::new(tag)
            .namespace(namespace)
            .add_children(children)
            .add_attributes(attrs),
    )
}

/// Create a textnode element
#[inline]
pub fn text<V, T, MSG>(v: V) -> Node<T, MSG>
where
    V: ToString,
    MSG: Clone,
{
    Node::Text(Text {
        text: v.to_string(),
    })
}

/// Create an attribute
#[inline]
pub fn attr<V, MSG>(name: &str, v: V) -> Attribute<MSG>
where
    V: Into<Value>,
    MSG: Clone,
{
    Attribute {
        name,
        value: AttribValue::Value(v.into()),
    }
}

/// Creates a callback object from the function closure
/// This will then be attached to the browser and emitted
/// when that event is triggered.
///
/// FIXME: callbacks are recrated eveytime, therefore they are not
/// equivalent when compared since function contents
/// can not be compared. Only Rc's are compared.
#[inline]
pub fn on<C, MSG>(name: &str, c: C) -> Attribute<MSG>
where
    C: Into<Callback<Event, MSG>>,
    MSG: Clone,
{
    Attribute {
        name,
        value: AttribValue::Callback(c.into()),
    }
}
