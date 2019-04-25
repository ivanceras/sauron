use std::collections::BTreeMap;
use std::fmt;

pub mod builder;
mod event;
mod value;

pub use event::{Event, InputEvent, KeyEvent, MouseButton, MouseEvent};
pub use value::Value;

/// This is the core data structure of the library.
/// Any tree can be represented by `Node`.
/// The `T` is generic instead of just using plain `&'static str`
/// in order for this library to be used not only in html based widget
/// but can also be used to represent native GUI widgets
/// in various platforms.
///
/// Note: Clone is necessary for the aesthetics in the construction of node through series of function
/// calls.
/// Without Clone, the user code would look like these:
/// ```ignore
///     div(&[class("some-class"), &[text("Some text")])
/// ```
/// as compared to
/// ```ignore
///     div([class("some-class"), [text("some text)])
/// ```
/// Cloning is only done once, and happens when constructing the views into a node tree.
/// Cloning also allows flexibility such as adding more children into an existing node/element.
#[derive(Debug, PartialEq, Clone)]
pub enum Node<T, CB> {
    Element(Element<T, CB>),
    Text(Text),
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Element<T, CB> {
    pub tag: T,
    pub attrs: BTreeMap<String, Value>,
    pub events: BTreeMap<String, CB>,
    pub children: Vec<Node<T, CB>>,
    pub namespace: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Text {
    pub text: String,
}

impl<T, CB> Element<T, CB> {
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

impl<T, CB> fmt::Display for Element<T, CB>
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
impl<T, CB> fmt::Display for Node<T, CB>
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

impl<T, CB> From<Element<T, CB>> for Node<T, CB> {
    fn from(v: Element<T, CB>) -> Self {
        Node::Element(v)
    }
}
