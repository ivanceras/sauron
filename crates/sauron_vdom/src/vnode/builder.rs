use crate::{Callback,
            Element,
            Node,
            Text,
            Value};
use std::convert::AsRef;

#[derive(Clone)]
pub struct Attribute<EVENT, MSG>
    where MSG: Clone
{
    name: &'static str,
    value: AttribValue<EVENT, MSG>,
}

#[derive(Clone)]
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
    pub fn children(mut self, children: Vec<Node<T, EVENT, MSG>>) -> Self {
        if let Some(element) = self.as_element() {
            element.add_children(children);
        }
        self
    }

    /// add attributes to the node
    pub fn attributes(mut self,
                      attributes: Vec<Attribute<EVENT, MSG>>)
                      -> Self {
        if let Some(elm) = self.as_element() {
            elm.add_attributes(attributes);
        }
        self
    }
}

impl<T, EVENT, MSG> Element<T, EVENT, MSG>
    where T: Clone,
          MSG: Clone,
          EVENT: Clone
{
    #[inline]
    pub fn add_attributes(&mut self, attrs: Vec<Attribute<EVENT, MSG>>) {
        for a in attrs {
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
    }

    #[inline]
    pub fn add_children(&mut self, children: Vec<Node<T, EVENT, MSG>>) {
        self.children.extend(children);
    }

    #[inline]
    pub fn add_event_listener(&mut self,
                              event: &'static str,
                              cb: Callback<EVENT, MSG>) {
        self.events.insert(event, cb);
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
    element_ns(tag, None, attrs, children)
}
pub fn element_ns<A, C, T, EVENT, MSG>(tag: T,
                                       namespace: Option<&'static str>,
                                       attrs: A,
                                       children: C)
                                       -> Node<T, EVENT, MSG>
    where C: AsRef<[Node<T, EVENT, MSG>]>,
          A: AsRef<[Attribute<EVENT, MSG>]>,
          T: Clone,
          MSG: Clone,
          EVENT: Clone
{
    let mut element =
        Element::with_children_and_maybe_ns(tag, children, namespace);
    element.add_attributes(attrs.as_ref().to_vec());
    Node::Element(element)
}

/// Create a textnode element
pub fn text<V, T, EVENT, MSG>(v: V) -> Node<T, EVENT, MSG>
    where V: ToString,
          EVENT: Clone,
          MSG: Clone
{
    Node::Text(Text { text: v.to_string() })
}

/// Create an attribute
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
pub fn on<C, EVENT, MSG>(name: &'static str, c: C) -> Attribute<EVENT, MSG>
    where C: Into<Callback<EVENT, MSG>>,
          EVENT: Clone,
          MSG: Clone
{
    Attribute { name,
                value: AttribValue::Callback(c.into()) }
}

/// Create an callback event which has a function
/// to map web_sys::Event to user event
pub fn on_with_extractor<EVENT, WEV2UDEF, UDEF, UDEF2MSG, MSG>(
    name: &'static str,
    webevent_to_user_def: WEV2UDEF,
    user_def_to_msg: UDEF2MSG)
    -> Attribute<EVENT, MSG>
    where WEV2UDEF: Fn(EVENT) -> UDEF + 'static,
          UDEF2MSG: Fn(UDEF) -> MSG + 'static,
          MSG: Clone + 'static,
          UDEF: 'static,
          EVENT: Clone + 'static
{
    let cb: Callback<EVENT, UDEF> = Callback::from(webevent_to_user_def);
    let cb2: Callback<EVENT, MSG> = cb.map(user_def_to_msg);
    Attribute { name,
                value: AttribValue::Callback(cb2) }
}
