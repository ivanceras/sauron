//! Provides functions and macros to build html elements
use crate::vdom;
use crate::vdom::leaf;
use crate::vdom::NodeTrait;
use crate::vdom::{Attribute, Element, Leaf, Node};
pub use jss::units;
pub use mt_dom::{element, element_ns};

#[macro_use]
pub mod attributes;
pub mod tags;

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

/// Creates an html element with the element tag name and namespace
/// This is specifically used for creating svg element where a namespace is needed, otherwise the
/// browser will not render it correctly.
/// # Examples
/// ```rust
/// use sauron::prelude::*;
///
/// let html:Node<()> =
///     html_element(Some("http://www.w3.org/2000/svg"),"svg", vec![width(200), height(200), xmlns("http://www.w3.org/2000/svg")], vec![], false);
/// assert_eq!(node!{<svg width=200 height=200 xmlns="http://www.w3.org/2000/svg"></svg>}, html);
/// ```
#[inline]
pub fn html_element<MSG>(
    namespace: Option<&'static str>,
    tag: &'static str,
    attrs: impl IntoIterator<Item = Attribute<MSG>>,
    children: impl IntoIterator<Item = Node<MSG>>,
    self_closing: bool,
) -> Node<MSG> {
    // we do a correction to children where text node siblings are next to each other by inserting
    // a comment separator in between them, to prevent the browser from merging the 2 text node
    // together
    let mut corrected_children: Vec<Node<MSG>> = vec![];
    for child in children {
        if let Some(last) = corrected_children.last() {
            if last.is_text() {
                corrected_children.push(comment("separator"));
            }
        }
        corrected_children.push(child);
    }
    element_ns(namespace, tag, attrs, corrected_children, self_closing)
}

/// create a custom html element
///
/// # Examples
/// ```rust
/// use sauron::prelude::*;
///
/// let my_clock:Node<()> =
///     custom_element("my-clock", [attr("time", "now")], [button([],[text("a clock")])]);
///
/// let expected = "<my-clock time=\"now\"><button>a clock</button></my-clock>";
/// assert_eq!(expected, my_clock.render_to_string());
/// ```
pub fn custom_element<MSG>(
    custom_tag: vdom::Tag,
    attrs: impl IntoIterator<Item = Attribute<MSG>>,
    children: impl IntoIterator<Item = Node<MSG>>,
) -> Node<MSG> {
    Node::Leaf(Leaf::CustomElement(Element::new(
        None, custom_tag, attrs, children, false,
    )))
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

/// Create a text node element
/// # Example
/// ```rust
/// ```
pub fn text<S, MSG>(s: S) -> Node<MSG>
where
    S: ToString,
{
    Node::Leaf(leaf::text(s))
}

/// Create an html and instruct the DOM renderer and/or DOM patcher that the operation is safe.
///
/// Note: this operation doesn't sanitize the html code. It is your responsibility
/// as a programmer to sanitize the input here.
/// # Example
/// ```rust
/// ```
pub fn safe_html<S, MSG>(s: S) -> Node<MSG>
where
    S: ToString,
{
    Node::Leaf(leaf::safe_html(s))
}

/// create a comment node
pub fn comment<S, MSG>(s: S) -> Node<MSG>
where
    S: ToString,
{
    Node::Leaf(leaf::comment(s))
}
