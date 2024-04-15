#![allow(clippy::type_complexity)]

use derive_where::derive_where;
use indexmap::IndexMap;
use crate::vdom::ComponentEventCallback;
use crate::vdom::EventCallback;

pub use attribute_value::AttributeValue;
pub use callback::Callback;
pub use style::Style;
pub use value::Value;

mod attribute_value;
pub mod callback;
pub(crate) mod special;
mod style;
mod value;

/// The type of the Namspace
pub type Namespace = &'static str;

/// The type of the Tag
pub type Tag = &'static str;

/// The type of Attribute Name
pub type AttributeName = &'static str;

/// These are the plain attributes of an element
#[derive_where(Clone, Debug, PartialEq, Eq)]
pub struct Attribute<MSG> {
    /// namespace of an attribute.
    /// This is specifically used by svg attributes
    /// such as xlink-href
    pub namespace: Option<Namespace>,
    /// the attribute name,
    /// optional since style attribute doesn't need to have an attribute name
    pub name: AttributeName,
    /// the attribute value, which could be a simple value, and event or a function call
    pub value: Vec<AttributeValue<MSG>>,
}

/// The Attributes partition into 4 different types
pub struct GroupedAttributeValues<'a, MSG> {
    /// the event listeners
    pub listeners: Vec<&'a EventCallback<MSG>>,
    /// the component event listeners
    pub component_callbacks: Vec<&'a ComponentEventCallback>,
    /// plain attribute values
    pub plain_values: Vec<&'a Value>,
    /// style attribute values
    pub styles: Vec<&'a Style>,
}

impl<MSG> Attribute<MSG> {
    /// create a plain attribute with namespace
    pub fn new(
        namespace: Option<Namespace>,
        name: AttributeName,
        value: AttributeValue<MSG>,
    ) -> Self {
        Attribute {
            name,
            value: vec![value],
            namespace,
        }
    }

    /// create from multiple values
    pub fn with_multiple_values(
        namespace: Option<Namespace>,
        name: AttributeName,
        value: impl IntoIterator<Item = AttributeValue<MSG>>,
    ) -> Self {
        Attribute {
            name,
            value: value.into_iter().collect(),
            namespace,
        }
    }

    /// return the name of this attribute
    pub fn name(&self) -> &AttributeName {
        &self.name
    }

    /// return the value of this attribute
    pub fn value(&self) -> &[AttributeValue<MSG>] {
        &self.value
    }

    /// return the namespace of this attribute
    pub fn namespace(&self) -> Option<&Namespace> {
        self.namespace.as_ref()
    }

    /// returns true if this attribute is an event listener
    pub fn is_event_listener(&self) -> bool {
        self.value
            .first()
            .map(|v| v.is_event_listener())
            .unwrap_or(false)
    }

    /// grouped values into plain, function calls, styles and event listeners
    pub(crate) fn group_values(attr: &Attribute<MSG>) -> GroupedAttributeValues<MSG> {
        let mut listeners = vec![];
        let mut component_callbacks = vec![];
        let mut plain_values = vec![];
        let mut styles = vec![];
        for av in attr.value() {
            match av {
                AttributeValue::Simple(v) => {
                    plain_values.push(v);
                }
                AttributeValue::Style(v) => {
                    styles.extend(v);
                }
                AttributeValue::EventListener(cb) => {
                    listeners.push(cb);
                }
                AttributeValue::ComponentEventListener(cb) => {
                    component_callbacks.push(cb);
                }
                AttributeValue::Empty => (),
            }
        }
        GroupedAttributeValues {
            listeners,
            component_callbacks,
            plain_values,
            styles,
        }
    }

    fn is_just_empty(&self) -> bool {
        self.value
            .first()
            .map(|av| av.is_just_empty())
            .unwrap_or(false)
    }

    /// merge the values of attributes with the same name
    /// also exclude the empty attribute
    pub fn merge_attributes_of_same_name<'a>(
        attributes: impl IntoIterator<Item = &'a Attribute<MSG>> + Iterator,
    ) -> Vec<Attribute<MSG>>
    where
        MSG: 'a,
    {
        let mut merged: IndexMap<&AttributeName, Attribute<MSG>> =
            IndexMap::with_capacity(attributes.size_hint().0);
        for att in attributes.into_iter() {
            if !att.is_just_empty() {
                if let Some(existing) = merged.get_mut(&att.name) {
                    existing.value.extend(att.value.clone());
                } else {
                    merged.insert(
                        &att.name,
                        Attribute {
                            namespace: att.namespace,
                            name: att.name,
                            value: att.value.clone(),
                        },
                    );
                }
            }
        }
        merged.into_values().collect()
    }

    /// group attributes of the same name
    #[doc(hidden)]
    pub fn group_attributes_per_name<'a>(
        attributes: impl IntoIterator<Item = &'a Attribute<MSG>> + Iterator,
    ) -> IndexMap<&'a AttributeName, Vec<&'a Attribute<MSG>>> {
        let mut grouped: IndexMap<&'a AttributeName, Vec<&'a Attribute<MSG>>> =
            IndexMap::with_capacity(attributes.size_hint().0);
        for attr in attributes {
            if let Some(existing) = grouped.get_mut(&attr.name) {
                existing.push(attr);
            } else {
                grouped.insert(&attr.name, vec![attr]);
            }
        }
        grouped
    }
}

/// Create an attribute
/// # Example
/// ```rust
/// use sauron::vdom::{Attribute,attr};
/// let class: Attribute<()> = attr("class", "container");
/// ```
#[inline]
pub fn attr<MSG>(name: AttributeName, value: impl Into<AttributeValue<MSG>>) -> Attribute<MSG> {
    attr_ns(None, name, value)
}

/// Create an attribute with namespace
/// # Example
/// ```rust
/// use sauron::vdom::{Attribute,attr_ns};
///
/// let href: Attribute<()> = attr_ns(Some("http://www.w3.org/1999/xlink"), "href", "cool-script.js");
/// ```
#[inline]
pub fn attr_ns<MSG>(
    namespace: Option<Namespace>,
    name: AttributeName,
    value: impl Into<AttributeValue<MSG>>,
) -> Attribute<MSG> {
    Attribute::new(namespace, name, value.into())
}
