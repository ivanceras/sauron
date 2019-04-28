use crate::{Callback,
            Element,
            Node,
            Text,
            Value};
use std::convert::AsRef;

pub struct Attribute<EVENT, MSG>
    where MSG: Clone
{
    name: &'static str,
    value: AttribValue<EVENT, MSG>,
}

pub enum AttribValue<EVENT, MSG>
    where MSG: Clone
{
    Value(Value),
    Callback(Callback<EVENT, MSG>),
}

impl<EVENT, MSG> From<Callback<EVENT, MSG>> for AttribValue<EVENT, MSG>
    where MSG: Clone
{
    fn from(cb: Callback<EVENT, MSG>) -> Self {
        AttribValue::Callback(cb)
    }
}

impl<T, EVENT, MSG> Node<T, EVENT, MSG>
    where T: Clone,
          MSG: Clone,
          EVENT: Clone
{
    pub fn as_element(&mut self) -> Option<&mut Element<T, EVENT, MSG>> {
        match *self {
            Node::Element(ref mut element) => Some(element),
            Node::Text(_) => None,
        }
    }

    pub fn as_element_ref(&mut self) -> Option<&Element<T, EVENT, MSG>> {
        match *self {
            Node::Element(ref element) => Some(element),
            Node::Text(_) => None,
        }
    }

    /// Append children to this element
    pub fn children<C>(mut self, children: C) -> Self
        where C: AsRef<[Node<T, EVENT, MSG>]>
    {
        if let Some(element) = self.as_element() {
            for child in children.as_ref() {
                element.children.push(child.clone());
            }
        }
        self
    }

    /// add attributes to the node
    pub fn attributes<A>(mut self, attributes: A) -> Self
        where A: AsRef<[Attribute<EVENT, MSG>]>
    {
        if let Some(elm) = self.as_element() {
            elm.add_attributes_ref(attributes.as_ref());
        }
        self
    }
}

impl<T, EVENT, MSG> Element<T, EVENT, MSG>
    where T: Clone,
          MSG: Clone,
          EVENT: Clone
{
    pub fn add_attributes<A>(mut self, attrs: A) -> Self
        where A: AsRef<[Attribute<EVENT, MSG>]>
    {
        self.add_attributes_ref(attrs);
        self
    }

    /// add the attribute values or events callback
    /// into this element
    pub fn add_attributes_ref<A>(&mut self, attrs: A) -> &mut Self
        where A: AsRef<[Attribute<EVENT, MSG>]>
    {
        for a in attrs.as_ref() {
            match a.value {
                AttribValue::Value(ref v) => {
                    if let Some(existing) = self.attrs.get_mut(a.name) {
                        existing.append(v.clone());
                    } else {
                        self.attrs.insert(a.name, v.clone());
                    }
                }
                AttribValue::Callback(ref v) => {
                    self.events.insert(a.name, v.clone());
                }
            }
        }
        self
    }

    pub fn add_children<C>(mut self, children: C) -> Self
        where C: AsRef<[Node<T, EVENT, MSG>]>
    {
        for c in children.as_ref() {
            self.children.push(c.clone());
        }
        self
    }

    pub fn add_event_listener(mut self,
                              event: &'static str,
                              cb: Callback<EVENT, MSG>)
                              -> Self {
        self.events.insert(event, cb);
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
///    let old:Node<&'static str, (), ()> = element(
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
/// ```
#[inline]
pub fn element<A, C, T, EVENT, MSG>(tag: T,
                                    attrs: A,
                                    children: C)
                                    -> Node<T, EVENT, MSG>
    where C: AsRef<[Node<T, EVENT, MSG>]>,
          A: AsRef<[Attribute<EVENT, MSG>]>,
          T: Clone,
          MSG: Clone,
          EVENT: Clone
{
    Node::Element(Element::new(tag).add_children(children)
                                   .add_attributes(attrs))
}
#[inline]
pub fn element_ns<A, C, T, EVENT, MSG>(tag: T,
                                       namespace: &'static str,
                                       attrs: A,
                                       children: C)
                                       -> Node<T, EVENT, MSG>
    where C: AsRef<[Node<T, EVENT, MSG>]>,
          A: AsRef<[Attribute<EVENT, MSG>]>,
          T: Clone,
          MSG: Clone,
          EVENT: Clone
{
    Node::Element(Element::new(tag).namespace(namespace)
                                   .add_children(children)
                                   .add_attributes(attrs))
}

/// Create a textnode element
#[inline]
pub fn text<V, T, EVENT, MSG>(v: V) -> Node<T, EVENT, MSG>
    where V: ToString,
          EVENT: Clone,
          MSG: Clone
{
    Node::Text(Text { text: v.to_string() })
}

/// Create an attribute
#[inline]
pub fn attr<V, EVENT, MSG>(name: &'static str, v: V) -> Attribute<EVENT, MSG>
    where V: Into<Value>,
          EVENT: Clone,
          MSG: Clone
{
    Attribute { name,
                value: AttribValue::Value(v.into()) }
}

/// Creates a callback object from the function closure
/// This will then be attached to the browser and emitted
/// when that event is triggered.
///
/// FIXME: callbacks are recrated eveytime, therefore they are not
/// equivalent when compared since function contents
/// can not be compared. Only Rc's are compared.
#[inline]
pub fn on<C, EVENT, MSG>(name: &'static str, c: C) -> Attribute<EVENT, MSG>
    where C: Into<Callback<EVENT, MSG>>,
          EVENT: Clone,
          MSG: Clone
{
    Attribute { name,
                value: AttribValue::Callback(c.into()) }
}

/// the func will be used to convert the native browser event and the result will
/// be fed into the Callback input
pub fn on_with_mapper<C, F, OUT, EVENT, MSG>(name: &'static str,
                                             func: F,
                                             c: C)
                                             -> Attribute<EVENT, MSG>
    where C: Into<Callback<OUT, MSG>>,
          F: Fn(EVENT) -> OUT + 'static,
          EVENT: Clone,
          MSG: Clone + 'static,
          OUT: 'static
{
    let cb: Callback<OUT, MSG> = c.into();
    let cb2: Callback<EVENT, MSG> = cb.reform(func);
    Attribute { name,
                value: AttribValue::Callback(cb2) }
}
