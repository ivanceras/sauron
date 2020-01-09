use crate::{
    vnode::{
        AttribValue,
        Attribute,
    },
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
///     let old: Node<&'static str, (), ()> = element(
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
pub fn element<T, EVENT, MSG>(
    tag: T,
    attrs: Vec<Attribute<EVENT, MSG>>,
    children: Vec<Node<T, EVENT, MSG>>,
) -> Node<T, EVENT, MSG> {
    element_ns(tag, None, attrs, children)
}

#[inline]
pub fn element_ns<T, EVENT, MSG>(
    tag: T,
    namespace: Option<&'static str>,
    attrs: Vec<Attribute<EVENT, MSG>>,
    children: Vec<Node<T, EVENT, MSG>>,
) -> Node<T, EVENT, MSG> {
    Node::Element(Element {
        tag,
        attrs,
        children,
        namespace,
    })
}

/// Create a textnode element
#[inline]
pub fn text<V, T, EVENT, MSG>(v: V) -> Node<T, EVENT, MSG>
where
    V: ToString,
{
    Node::Text(Text {
        text: v.to_string(),
    })
}

/// Create an attribute
#[inline]
pub fn attr<V, EVENT, MSG>(name: &'static str, v: V) -> Attribute<EVENT, MSG>
where
    V: Into<Value>,
{
    attr_ns(None, name, v)
}

/// Create an attribute
#[inline]
pub fn attr_ns<V, EVENT, MSG>(
    namespace: Option<&'static str>,
    name: &'static str,
    v: V,
) -> Attribute<EVENT, MSG>
where
    V: Into<Value>,
{
    Attribute {
        name,
        value: AttribValue::Value(v.into()),
        namespace,
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
pub fn on<C, EVENT, MSG>(name: &'static str, c: C) -> Attribute<EVENT, MSG>
where
    C: Into<Callback<EVENT, MSG>>,
{
    Attribute {
        name,
        value: AttribValue::Callback(c.into()),
        namespace: None,
    }
}

/// Create an callback event which has a function
/// to map web_sys::Event to user event
pub fn on_with_extractor<EVENT, WEV2UDEF, UDEF, UDEF2MSG, MSG>(
    name: &'static str,
    webevent_to_user_def: WEV2UDEF,
    user_def_to_msg: UDEF2MSG,
) -> Attribute<EVENT, MSG>
where
    WEV2UDEF: Fn(EVENT) -> UDEF + 'static,
    UDEF2MSG: Fn(UDEF) -> MSG + 'static,
    MSG: 'static,
    UDEF: 'static,
    EVENT: 'static,
{
    let cb: Callback<EVENT, UDEF> = Callback::from(webevent_to_user_def);
    let cb2: Callback<EVENT, MSG> = cb.map(user_def_to_msg);
    Attribute {
        name,
        value: AttribValue::Callback(cb2),
        namespace: None,
    }
}
