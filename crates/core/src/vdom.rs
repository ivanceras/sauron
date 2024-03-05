//! vdom stands for virtual-dom.
//! This module contains types that are derived from mt-dom
//! where we assign concrete types into the generics.
//!
//! All the code in this module are run in purely rust environment, that is there is NO code here
//! involves accessing the real DOM.
//!
use crate::dom::Event;
pub use attribute::Attribute;
pub use element::Element;
pub use leaf::Leaf;
pub use node_trait::NodeTrait;
pub use attribute::Callback;

mod attribute;
mod element;
mod leaf;
mod map_msg;
mod node_trait;

pub use attribute::{
    attr, attr_ns, merge_attributes_of_same_name, AttributeName,
    AttributeValue, Namespace, Style, Tag, Value,
};
pub use diff::{diff, diff_recursive};
pub use node::{element, element_ns, fragment, leaf, node_list, Node};
pub use patch::{Patch, PatchType, TreePath};
pub(crate) use attribute::special::{KEY, REPLACE, SKIP, SKIP_CRITERIA};
pub use attribute::special::{key, replace, skip, skip_criteria};

pub mod diff;
mod diff_lis;
mod node;
pub mod patch;

/// Callback where Event type is supplied
/// for Components
pub type EventCallback<MSG> = Callback<Event, MSG>;