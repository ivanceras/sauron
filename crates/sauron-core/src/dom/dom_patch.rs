use crate::dom::CreatedNode;
use crate::dom::Dispatch;
use crate::vdom::{Attribute, Patch, PatchType};
use mt_dom::TreePath;
use web_sys::Element;

/// a Patch where the virtual nodes are all created in the document.
/// This is necessary since the CreatedNode doesn't contain references
/// as opposed to Patch which contains reference to the vdom, which makes it hard
/// to be included in a struct
///
pub struct DomPatch<MSG> {
    /// The path to traverse to get to the target_element
    pub patch_path: TreePath,
    /// the target node
    pub target_element: Element,
    /// the patch variant
    pub patch_variant: PatchVariant<MSG>,
}

/// patch variant
pub enum PatchVariant<MSG> {
    /// Insert nodes before the target node
    InsertBeforeNode {
        /// nodes to be inserted before the target node
        nodes: Vec<CreatedNode>,
    },
    /// Insert nodes after the target node
    InsertAfterNode {
        /// the nodes to be inserted after the target node
        nodes: Vec<CreatedNode>,
    },
    /// Append nodes into the target node
    AppendChildren {
        /// the children nodes to be appended into the target node
        children: Vec<CreatedNode>,
    },
    /// Add attributes to the target node
    AddAttributes {
        /// the attributes to be added to the target node
        attrs: Vec<Attribute<MSG>>,
    },
    /// Remove attributes from the target node
    RemoveAttributes {
        /// the attributes names to be removed
        attrs: Vec<Attribute<MSG>>,
    },
    /// Replace the target node with the replacement node
    ReplaceNode {
        /// the replacement node
        replacement: Vec<CreatedNode>,
    },
    /// Remove the target node
    RemoveNode,
}

impl<MSG> DomPatch<MSG> {
    /// convert a virtual DOM Patch into a created DOM node Patch
    pub fn from_patch<DSP>(program: &DSP, target_element: &Element, patch: &Patch<MSG>) -> Self
    where
        MSG: 'static,
        DSP: Clone + Dispatch<MSG> + 'static,
    {
        let target_element = target_element.clone();
        let Patch {
            patch_path,
            patch_type,
            ..
        } = patch;

        let patch_path = patch_path.clone();

        match patch_type {
            PatchType::InsertBeforeNode { nodes } => {
                let nodes: Vec<CreatedNode> = nodes
                    .iter()
                    .map(|for_insert| CreatedNode::create_dom_node::<DSP, MSG>(program, for_insert))
                    .collect();
                Self {
                    patch_path,
                    target_element,
                    patch_variant: PatchVariant::InsertBeforeNode { nodes },
                }
            }
            PatchType::InsertAfterNode { nodes } => {
                let nodes: Vec<CreatedNode> = nodes
                    .iter()
                    .map(|for_insert| CreatedNode::create_dom_node::<DSP, MSG>(program, for_insert))
                    .collect();
                Self {
                    patch_path,
                    target_element,
                    patch_variant: PatchVariant::InsertAfterNode { nodes },
                }
            }

            PatchType::AddAttributes { attrs } => Self {
                patch_path,
                target_element,
                patch_variant: PatchVariant::AddAttributes {
                    attrs: attrs.iter().map(|a| (*a).clone()).collect(),
                },
            },
            PatchType::RemoveAttributes { attrs } => Self {
                patch_path,
                target_element,
                patch_variant: PatchVariant::RemoveAttributes {
                    attrs: attrs.iter().map(|a| (*a).clone()).collect(),
                },
            },

            PatchType::ReplaceNode { replacement } => {
                let replacement: Vec<CreatedNode> = replacement
                    .iter()
                    .map(|node| CreatedNode::create_dom_node::<DSP, MSG>(program, node))
                    .collect();
                Self {
                    patch_path,
                    target_element,
                    patch_variant: PatchVariant::ReplaceNode { replacement },
                }
            }
            PatchType::RemoveNode => Self {
                patch_path,
                target_element,
                patch_variant: PatchVariant::RemoveNode,
            },
            PatchType::AppendChildren { children } => {
                let children: Vec<CreatedNode> = children
                    .iter()
                    .map(|for_insert| CreatedNode::create_dom_node::<DSP, MSG>(program, for_insert))
                    .collect();

                Self {
                    patch_path,
                    target_element,
                    patch_variant: PatchVariant::AppendChildren { children },
                }
            }
        }
    }
}
