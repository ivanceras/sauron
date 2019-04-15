use crate::Callback;
use crate::{Element, Event, Node, Text, Value};
use std::convert::AsRef;

pub struct Attribute<'a> {
    name: &'a str,
    value: AttribValue,
}

pub enum AttribValue {
    Value(Value),
    Callback(Callback<Event>),
}

impl<V: Into<Value>> From<V> for AttribValue {
    fn from(v: V) -> Self {
        AttribValue::Value(v.into())
    }
}

impl From<Callback<Event>> for AttribValue {
    fn from(c: Callback<Event>) -> Self {
        AttribValue::Callback(c)
    }
}

impl<T> Node<T>
where
    T: Clone,
{
    pub fn as_element(&mut self) -> Option<&mut Element<T>> {
        match *self {
            Node::Element(ref mut element) => Some(element),
            Node::Text(_) => None,
        }
    }

    /// Append children to this element
    pub fn children<C>(mut self, children: C) -> Self
    where
        C: AsRef<[Node<T>]>,
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
        A: AsRef<[Attribute<'a>]>,
    {
        if let Some(elm) = self.as_element() {
            elm.add_attributes_ref(attributes.as_ref());
        }
        self
    }
}

impl<T> Element<T> {
    pub fn add_attributes<'a, A>(mut self, attrs: A) -> Self
    where
        A: AsRef<[Attribute<'a>]>,
    {
        self.add_attributes_ref(attrs);
        self
    }

    /// add the attribute values or events callback
    /// into this element
    pub fn add_attributes_ref<'a, A>(&mut self, attrs: A) -> &mut Self
    where
        A: AsRef<[Attribute<'a>]>,
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
        C: AsRef<[Node<T>]>,
        T: Clone,
    {
        for c in children.as_ref() {
            self.children.push(c.clone());
        }
        self
    }

    pub fn add_event_listener(mut self, event: &str, cb: Callback<Event>) -> Self {
        self.events.insert(event.to_string(), cb);
        self
    }
}

/// Create an element
///
///```
/// use sauron_vdom::builder::*;
/// fn main(){
///    let old = element(
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
pub fn element<'a, A, C, T>(tag: T, attrs: A, children: C) -> Node<T>
where
    C: AsRef<[Node<T>]>,
    A: AsRef<[Attribute<'a>]>,
    T: Clone,
{
    Node::Element(
        Element::new(tag)
            .add_children(children)
            .add_attributes(attrs),
    )
}
#[inline]
pub fn element_ns<'a, A, C, T>(tag: T, namespace: &str, attrs: A, children: C) -> Node<T>
where
    C: AsRef<[Node<T>]>,
    A: AsRef<[Attribute<'a>]>,
    T: Clone,
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
pub fn text<V, T>(v: V) -> Node<T>
where
    V: Into<String>,
{
    Node::Text(Text { text: v.into() })
}

/// Create an attribute
#[inline]
pub fn attr<V>(name: &str, v: V) -> Attribute
where
    V: Into<Value>,
{
    Attribute {
        name,
        value: v.into().into(),
    }
}

/// Attach a callback to an event
#[inline]
pub fn on<C>(name: &str, c: C) -> Attribute
where
    C: Into<Callback<Event>>,
{
    Attribute {
        name,
        value: c.into().into(),
    }
}
