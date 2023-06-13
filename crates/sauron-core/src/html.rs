//! Provides functions and macros to build html elements
use crate::vdom::{leaf, Attribute, Node, NodeTrait};
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
/// use sauron::{*, html::*};
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
/// use sauron::{*, html::*, svg::attributes::*};
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

/// creates a text node using a formatter
/// # Examples
/// ```rust
/// use sauron::{*, html::*};
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
/// use sauron::{*, html::*};
/// let node: Node<()> = text("hi");
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
/// use sauron::{*,html::*};
///
/// let node: Node<()> = safe_html("<div>In a safe html</div>");
/// ```
pub fn safe_html<S, MSG>(s: S) -> Node<MSG>
where
    S: ToString,
{
    Node::Leaf(leaf::safe_html(s))
}

/// create a comment node
/// # Example
/// ```rust
/// use sauron::{*, html::*};
/// let node: Node<()> = comment("This is a comment");
/// ```
pub fn comment<S, MSG>(s: S) -> Node<MSG>
where
    S: ToString,
{
    Node::Leaf(leaf::comment(s))
}

/// fragment is a list of nodes
/// # Example
/// ```rust
/// use sauron::{*, html::*};
///
/// let node: Node<()> = fragment([div([],[]), span([],[])]);
/// ```
pub fn fragment<MSG>(nodes: impl IntoIterator<Item = Node<MSG>>) -> Node<MSG> {
    Node::Leaf(leaf::fragment(nodes))
}

/// create a doctype
pub fn doctype<MSG>(s: impl ToString) -> Node<MSG> {
    Node::Leaf(leaf::doctype(s))
}

/// create a node which contains a list of nodes
pub fn node_list<MSG>(nodes: impl IntoIterator<Item = Node<MSG>>) -> Node<MSG> {
    Node::NodeList(nodes.into_iter().collect())
}
