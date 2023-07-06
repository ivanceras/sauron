use crate::dom::created_node::intern;
use crate::dom::{Application, CreatedNode, Program};
use crate::vdom::{Attribute, AttributeValue, Patch, PatchType};
use mt_dom::TreePath;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::Element;
use web_sys::Node;

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

impl<APP, MSG> Program<APP, MSG>
where
    MSG: 'static,
    APP: Application<MSG> + 'static,
{
    /// convert a virtual DOM Patch into a created DOM node Patch
    pub fn convert_patch(&self, target_element: &Element, patch: &Patch<MSG>) -> DomPatch<MSG> {
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
                    .map(|for_insert| CreatedNode::create_dom_node::<APP, MSG>(self, for_insert))
                    .collect();
                DomPatch {
                    patch_path,
                    target_element,
                    patch_variant: PatchVariant::InsertBeforeNode { nodes },
                }
            }
            PatchType::InsertAfterNode { nodes } => {
                let nodes: Vec<CreatedNode> = nodes
                    .iter()
                    .map(|for_insert| CreatedNode::create_dom_node::<APP, MSG>(self, for_insert))
                    .collect();
                DomPatch {
                    patch_path,
                    target_element,
                    patch_variant: PatchVariant::InsertAfterNode { nodes },
                }
            }

            PatchType::AddAttributes { attrs } => DomPatch {
                patch_path,
                target_element,
                patch_variant: PatchVariant::AddAttributes {
                    attrs: attrs.iter().map(|a| (*a).clone()).collect(),
                },
            },
            PatchType::RemoveAttributes { attrs } => DomPatch {
                patch_path,
                target_element,
                patch_variant: PatchVariant::RemoveAttributes {
                    attrs: attrs.iter().map(|a| (*a).clone()).collect(),
                },
            },

            PatchType::ReplaceNode { replacement } => {
                let replacement: Vec<CreatedNode> = replacement
                    .iter()
                    .map(|node| CreatedNode::create_dom_node::<APP, MSG>(self, node))
                    .collect();
                DomPatch {
                    patch_path,
                    target_element,
                    patch_variant: PatchVariant::ReplaceNode { replacement },
                }
            }
            PatchType::RemoveNode => DomPatch {
                patch_path,
                target_element,
                patch_variant: PatchVariant::RemoveNode,
            },
            PatchType::AppendChildren { children } => {
                let children: Vec<CreatedNode> = children
                    .iter()
                    .map(|for_insert| CreatedNode::create_dom_node::<APP, MSG>(self, for_insert))
                    .collect();

                DomPatch {
                    patch_path,
                    target_element,
                    patch_variant: PatchVariant::AppendChildren { children },
                }
            }
        }
    }

    pub(crate) fn apply_dom_patch(&self, dom_patch: DomPatch<MSG>) -> Result<(), JsValue> {
        let DomPatch {
            patch_path,
            target_element,
            patch_variant,
        } = dom_patch;

        match patch_variant {
            PatchVariant::InsertBeforeNode { nodes } => {
                // we insert the node before this target element
                if let Some(parent_target) = target_element.parent_node() {
                    for for_insert in nodes {
                        parent_target
                            .insert_before(&for_insert.node, Some(&target_element))
                            .expect("must remove target node");
                        CreatedNode::dispatch_mount_event(&for_insert.node);
                    }
                } else {
                    panic!("unable to get parent node of the target element: {target_element:?} for patching: {nodes:#?}");
                }
            }

            PatchVariant::InsertAfterNode { nodes } => {
                // we insert the node before this target element
                for for_insert in nodes.into_iter().rev() {
                    let created_element: &Element = for_insert
                        .node
                        .dyn_ref()
                        .expect("only elements is supported for now");
                    // trigger a
                    target_element
                        .insert_adjacent_element(intern("afterend"), created_element)
                        .expect("must remove target node");
                    CreatedNode::dispatch_mount_event(&for_insert.node);
                }
            }
            PatchVariant::AppendChildren { children } => {
                for child in children.into_iter() {
                    CreatedNode::append_child_and_dispatch_mount_event(
                        target_element.unchecked_ref(),
                        &child.node,
                    );
                }
            }

            PatchVariant::AddAttributes { attrs } => {
                let attrs: Vec<&Attribute<MSG>> = attrs.iter().collect();
                CreatedNode::set_element_attributes(self, &target_element, &attrs);
            }
            PatchVariant::RemoveAttributes { attrs } => {
                for attr in attrs.iter() {
                    for att_value in attr.value() {
                        match att_value {
                            AttributeValue::Simple(_) => {
                                CreatedNode::remove_element_attribute(&target_element, attr)?;
                            }
                            // it is an event listener
                            AttributeValue::EventListener(_) => {
                                CreatedNode::remove_event_listener_with_name(
                                    self,
                                    attr.name(),
                                    &target_element,
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
            PatchVariant::ReplaceNode { mut replacement } => {
                if target_element.node_type() == Node::ELEMENT_NODE {
                    CreatedNode::remove_event_listeners(self, &target_element)?;
                }
                let first_node = replacement.pop().expect("must have a first node");
                //TODO: make a dispatch_on_dismount event and in the method in created node
                target_element
                    .replace_with_with_node_1(&first_node.node)
                    .expect("must replace node");

                CreatedNode::dispatch_mount_event(&first_node.node);

                let first_node_elm: &web_sys::Element = first_node.node.unchecked_ref();

                for node in replacement.into_iter() {
                    let node_elm: &web_sys::Element = node.node.unchecked_ref();
                    first_node_elm
                        .insert_adjacent_element(intern("beforebegin"), node_elm)
                        .expect("append child");
                    CreatedNode::dispatch_mount_event(node_elm);
                }

                //Note: it is important that root_node points to the original mutable reference here
                // since it can be replaced with a new root Node(the top-level node of the view) when patching
                // if what we are replacing is a root node:
                // we replace the root node here, so that's reference is updated
                // to the newly created node
                if patch_path.path.is_empty() {
                    *self.root_node.borrow_mut() = Some(first_node.node);
                    #[cfg(feature = "with-debug")]
                    log::info!("the root_node is replaced with {:?}", &self.root_node);
                }
            }
            PatchVariant::RemoveNode => {
                let parent_target = target_element
                    .parent_node()
                    .expect("must have a parent node");
                //TODO: trigger a on_dispatch event here
                parent_target
                    .remove_child(&target_element)
                    .expect("must remove target node");
                if target_element.node_type() == Node::ELEMENT_NODE {
                    CreatedNode::remove_event_listeners(self, &target_element)?;
                }
            }
        }
        Ok(())
    }
}
