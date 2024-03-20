//! Leaf node for html dom tree
use crate::dom::StatefulModel;
use crate::dom::StatelessModel;
use std::borrow::Cow;
use crate::vdom::Node;
use derive_where::derive_where;
use crate::vdom::TemplatedView;


/// A leaf node value of html dom tree
#[derive_where(Clone, Debug)]
pub enum Leaf<MSG> {
    /// Text variant of a virtual node
    Text(Cow<'static, str>),
    /// A safe html variant
    SafeHtml(Cow<'static, str>),
    /// A comment node
    Comment(Cow<'static, str>),
    /// doctype: html, math, svg
    /// <https://www.w3.org/QA/2002/04/valid-dtd-list.html>
    DocType(Cow<'static, str>),
    /// A node containing nodes, this will be unrolled together with the rest of the children of
    /// the node
    NodeList(Vec<Node<MSG>>),
    /// A document fragment node, will be created using fragment node and attached to the dom
    Fragment(Vec<Node<MSG>>),
    /// Stateful Component leaf
    StatefulComponent(StatefulModel<MSG>),
    /// Stateless Component leaf
    StatelessComponent(StatelessModel<MSG>),
    /// a view where a template and skip diff is provided
    TemplatedView(TemplatedView<MSG>),
}



impl<MSG> PartialEq for Leaf<MSG> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Text(v), Self::Text(o)) => v == o,
            (Self::SafeHtml(v), Self::SafeHtml(o)) => v == o,
            (Self::Comment(v), Self::Comment(o)) => v == o,
            (Self::DocType(v), Self::DocType(o)) => v == o,
            (Self::NodeList(v), Self::NodeList(o)) => v == o,
            (Self::Fragment(v), Self::Fragment(o)) => v == o,
            (Self::StatefulComponent(v), Self::StatefulComponent(o)) => v.type_id == o.type_id,
            (Self::StatelessComponent(v), Self::StatelessComponent(o)) => v == o,
            (Self::TemplatedView(v), Self::TemplatedView(o)) => v == o,
            _ => false,
        }
    }
}

impl<MSG> Eq for Leaf<MSG> {}


impl<MSG> Leaf<MSG> {
    /// returns true if this a text node
    pub fn is_text(&self) -> bool {
        matches!(self, Self::Text(_))
    }

    /// returns true if this is a safe html text node
    pub fn is_safe_html(&self) -> bool {
        matches!(self, Self::SafeHtml(_))
    }

    /// return the text content if it is a text node
    pub fn as_text(&self) -> Option<&str> {
        match self {
            Self::Text(ref text) => Some(text),
            _ => None,
        }
    }

    /// return the text content if this a text node,
    pub fn as_safe_html(&self) -> Option<&str> {
        match self {
            Self::SafeHtml(ref html) => Some(html),
            _ => None,
        }
    }

}

impl<MSG> From<&'static str> for Leaf<MSG> {
    fn from(s: &'static str) -> Self {
        Self::Text(Cow::from(s))
    }
}

impl<MSG> From<String> for Leaf<MSG> {
    fn from(s: String) -> Self {
        Self::Text(Cow::from(s))
    }
}
