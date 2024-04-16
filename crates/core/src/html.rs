//! Provides functions and macros to build html elements
use crate::vdom;
use crate::vdom::Leaf;
pub use crate::vdom::{element, element_ns};
use crate::vdom::{Attribute, Node};
use std::borrow::Cow;
pub use tags::{commons::*, self_closing::*, *};

#[macro_use]
pub mod attributes;
#[cfg(feature = "with-lookup")]
pub mod lookup;
pub mod tags;
pub mod units;

#[cfg(feature = "with-dom")]
pub use crate::dom::events;

/// A help function which render the view when the condition is met, otherwise
/// just display a `span(vec![], vec![])`
///
/// # Examples
/// ```rust
/// use sauron::*;
///
/// let content = "hello world";
/// let html: Node<()> = view_if(!content.is_empty(), p(vec![], vec![text(content)]));
///
/// assert_eq!(node!{<p>"hello world"</p>}, html);
/// ```
/// Note: that the node here is already evaluated therefore not suitable for building the nodes
/// prior and will end up not being displayed.
/// An alternative would be to just use if else statement like so:
/// ```ignore
/// if flag{
///     expensive_code_to_build_the_view()
/// }else{
///     comment("not yet ready")
/// }
/// ```
pub fn view_if<MSG>(flag: bool, node: Node<MSG>) -> Node<MSG> {
    if flag {
        node
    } else {
        comment("hidden")
    }
}

/// evaluate the fn_node only if flag is true and return the evaluated Node
pub fn lazy_view_if<F, MSG>(flag: bool, fn_node: F) -> Node<MSG>
where
    F: Fn() -> Node<MSG>,
{
    if flag {
        fn_node()
    } else {
        comment("hidden")
    }
}

/// Creates an html element with the element tag name and namespace
/// This is specifically used for creating svg element where a namespace is needed, otherwise the
/// browser will not render it correctly.
/// # Examples
/// ```rust
/// use sauron::{*,html::html_element};
///
/// let html:Node<()> =
///     html_element(Some("http://www.w3.org/2000/svg"),"svg", vec![width(200), height(200), xmlns("http://www.w3.org/2000/svg")], vec![], false);
/// assert_eq!(node!{<svg width=200 height=200 xmlns="http://www.w3.org/2000/svg"></svg>}, html);
/// ```
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
            //TODO: put this behind a flag: #auto-separator to automatically
            //add separator between text nodes
            if last.is_text() && child.is_text() {
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
/// use sauron::*;
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
/// use sauron::*;
/// let node: Node<()> = text("hi");
/// ```
pub fn text<MSG>(s: impl ToString) -> Node<MSG> {
    Node::Leaf(Leaf::Text(Cow::from(s.to_string())))
}

/// create a comment node
/// # Example
/// ```rust
/// use sauron::*;
/// let node: Node<()> = comment("This is a comment");
/// ```
pub fn comment<MSG>(s: impl Into<Cow<'static, str>>) -> Node<MSG> {
    Node::Leaf(Leaf::Comment(s.into()))
}

/// fragment is a list of nodes
/// # Example
/// ```rust
/// use sauron::{*, html::*};
///
/// let node: Node<()> = fragment([div([],[]), span([],[])]);
/// ```
pub fn fragment<MSG>(nodes: impl IntoIterator<Item = Node<MSG>>) -> Node<MSG> {
    vdom::fragment(nodes)
}

/// create a doctype
pub fn doctype<MSG>(s: impl Into<Cow<'static, str>>) -> Node<MSG> {
    Node::Leaf(Leaf::DocType(s.into()))
}

/// create a node which contains a list of nodes
pub fn node_list<MSG>(nodes: impl IntoIterator<Item = Node<MSG>>) -> Node<MSG> {
    Node::Leaf(Leaf::NodeList(nodes.into_iter().collect()))
}

/// Create html entities such as `&nbsp;` `&gt`
pub fn symbol<MSG>(s: &str) -> Node<MSG> {
    let s = escape_html_text(&s);
    Node::Leaf(Leaf::Symbol(s.into()))
}

fn escape_html_text(s: &str) -> String {
    s.chars()
        .map(|ch| match ch {
            '>' => Cow::from("&gt;"),
            '<' => Cow::from("&lt;"),
            '\'' => Cow::from("&#39;"),
            '"' => Cow::from("&quot;"),
            _ => Cow::from(ch.to_string()),
        })
        .collect()
}
