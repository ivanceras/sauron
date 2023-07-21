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

/// namespace type in node, which could be change to an enum
pub type Namespace = &'static str;
/// tags are using static str for now, can also be enum tags
pub type Tag = &'static str;
/// attribute keys or attribute names
pub type AttributeName = &'static str;

/// A simplified version of saurdon_vdom node, where we supplied the type for the tag
/// which is a &'static str. The missing type is now only MSG which will be supplied by the users
/// App code.
pub type Node<MSG> = mt_dom::Node<Namespace, Tag, Leaf<MSG>, AttributeName, AttributeValue<MSG>>;

/// Element type with tag and attribute name type set to &'static str
pub type Element<MSG> =
    mt_dom::Element<Namespace, Tag, Leaf<MSG>, AttributeName, AttributeValue<MSG>>;

/// Patch as result of diffing the current_vdom and the new vdom.
/// The tag and attribute name types is set to &'static str
pub type Patch<'a, MSG> =
    mt_dom::Patch<'a, Namespace, Tag, Leaf<MSG>, AttributeName, AttributeValue<MSG>>;

/// patch type variant
pub type PatchType<'a, MSG> =
    mt_dom::PatchType<'a, Namespace, Tag, Leaf<MSG>, AttributeName, AttributeValue<MSG>>;
/// Attribute type used in sauron where the type of the Attribute name is &'static str
pub type Attribute<MSG> = mt_dom::Attribute<Namespace, AttributeName, AttributeValue<MSG>>;

/// Callback where Event type is supplied
/// for Components
pub type Listener<MSG> = attributes::Listener<Event, MSG>;

/// This is a sauron html specific functionality
/// diff 2 nodes with attribute using `&'static str` instead of generic ATT
pub fn diff<'a, MSG>(old: &'a Node<MSG>, new: &'a Node<MSG>) -> Vec<Patch<'a, MSG>>
where
    MSG: 'static,
{
    use crate::html::attributes::Special;
    use map_msg::NodeMapMsg;

    // check if the skip attribute is true
    // if it is true, skip diffing and no patches is created at this dom
    let skip = |old_node: &'a Node<MSG>, new_node: &'a Node<MSG>| {
        let new_skip_criteria = new_node.attribute_value(&"skip_criteria");
        let old_skip_criteria = old_node.attribute_value(&"skip_criteria");
        // if old and new skip_criteria didn't change skip diffing this nodes
        match (new_skip_criteria, old_skip_criteria) {
            (Some(new), Some(old)) => new == old,
            _ => new_node
                .get_value("skip")
                .and_then(|v| v.as_bool())
                .unwrap_or(false),
        }
    };

    // check if the replace attribute evaluates to true,
    // if it is, a replace patch replace the old node with the new node
    // without diffing the dom tree
    let replace = |old_node: &'a Node<MSG>, new_node: &'a Node<MSG>| {
        let explicit_replace_attr = new_node
            .get_value("replace")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let old_node_has_event = !old_node.get_callbacks().is_empty();
        let new_node_has_event = !new_node.get_callbacks().is_empty();
        // don't recycle when old node has event while new new doesn't have
        let forbid_recycle = old_node_has_event && !new_node_has_event;

        explicit_replace_attr || forbid_recycle
    };
    mt_dom::diff::diff_with_functions(old, new, &"key", &skip, &replace)
}
