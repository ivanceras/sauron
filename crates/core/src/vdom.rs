//! vdom stands for virtual-dom.
//! This module contains types that are derived from mt-dom
//! where we assign concrete types into the generics.
//!
//! All the code in this module are run in purely rust environment, that is there is NO code here
//! involves accessing the real DOM.
//!
use crate::dom::Event;
pub use attribute::Attribute;
pub use attribute::Callback;
pub use attribute::GroupedAttributeValues;
pub use element::Element;
pub use leaf::Leaf;
pub use templated_view::TemplatedView;

mod attribute;
mod element;
mod leaf;
mod map_msg;
mod render;
mod templated_view;

pub use attribute::special::{
    key, replace, skip, skip_criteria, KEY, REPLACE, SKIP, SKIP_CRITERIA,
};
#[cfg(feature = "ensure-attr-set")]
pub(crate) use attribute::special::{CHECKED, DISABLED, OPEN, VALUE};
pub use attribute::{attr, attr_ns, AttributeName, AttributeValue, Namespace, Style, Tag, Value};
pub use diff::{diff, diff_recursive};
pub use node::{element, element_ns, fragment, leaf, node_list, Node};
pub use patch::{Patch, PatchType, TreePath};

pub mod diff;
mod diff_lis;
mod node;
pub mod patch;

/// Callback where Event type is supplied
/// for Components
pub type EventCallback<MSG> = Callback<Event, MSG>;

/// Mount callback is used for mounting the component into the DOM
/// This requires no MSG to be emitted
pub type ComponentEventCallback = Callback<Event, ()>;
