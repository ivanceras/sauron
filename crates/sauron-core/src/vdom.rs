//! vdom stands for virtual-dom.
//! This module contains types that are derived from mt-dom
//! where we assign concrete types into the generics.
//!
//! All the code in this module are run in purely rust environment, that is there is NO code here
//! involves accessing the real DOM.
//!
pub use crate::html::attributes::AttributeValue;
use crate::{dom::Event, html::attributes};
pub use leaf::Leaf;
pub use node_trait::NodeTrait;

pub mod leaf;
pub(crate) mod map_msg;
mod node_trait;


pub use diff::{diff, diff_recursive};
pub use node::{
    attribute::{
        attr, attr_ns, group_attributes_per_name, merge_attributes_of_same_name,
        Tag, KEY,  Namespace, AttributeName,
    },
    element, element_ns, fragment, leaf, node_list, Attribute, Element, Node,
};
pub use patch::{Patch, PatchType, TreePath};

pub mod diff;
mod diff_lis;
mod node;
pub mod patch;


/// Callback where Event type is supplied
/// for Components
pub type Listener<MSG> = attributes::Listener<Event, MSG>;

