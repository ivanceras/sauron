//! Provides functions and macros to build html elements
use crate::{
    Attribute,
    Node,
};
pub use mt_dom::{
    element,
    element_ns,
    text,
};

#[macro_use]
pub mod attributes;
pub mod tags;
pub mod units;

#[cfg(feature = "with-dom")]
pub use crate::dom::events;

pub use tags::{
    commons::*,
    self_closing::*,
};

/// A help function which render the view when the condition is met, otherwise
/// just display a text("")
pub fn view_if<MSG>(flag: bool, node: Node<MSG>) -> Node<MSG> {
    if flag {
        node
    } else {
        span(vec![], vec![])
    }
}

/// creates an html element, where the first argument: tag is the html element tag.
/// Example:
/// ```rust,ignore
/// html_element("div", vec![class("container")], vec![])
/// ```
///
#[inline]
pub fn html_element<MSG>(
    tag: &'static str,
    attrs: Vec<Attribute<MSG>>,
    children: Vec<Node<MSG>>,
) -> Node<MSG> {
    element(tag, attrs, children)
}

/// for self closing tags
#[inline]
pub fn html_element_sc<MSG>(
    tag: &'static str,
    attrs: Vec<Attribute<MSG>>,
    children: Vec<Node<MSG>>,
    self_closing: bool,
) -> Node<MSG> {
    element_ns(None, tag, attrs, children, self_closing)
}

/// creates an html element with the element tag name and namespace
/// This is specifically used for creating svg element where a namespace is needed, otherwise the
/// browser will not render it correctly.
#[inline]
pub fn html_element_ns<MSG>(
    tag: &'static str,
    namespace: &'static str,
    attrs: Vec<Attribute<MSG>>,
    children: Vec<Node<MSG>>,
) -> Node<MSG> {
    element_ns(Some(namespace), tag, attrs, children, false)
}

/// creates a text node
/// Example
/// ```rust,ignore
/// h1(vec![], vec![text("This is the content")])
/// ```
/// will produce the corresponding html document:
/// <h1>This is the content</h1>
///
#[macro_export]
macro_rules! text {
    ( $($arg: tt)* ) => {
        $crate::html::text(format!($($arg)*))
    };
}
