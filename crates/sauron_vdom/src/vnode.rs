use std::collections::BTreeMap;
use std::fmt;

pub mod builder;
mod event;
mod value;

use crate::Callback;
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
pub enum Node<T, MSG> {
    Element(Element<T, MSG>),
    Text(Text),
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Element<T, MSG> {
    pub tag: T,
    pub attrs: BTreeMap<String, Value>,
    pub events: BTreeMap<String, Callback<Event, MSG>>,
    pub children: Vec<Node<T, MSG>>,
    pub namespace: Option<String>,
}

impl<T, MSG> Node<T, MSG> {
    /// map the return of the callback from MSG to MSG2
    pub fn map<F, MSG2>(self, func: F) -> Node<T, MSG2>
    where
        F: Fn(MSG) -> MSG2 + 'static + Clone,
        MSG: 'static,
    {
        match self {
            Node::Element(element) => Node::Element(element.map(func)),
            Node::Text(text) => Node::Text(Text::new(text.text)),
        }
    }
}

impl<T, MSG> Element<T, MSG> {
    /// map the return of the callback from MSG to MSG2
    pub fn map<F, MSG2>(self, func: F) -> Element<T, MSG2>
    where
        F: Fn(MSG) -> MSG2 + 'static + Clone,
        MSG: 'static,
    {
        let mut new_element: Element<T, MSG2> = Element {
            tag: self.tag,
            attrs: self.attrs,
            namespace: self.namespace,
            children: vec![],
            events: BTreeMap::new(),
        };
        for child in self.children {
            let new_child = child.map(func.clone());
            new_element.children.push(new_child);
        }
        for (event, cb) in self.events {
            // map the callback to return something else
            let new_cb = cb.map(func.clone());
            new_element.events.insert(event, new_cb);
        }
        new_element
    }
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Text {
    pub text: String,
}

impl<T, MSG> Element<T, MSG> {
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

impl<T, MSG> fmt::Display for Element<T, MSG>
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
impl<T, MSG> fmt::Display for Node<T, MSG>
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

impl<T, MSG> From<Element<T, MSG>> for Node<T, MSG> {
    fn from(v: Element<T, MSG>) -> Self {
        Node::Element(v)
    }
}
