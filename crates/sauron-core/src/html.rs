//! Provides functions and macros to build html elements
use crate::{Attribute, Node};
pub use mt_dom::{comment, element, element_ns, safe_html, text};

#[macro_use]
pub mod attributes;
pub mod tags;
pub mod units;

#[cfg(feature = "with-dom")]
pub use crate::dom::events;

pub use tags::{commons::*, self_closing::*};

/// A help function which render the view when the condition is met, otherwise
/// just display a `span(vec![], vec![])`
///
/// # Examples
/// ```rust
/// use sauron::prelude::*;
///
/// let content = "hello world";
/// let html: Node<()> = view_if(!content.is_empty(), p(vec![], vec![text(content)]));
///
/// assert_eq!(node!{<p>"hello world"</p>}, html);
/// ```
pub fn view_if<MSG>(flag: bool, node: Node<MSG>) -> Node<MSG> {
    if flag {
        node
    } else {
        comment("hidden")
    }
}

/// Creates an html element
///
/// # Examples
/// ```rust
/// use sauron::prelude::*;
///
/// let container:Node<()> = html_element("div", vec![class("container")], vec![]);
/// assert_eq!(node!{<div class="container"></div>},container);
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

/// Create a self closing html element.
/// # Examples
/// ```ignore
/// use sauron::prelude::*;
///
/// let photo:Node<()> = html_element_self_closing("img", vec![class("pic")], vec![], true);
/// assert_eq!(node!{<img class="pic"/>}, photo);
/// ```
#[inline]
pub fn html_element_self_closing<MSG>(
    tag: &'static str,
    attrs: Vec<Attribute<MSG>>,
    children: Vec<Node<MSG>>,
    self_closing: bool,
) -> Node<MSG> {
    element_ns(None, tag, attrs, children, self_closing)
}

/// Creates an html element with the element tag name and namespace
/// This is specifically used for creating svg element where a namespace is needed, otherwise the
/// browser will not render it correctly.
/// # Examples
/// ```rust
/// use sauron::prelude::*;
///
/// let html:Node<()> =
///     html_element_ns("svg","http://www.w3.org/2000/svg", vec![width(200), height(200), xmlns("http://www.w3.org/2000/svg")], vec![]);
/// assert_eq!(node!{<svg width=200 height=200 xmlns="http://www.w3.org/2000/svg"></svg>}, html);
/// ```
#[inline]
pub fn html_element_ns<MSG>(
    tag: &'static str,
    namespace: &'static str,
    attrs: Vec<Attribute<MSG>>,
    children: Vec<Node<MSG>>,
) -> Node<MSG> {
    element_ns(Some(namespace), tag, attrs, children, false)
}

/// creates a text node using a formatter
/// # Examples
/// ```rust
/// use sauron::prelude::*;
///
/// let number = 42;
/// let title:Node<()> = h1(vec![], vec![text!("This is the content number: {}", number)]);
///
/// assert_eq!(node!{<h1>"This is the content number: 42"</h1>}, title);
/// ```
#[macro_export]
macro_rules! text {
    ( $($arg: tt)* ) => {
        $crate::html::text(format!($($arg)*))
    };
}
