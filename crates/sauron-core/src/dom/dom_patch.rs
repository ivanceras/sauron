use crate::dom::Dispatch;
use crate::vdom::{Attribute, Patch};
use crate::CreatedNode;
use mt_dom::TreePath;
use web_sys::Node;

/// a Patch where the virtual nodes are all created in the document.
/// This is necessary since the CreatedNode doesn't contain references
/// as opposed to Patch which contains reference to the vdom, which makes it hard
/// to be included in a struct
///
/// TODO: use target_element instead of target_node
pub enum DomPatch<MSG> {
    /// Insert nodes before the target node
    InsertBeforeNode {
        /// The path to traverse to get to the target_node
        patch_path: TreePath,
        /// the target node
        target_node: Node,
        /// nodes to be inserted before the target node
        nodes: Vec<CreatedNode>,
    },
    /// Insert nodes after the target node
    InsertAfterNode {
        /// The path to traverse to get to the target_node
        patch_path: TreePath,
        /// the target node
        target_node: Node,
        /// the nodes to be inserted after the target node
        nodes: Vec<CreatedNode>,
    },
    /// Append nodes into the target node
    AppendChildren {
        /// The path to traverse to get to the target_node
        patch_path: TreePath,
        /// the target node
        target_node: Node,
        /// the children nodes to be appended into the target node
        children: Vec<CreatedNode>,
    },
    /// Add attributes to the target node
    AddAttributes {
        /// The path to traverse to get to the target_node
        patch_path: TreePath,
        /// the target node
        target_node: Node,
        /// the attributes to be added to the target node
        attrs: Vec<Attribute<MSG>>,
    },
    /// Remove attributes from the target node
    RemoveAttributes {
        /// The path to traverse to get to the target_node
        patch_path: TreePath,
        /// the target node
        target_node: Node,
        /// the attributes names to be removed
        attrs: Vec<Attribute<MSG>>,
    },
    /// Replace the target node with the replacement node
    ReplaceNode {
        /// The path to traverse to get to the target_node
        patch_path: TreePath,
        /// the target node
        target_node: Node,
        /// the replacement node
        replacement: CreatedNode,
    },
    /// Remove the target node
    RemoveNode {
        /// The path to traverse to get to the target_node
        patch_path: TreePath,
        /// the target node
        target_node: Node,
    },
}

impl<MSG> DomPatch<MSG> {
    /// convert a virtual DOM Patch into a created DOM node Patch
    pub fn from_patch<'a, DSP>(
        program: &DSP,
        target_node: &Node,
        focused_node: &mut Option<Node>,
        patch: &Patch<'a, MSG>,
    ) -> Self
    where
        MSG: 'static,
        DSP: Clone + Dispatch<MSG> + 'static,
    {
        let target_node = target_node.clone();
        match patch {
            Patch::InsertBeforeNode {
                patch_path, nodes, ..
            } => {
                let nodes: Vec<CreatedNode> = nodes
                    .iter()
                    .map(|for_insert| {
                        CreatedNode::create_dom_node::<DSP, MSG>(
                            program,
                            for_insert,
                            focused_node,
                        )
                    })
                    .collect();
                Self::InsertBeforeNode {
                    patch_path: patch_path.clone(),
                    target_node,
                    nodes,
                }
            }
            Patch::InsertAfterNode {
                patch_path, nodes, ..
            } => {
                let nodes: Vec<CreatedNode> = nodes
                    .iter()
                    .map(|for_insert| {
                        CreatedNode::create_dom_node::<DSP, MSG>(
                            program,
                            for_insert,
                            focused_node,
                        )
                    })
                    .collect();
                Self::InsertAfterNode {
                    patch_path: patch_path.clone(),
                    target_node,
                    nodes,
                }
            }

            Patch::AddAttributes {
                patch_path, attrs, ..
            } => Self::AddAttributes {
                patch_path: patch_path.clone(),
                target_node,
                attrs: attrs.iter().map(|a| (*a).clone()).collect(),
            },
            Patch::RemoveAttributes {
                patch_path, attrs, ..
            } => Self::RemoveAttributes {
                patch_path: patch_path.clone(),
                target_node,
                attrs: attrs.iter().map(|a| (*a).clone()).collect(),
            },

            Patch::ReplaceNode {
                patch_path,
                replacement,
                ..
            } => {
                let replacement = CreatedNode::create_dom_node::<DSP, MSG>(
                    program,
                    replacement,
                    focused_node,
                );
                Self::ReplaceNode {
                    patch_path: patch_path.clone(),
                    target_node,
                    replacement,
                }
            }
            Patch::RemoveNode { patch_path, .. } => Self::RemoveNode {
                patch_path: patch_path.clone(),
                target_node,
            },
            Patch::AppendChildren {
                patch_path,
                children,
                ..
            } => {
                let children: Vec<CreatedNode> = children
                    .iter()
                    .map(|for_insert| {
                        CreatedNode::create_dom_node::<DSP, MSG>(
                            program,
                            for_insert,
                            focused_node,
                        )
                    })
                    .collect();
                Self::AppendChildren {
                    patch_path: patch_path.clone(),
                    target_node,
                    children,
                }
            }
        }
    }
}
