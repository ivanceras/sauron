use std::{collections::BTreeMap,
          fmt};

pub mod builder;
pub mod event;
mod value;

use crate::Callback;
pub use event::Event;
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
pub enum Node<T, EVENT, MSG> {
    Element(Element<T, EVENT, MSG>),
    Text(Text),
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Element<T, EVENT, MSG> {
    pub tag: T,
    pub attrs: BTreeMap<&'static str, Value>,
    pub events: BTreeMap<&'static str, Callback<EVENT, MSG>>,
    pub children: Vec<Node<T, EVENT, MSG>>,
    pub namespace: Option<&'static str>,
}

impl<T, EVENT, MSG> Node<T, EVENT, MSG>
    where EVENT: 'static,
          MSG: 'static
{
    /// map the return of the callback from MSG to MSG2
    pub fn map<F, MSG2>(self, func: F) -> Node<T, EVENT, MSG2>
        where F: Fn(MSG) -> MSG2 + 'static + Clone
    {
        match self {
            Node::Element(element) => Node::Element(element.map(func)),
            Node::Text(text) => Node::Text(Text::new(text.text)),
        }
    }
}

impl<T, EVENT, MSG> Element<T, EVENT, MSG>
    where EVENT: 'static,
          MSG: 'static
{
    /// map the return of the callback from MSG to MSG2
    pub fn map<F, MSG2>(self, func: F) -> Element<T, EVENT, MSG2>
        where F: Fn(MSG) -> MSG2 + 'static + Clone
    {
        let mut new_element: Element<T, EVENT, MSG2> =
            Element { tag: self.tag,
                      attrs: self.attrs,
                      namespace: self.namespace,
                      children: vec![],
                      events: BTreeMap::new() };
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

impl<T, EVENT, MSG> Element<T, EVENT, MSG>
    where T: Clone,
          MSG: Clone,
          EVENT: Clone
{
    #[inline]
    pub fn new(tag: T) -> Self {
        Self::with_children(tag, [])
    }

    /// Create a Element using the supplied tag name
    #[inline]
    pub fn with_children<C>(tag: T, children: C) -> Self
        where C: AsRef<[Node<T, EVENT, MSG>]>
    {
        Self::with_children_and_maybe_ns(tag, children, None)
    }

    pub fn with_children_and_maybe_ns<C>(tag: T,
                                         children: C,
                                         ns: Option<&'static str>)
                                         -> Self
        where C: AsRef<[Node<T, EVENT, MSG>]>
    {
        Element { tag,
                  attrs: BTreeMap::new(),
                  events: BTreeMap::new(),
                  children: children.as_ref().to_vec(),
                  namespace: ns }
    }
}

impl<T, EVENT, MSG> fmt::Display for Element<T, EVENT, MSG> where T: ToString
{
    // Turn a Element and all of it's children (recursively) into an HTML string
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{}", self.tag.to_string())?;

        for (attr, value) in self.attrs.iter() {
            write!(f, r#" {}="{}""#, attr, value)?;
        }

        write!(f, ">")?;

        for child in self.children.iter() {
            write!(f, "{}", child)?;
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
impl<T, EVENT, MSG> fmt::Display for Node<T, EVENT, MSG> where T: ToString
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Node::Element(element) => write!(f, "{}", element),
            Node::Text(text) => write!(f, "{}", text),
        }
    }
}

impl<T, EVENT, MSG> From<Element<T, EVENT, MSG>> for Node<T, EVENT, MSG> {
    fn from(v: Element<T, EVENT, MSG>) -> Self {
        Node::Element(v)
    }
}
