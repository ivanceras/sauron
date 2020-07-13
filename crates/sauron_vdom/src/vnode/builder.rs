//! provides utility function for building virtual nodes and attributes
use crate::{
    vnode::Attribute,
    Callback,
    Element,
    Node,
    Text,
    Value,
};

/// Create an element
///
///```
/// use sauron_vdom::{
///     builder::*,
///     Callback,
///     Event,
///     Node,
/// };
/// fn main() {
///     let old: Node<&'static str, &'static str, (), ()> = element(
///         "div",
///         vec![
///             attr("class", "some-class"),
///             attr("id", "some-id"),
///             on("click", |_| {
///                 println!("clicked");
///             }),
///             attr("data-id", 1111),
///             on("mouseover", |_| {
///                 println!("i've been clicked");
///             }),
///         ],
///         vec![element("div", vec![], vec![text("Hello world!")])],
///     );
/// }
/// ```
#[inline]
pub fn element<T, ATT, EVENT, MSG>(
    tag: T,
    attrs: Vec<Attribute<ATT, EVENT, MSG>>,
    children: Vec<Node<T, ATT, EVENT, MSG>>,
) -> Node<T, ATT, EVENT, MSG>
where
    ATT: Clone,
{
    element_ns(tag, None, attrs, children)
}

/// creates an virtual node element with namespace
#[inline]
pub fn element_ns<T, ATT, EVENT, MSG>(
    tag: T,
    namespace: Option<&'static str>,
    attrs: Vec<Attribute<ATT, EVENT, MSG>>,
    children: Vec<Node<T, ATT, EVENT, MSG>>,
) -> Node<T, ATT, EVENT, MSG>
where
    ATT: Clone,
{
    Node::Element(Element {
        tag,
        attrs,
        children,
        namespace,
    })
}

/// Create a textnode element
#[inline]
pub fn text<V, T, ATT, EVENT, MSG>(v: V) -> Node<T, ATT, EVENT, MSG>
where
    V: ToString,
    ATT: Clone,
{
    Node::Text(Text {
        text: v.to_string(),
    })
}

/// Create an attribute
#[inline]
pub fn attr<V, ATT, EVENT, MSG>(name: ATT, v: V) -> Attribute<ATT, EVENT, MSG>
where
    V: Into<Value>,
    ATT: Clone,
{
    attr_ns(None, name, v)
}

/// Create an attribute
#[inline]
pub fn attr_ns<V, ATT, EVENT, MSG>(
    namespace: Option<&'static str>,
    name: ATT,
    v: V,
) -> Attribute<ATT, EVENT, MSG>
where
    V: Into<Value>,
    ATT: Clone,
{
    Attribute::with_namespace(name, v.into(), namespace)
}

/// Creates a callback object from the function closure
/// This will then be attached to the browser and emitted
/// when that event is triggered.
///
/// FIXME: callbacks are recrated eveytime, therefore they are not
/// equivalent when compared since function contents
/// can not be compared. Only Rc's are compared.
#[inline]
pub fn on<C, ATT, EVENT, MSG>(name: ATT, c: C) -> Attribute<ATT, EVENT, MSG>
where
    C: Into<Callback<EVENT, MSG>>,
    ATT: Clone,
{
    Attribute::from_callback(name, c.into())
}

/// Create an callback event which has a function
/// to map web_sys::Event to user event
pub fn on_with_extractor<ATT, EVENT, WEV2UDEF, UDEF, UDEF2MSG, MSG>(
    name: ATT,
    webevent_to_user_def: WEV2UDEF,
    user_def_to_msg: UDEF2MSG,
) -> Attribute<ATT, EVENT, MSG>
where
    WEV2UDEF: Fn(EVENT) -> UDEF + 'static,
    UDEF2MSG: Fn(UDEF) -> MSG + 'static,
    MSG: 'static,
    UDEF: 'static,
    EVENT: 'static,
    ATT: Clone,
{
    let cb: Callback<EVENT, UDEF> = Callback::from(webevent_to_user_def);
    let cb2: Callback<EVENT, MSG> = cb.map(user_def_to_msg);
    Attribute::from_callback(name, cb2)
}
