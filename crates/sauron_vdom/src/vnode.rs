use std::collections::BTreeMap;
use std::fmt;

use crate::Callback;

pub mod builder;
mod event;
mod value;

pub use event::{Event, InputEvent, KeyEvent, MouseButton, MouseEvent};
pub use value::Value;

/// When building your views you'll typically use the `html!` macro to generate
/// `VirtualNode`'s.
///
/// `html! { <div> <span></span> </div> }` really generates a `VirtualNode` with
/// one child (span).
///
/// Later, on the client side, you'll use the `diff` and `patch` modules to
/// update the real DOM with your latest tree of virtual nodes (virtual dom).
///
/// Or on the server side you'll just call `.to_string()` on your root virtual node
/// in order to recursively render the node and all of its children.
///
/// TODO: Make all of these fields private and create accessor methods
/// TODO: Create a builder to create instances of VirtualNode::Element with
/// attrs and children without having to explicitly create a Element
#[derive(Debug, PartialEq, Clone)]
pub enum Node<T> {
    Element(Element<T>),
    Text(Text),
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Element<T> {
    pub tag: T,
    pub attrs: BTreeMap<String, Value>,
    pub events: BTreeMap<String, Callback<Event>>,
    pub children: Vec<Node<T>>,
    pub namespace: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Text {
    pub text: String,
}

impl<T> Element<T> {
    /// Create a Element using the supplied tag name
    pub fn new(tag: T) -> Self {
        Element {
            tag,
            attrs: BTreeMap::new(),
            events: BTreeMap::new(),
            children: vec![],
            namespace: None,
        }
    }

    /// set the namespace of this element
    pub fn namespace(mut self, namespace: &str) -> Self {
        self.namespace = Some(namespace.to_string());
        self
    }
}

impl<T> fmt::Display for Element<T>
where
    T: ToString,
{
    // Turn a Element and all of it's children (recursively) into an HTML string
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{}", self.tag.to_string()).unwrap();

        for (attr, value) in self.attrs.iter() {
            write!(f, r#" {}="{}""#, attr, value)?;
        }

        write!(f, ">")?;

        for child in self.children.iter() {
            write!(f, "{}", child.to_string())?;
        }

        write!(f, "</{}>", self.tag.to_string())?;

        Ok(())
    }
}

impl Text {
    pub fn new<S: Into<String>>(s: S) -> Self {
        Text { text: s.into() }
    }
}

// Turn a Text into an HTML string
impl fmt::Display for Text {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}

// Turn a Node into an HTML string (delegate impl to variants)
impl<T> fmt::Display for Node<T>
where
    T: ToString,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Node::Element(element) => write!(f, "{}", element),
            Node::Text(text) => write!(f, "{}", text),
        }
    }
}

impl<T> From<Element<T>> for Node<T> {
    fn from(v: Element<T>) -> Self {
        Node::Element(v)
    }
}
