use crate::dom::apply_patches::*;
use crate::dom::created_node::ActiveClosure;
use crate::dom::Dispatch;
use crate::vdom::{Attribute, AttributeValue, Patch};
use crate::CreatedNode;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Element, Node};

/// a Patch where the virtual nodes are all created in the document.
/// This is necessary since the CreatedNode doesn't contain references
/// as opposed to Patch which contains reference to the vdom, which makes it hard
/// to be included in a struct
///
/// TODO: use target_element instead of target_node
pub enum DomPatch<MSG> {
    /// Insert nodes before the target node
    InsertBeforeNode {
        /// the target node
        target_node: Node,
        /// nodes to be inserted before the target node
        nodes: Vec<CreatedNode>,
    },
    /// Insert nodes after the target node
    InsertAfterNode {
        /// the target node
        target_node: Node,
        /// the nodes to be inserted after the target node
        nodes: Vec<CreatedNode>,
    },
    /// Append nodes into the target node
    AppendChildren {
        /// the target node
        target_node: Node,
        /// the children nodes to be appended into the target node
        children: Vec<CreatedNode>,
    },
    /// Add attributes to the target node
    AddAttributes {
        /// the target node
        target_node: Node,
        /// the attributes to be added to the target node
        attrs: Vec<Attribute<MSG>>,
    },
    /// Remove attributes from the target node
    RemoveAttributes {
        /// the target node
        target_node: Node,
        /// the attributes names to be removed
        attrs: Vec<Attribute<MSG>>,
    },
    /// Replace the target node with the replacement node
    ReplaceNode {
        /// the target node
        target_node: Node,
        /// the replacement node
        replacement: CreatedNode,
    },
    /// Remove the target node
    RemoveNode {
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
            Patch::InsertBeforeNode { nodes, .. } => {
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
                Self::InsertBeforeNode { target_node, nodes }
            }
            Patch::InsertAfterNode { nodes, .. } => {
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
                Self::InsertAfterNode { target_node, nodes }
            }

            Patch::AddAttributes { attrs, .. } => Self::AddAttributes {
                target_node,
                attrs: attrs.iter().map(|a| (*a).clone()).collect(),
            },
            Patch::RemoveAttributes { attrs, .. } => Self::RemoveAttributes {
                target_node,
                attrs: attrs.iter().map(|a| (*a).clone()).collect(),
            },

            Patch::ReplaceNode { replacement, .. } => {
                let replacement = CreatedNode::create_dom_node::<DSP, MSG>(
                    program,
                    replacement,
                    focused_node,
                );
                Self::ReplaceNode {
                    target_node,
                    replacement,
                }
            }
            Patch::RemoveNode { .. } => Self::RemoveNode { target_node },
            Patch::AppendChildren { children, .. } => {
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
                    target_node,
                    children,
                }
            }
        }
    }

    /// execute the dom patch
    pub fn apply<DSP>(
        self,
        program: &DSP,
        active_closures: &mut ActiveClosure,
    ) -> Result<(), JsValue>
    where
        MSG: 'static,
        DSP: Clone + Dispatch<MSG> + 'static,
    {
        match self {
            Self::InsertBeforeNode { target_node, nodes } => {
                // we inser the node before this target element
                let target_element: &Element = target_node.unchecked_ref();
                if let Some(parent_target) = target_element.parent_node() {
                    for for_insert in nodes {
                        parent_target
                            .insert_before(
                                &for_insert.node,
                                Some(target_element),
                            )
                            .expect("must remove target node");

                        active_closures.extend(for_insert.closures);
                    }
                } else {
                    panic!("unable to get parent node of the target element: {:?} for patching: {:#?}", target_element, nodes);
                }
            }

            Self::InsertAfterNode { target_node, nodes } => {
                // we insert the node before this target element
                let target_element: &Element = target_node.unchecked_ref();
                for for_insert in nodes.into_iter().rev() {
                    let created_element: &Element = for_insert
                        .node
                        .dyn_ref()
                        .expect("only elements is supported for now");
                    target_element
                        .insert_adjacent_element("afterend", created_element)
                        .expect("must remove target node");
                    active_closures.extend(for_insert.closures);
                }
            }
            Self::AppendChildren {
                target_node,
                children,
            } => {
                let target_element: &Element = target_node.unchecked_ref();
                for child in children.into_iter() {
                    target_element.append_child(&child.node)?;
                    active_closures.extend(child.closures);
                }
            }

            Self::AddAttributes {
                target_node, attrs, ..
            } => {
                let target_element: &Element = target_node.unchecked_ref();
                let attrs: Vec<&Attribute<MSG>> =
                    attrs.iter().map(|a| a).collect();
                CreatedNode::set_element_attributes(
                    program,
                    active_closures,
                    target_element,
                    &attrs,
                );
            }
            Self::RemoveAttributes {
                target_node, attrs, ..
            } => {
                let target_element: &Element = target_node.unchecked_ref();
                for attr in attrs.iter() {
                    for att_value in attr.value() {
                        match att_value {
                            AttributeValue::Simple(_) => {
                                CreatedNode::remove_element_attribute(
                                    target_element,
                                    attr,
                                )?;
                            }
                            // it is an event listener
                            AttributeValue::EventListener(_) => {
                                remove_event_listener_with_name(
                                    attr.name(),
                                    target_element,
                                    active_closures,
                                )?;
                            }
                            AttributeValue::FunctionCall(_)
                            | AttributeValue::Style(_)
                            | AttributeValue::Empty => (),
                        }
                    }
                }
            }

            // This also removes the associated closures and event listeners to the node being replaced
            // including the associated closures of the descendant of replaced node
            // before it is actully replaced in the DOM
            //
            Self::ReplaceNode {
                target_node,
                replacement,
            } => {
                let target_element: &Element = target_node.unchecked_ref();
                // FIXME: performance bottleneck here
                // Each element and it's descendant is created. Each call to dom to create the element
                // has a cost of ~1ms due to bindings in wasm-bindgen, multiple call of 1000 elements can accumulate to 1s time.
                //
                // Possible fix: stringify and process the patch in plain javascript code.
                // That way, all the code is done at once.
                if target_element.node_type() == Node::ELEMENT_NODE {
                    remove_event_listeners(target_element, active_closures)?;
                }
                target_element
                    .replace_with_with_node_1(&replacement.node)
                    .expect("must replace node");

                /*
                // if what we are replacing is a root node:
                // we replace the root node here, so that's reference is updated
                // to the newly created node
                if patch_path.path.is_empty() {
                    *root_node = replacement.node;
                    #[cfg(feature = "with-debug")]
                    log::info!(
                        "the root_node is replaced with {:?}",
                        root_node
                    );
                }
                */
                active_closures.extend(replacement.closures);
            }
            Self::RemoveNode { target_node } => {
                let target_element: &Element = target_node.unchecked_ref();
                let parent_target = target_element
                    .parent_node()
                    .expect("must have a parent node");
                parent_target
                    .remove_child(target_element)
                    .expect("must remove target node");
                if target_element.node_type() == Node::ELEMENT_NODE {
                    remove_event_listeners(target_element, active_closures)?;
                }
            }
        }
        Ok(())
    }
}
