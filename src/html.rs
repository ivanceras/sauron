use crate::{
    Attribute,
    Node,
};
pub use sauron_vdom::builder::{
    attr,
    on,
    text,
};

#[macro_use]
pub mod attributes;
#[cfg(feature = "with-dom")]
pub mod events;
pub mod tags;
pub mod units;

pub use tags::commons::*;

/// A help function which render the view when the condition is met, otherwise
/// just display a text("")
pub fn view_if<MSG>(flag: bool, node: Node<MSG>) -> Node<MSG> {
    if flag {
        node
    } else {
        text("")
    }
}

#[inline]
pub fn html_element<MSG>(
    tag: &'static str,
    attrs: Vec<Attribute<MSG>>,
    children: Vec<Node<MSG>>,
) -> Node<MSG> {
    sauron_vdom::builder::element(tag, attrs, children)
}

#[inline]
pub fn html_element_ns<MSG>(
    tag: &'static str,
    namespace: &'static str,
    attrs: Vec<Attribute<MSG>>,
    children: Vec<Node<MSG>>,
) -> Node<MSG> {
    sauron_vdom::builder::element_ns(tag, Some(namespace), attrs, children)
}

#[macro_export]
macro_rules! text {
    ( $($arg: tt)* ) => {
        $crate::html::text(format!($($arg)*))
    };
}
