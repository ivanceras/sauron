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
    #[inline]
    pub fn as_element(&mut self) -> Option<&mut Element<T, EVENT, MSG>> {
        match *self {
            Node::Element(ref mut element) => Some(element),
            Node::Text(_) => None,
        }
    }

    #[inline]
    pub fn as_element_ref(&mut self) -> Option<&Element<T, EVENT, MSG>> {
        match *self {
            Node::Element(ref element) => Some(element),
            Node::Text(_) => None,
        }
    }

    /// Append children to this element
    #[inline]
    pub fn children<C>(mut self, children: C) -> Self
        where C: AsRef<[Node<T, EVENT, MSG>]>
    {
        if let Some(element) = self.as_element() {
            element.add_children_ref(children);
        }
        self
    }

    /// add attributes to the node
    #[inline]
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
    #[inline]
    pub fn add_attributes<A>(mut self, attrs: A) -> Self
        where A: AsRef<[Attribute<EVENT, MSG>]>
    {
        self.add_attributes_ref(attrs);
        self
    }

    /// add the attribute values or events callback
    /// into this element
    #[inline]
    fn add_attributes_ref<A>(&mut self, attrs: A) -> &mut Self
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

    #[inline]
    pub fn add_children<C>(mut self, children: C) -> Self
        where C: AsRef<[Node<T, EVENT, MSG>]>
    {
        self.add_children_ref(children);
        self
    }

    #[inline]
    fn add_children_ref<C>(&mut self, children: C) -> &mut Self
        where C: AsRef<[Node<T, EVENT, MSG>]>
    {
        for c in children.as_ref() {
            self.children.push(c.clone());
        }
        self
    }

    #[inline]
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
    Node::Element(Element::with_children(tag, children).add_attributes(attrs))
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
    Node::Element(Element::with_children_and_maybe_ns(tag, children, Some(namespace))
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

/// Create an callback event which has a function
/// to map web_sys::Event to user event
#[inline]
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
