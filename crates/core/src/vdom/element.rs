use super::attribute::{AttributeName, Namespace, Tag};
use super::{Attribute, Node};

use crate::vdom::AttributeValue;
use crate::vdom::Leaf;
use crate::vdom::Value;
use derive_where::derive_where;
use indexmap::IndexMap;

/// Represents an element of the virtual node
/// An element has a generic tag, this tag could be a static str tag, such as usage in html dom.
///     Example of which are `div`, `a`, `input`, `img`, etc.
///
/// Tag is a generic type, which can represent a different DOM tree other than the html dom
/// such as widgets in native platform such as gtk, example of which are `Hpane`, `Vbox`, `Image`,
///
/// An element can have an optional namespace, such in the case for html dom where namespace like
/// HTML and SVG, which needs to specified in order to create the DOM element to work on the
/// browser.
///
/// The namespace is also needed in attributes where namespace are necessary such as `xlink:href`
/// where the namespace `xlink` is needed in order for the linked element in an svg image to work.
#[derive_where(Clone, Debug, PartialEq, Eq)]
pub struct Element<MSG> {
    /// namespace of this element,
    /// svg elements requires namespace to render correcly in the browser
    pub namespace: Option<Namespace>,
    /// the element tag, such as div, a, button
    pub tag: Tag,
    /// attributes for this element
    pub(crate) attrs: Vec<Attribute<MSG>>,
    /// children elements of this element
    pub(crate) children: Vec<Node<MSG>>,
    /// is the element has a self closing tag
    pub self_closing: bool,
}

impl<MSG> Element<MSG> {
    /// create a new instance of an element
    pub fn new(
        namespace: Option<Namespace>,
        tag: Tag,
        attrs: impl IntoIterator<Item = Attribute<MSG>>,
        children: impl IntoIterator<Item = Node<MSG>>,
        self_closing: bool,
    ) -> Self {
        //unroll the nodelist
        let children = children
            .into_iter()
            .flat_map(|child| match child {
                Node::Leaf(Leaf::NodeList(node_list)) => node_list,
                _ => vec![child],
            })
            .collect();
        Self {
            namespace,
            tag,
            attrs: attrs.into_iter().collect(),
            children,
            self_closing,
        }
    }

    /// add attributes to this element
    pub fn add_attributes(&mut self, attrs: impl IntoIterator<Item = Attribute<MSG>>) {
        self.attrs.extend(attrs)
    }

    /// add children virtual node to this element
    pub fn add_children(&mut self, children: impl IntoIterator<Item = Node<MSG>>) {
        self.children.extend(children);
    }

    /// returns a refernce to the children of this node
    pub fn children(&self) -> &[Node<MSG>] {
        &self.children
    }

    /// returns a mutable reference to the children of this node
    pub fn children_mut(&mut self) -> &mut [Node<MSG>] {
        &mut self.children
    }

    /// Removes an child node  from this element and returns it.
    ///
    /// The removed child is replaced by the last child of the element's children.
    ///
    /// # Panics
    /// Panics if index is out of bounds in children
    ///
    pub fn swap_remove_child(&mut self, index: usize) -> Node<MSG> {
        self.children.swap_remove(index)
    }

    /// Swaps the 2 child node in this element
    ///
    /// # Arguments
    /// * a - The index of the first child node
    /// * b - The index of the second child node
    ///
    /// # Panics
    /// Panics if both `a` and `b` are out of bounds
    ///
    pub fn swap_children(&mut self, a: usize, b: usize) {
        self.children.swap(a, b)
    }

    /// consume self and return the children
    pub fn take_children(self) -> Vec<Node<MSG>> {
        self.children
    }

    /// return a reference to the attribute of this element
    pub fn attributes(&self) -> &[Attribute<MSG>] {
        &self.attrs
    }

    /// consume self and return the attributes
    pub fn take_attributes(self) -> Vec<Attribute<MSG>> {
        self.attrs
    }

    /// return the namespace of this element
    pub fn namespace(&self) -> Option<&Namespace> {
        self.namespace.as_ref()
    }

    /// return the tag of this element
    pub fn tag(&self) -> &Tag {
        &self.tag
    }

    /// consume self and return the tag of this element
    pub fn take_tag(self) -> Tag {
        self.tag
    }

    /// change the tag of this element
    pub fn set_tag(&mut self, tag: Tag) {
        self.tag = tag;
    }

    /// remove the attributes with this key
    pub fn remove_attribute(&mut self, name: &AttributeName) {
        self.attrs.retain(|att| att.name != *name)
    }

    /// remove the existing values of this attribute
    /// and add the new values
    pub fn set_attributes(&mut self, attrs: impl IntoIterator<Item = Attribute<MSG>>) {
        for attr in attrs {
            self.remove_attribute(&attr.name);
            self.attrs.push(attr);
        }
    }

    /// merge to existing attributes if it exist
    pub fn merge_attributes(&mut self, new_attrs: impl IntoIterator<Item = Attribute<MSG>>) {
        for new_att in new_attrs {
            if let Some(existing_attr) = self.attrs.iter_mut().find(|att| att.name == new_att.name)
            {
                existing_attr.value.extend(new_att.value);
            } else {
                self.attrs.push(new_att);
            }
        }
    }

    /// return all the attribute values which the name &AttributeName
    pub fn attribute_value(&self, name: &AttributeName) -> Option<Vec<&AttributeValue<MSG>>> {
        let result: Vec<&AttributeValue<MSG>> = self
            .attrs
            .iter()
            .filter(|att| att.name == *name)
            .flat_map(|att| att.value())
            .collect();

        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    }

    /// get the first value of the attribute which has the name `att_name` of this element
    pub fn first_value(&self, att_name: &AttributeName) -> Option<&Value> {
        self.attribute_value(att_name)
            .and_then(|att_values| att_values.first().and_then(|v| v.get_simple()))
    }

    /// grouped the attributes, but retain the index of the attribute
    /// relative to its location in the element
    pub fn group_indexed_attributes_per_name<'a>(
        &'a self,
    ) -> IndexMap<&'a AttributeName, Vec<(usize, &'a Attribute<MSG>)>> {
        let mut grouped: IndexMap<&'a AttributeName, Vec<(usize, &'a Attribute<MSG>)>> =
            IndexMap::new();
        for (i, attr) in self.attributes().iter().enumerate() {
            if let Some(existing) = grouped.get_mut(&attr.name) {
                existing.push((i, attr));
            } else {
                grouped.insert(&attr.name, vec![(i, attr)]);
            }
        }
        grouped
    }

    /// return true if this element has a mount callback
    pub fn has_mount_callback(&self) -> bool {
        self.attributes().iter().any(|a|a.is_mount_callback())
    }
}
