use crate::dom::dom_node::find_node;
use crate::dom::dom_node::intern;
use crate::dom::{Application, Program};
use crate::vdom::{Attribute, AttributeValue, Patch, PatchType};
use mt_dom::TreePath;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::Element;
use web_sys::Node;

/// a Patch where the virtual nodes are all created in the document.
/// This is necessary since the created Node  doesn't contain references
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
        nodes: Vec<Node>,
    },
    /// Insert nodes after the target node
    InsertAfterNode {
        /// the nodes to be inserted after the target node
        nodes: Vec<Node>,
    },
    /// Append nodes into the target node
    AppendChildren {
        /// the children nodes to be appended into the target node
        children: Vec<Node>,
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
        replacement: Vec<Node>,
    },
    /// Remove the target node
    RemoveNode,
    /// Move the target node before the node specified in the path location
    MoveBeforeNode {
        /// before the node at this location
        node: Node,
    },
    /// Move the target node after the node specified in the path location
    MoveAfterNode {
        /// after the node at this location
        node: Node,
    },
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
                let nodes: Vec<Node> = nodes
                    .iter()
                    .map(|for_insert| self.create_dom_node(for_insert))
                    .collect();
                DomPatch {
                    patch_path,
                    target_element,
                    patch_variant: PatchVariant::InsertBeforeNode { nodes },
                }
            }
            PatchType::InsertAfterNode { nodes } => {
                let nodes: Vec<Node> = nodes
                    .iter()
                    .map(|for_insert| self.create_dom_node(for_insert))
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
                let replacement: Vec<Node> = replacement
                    .iter()
                    .map(|node| self.create_dom_node(node))
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
            PatchType::MoveBeforeNode { path } => {
                let mut path = path.clone();
                let node = find_node(
                    self.root_node
                        .borrow()
                        .as_ref()
                        .expect("must have a root node"),
                    &mut path,
                )
                .expect("must find the node");
                DomPatch {
                    patch_path,
                    target_element,
                    patch_variant: PatchVariant::MoveBeforeNode { node },
                }
            }
            PatchType::MoveAfterNode { path } => {
                let mut path = path.clone();
                let node = find_node(
                    self.root_node
                        .borrow()
                        .as_ref()
                        .expect("must have a root node"),
                    &mut path,
                )
                .expect("must find the node");
                DomPatch {
                    patch_path,
                    target_element,
                    patch_variant: PatchVariant::MoveAfterNode { node },
                }
            }
            PatchType::AppendChildren { children } => {
                let children: Vec<Node> = children
                    .iter()
                    .map(|for_insert| self.create_dom_node(for_insert))
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
                            .insert_before(&for_insert, Some(&target_element))
                            .expect("must remove target node");
                        Self::dispatch_mount_event(&for_insert);
                    }
                } else {
                    panic!("unable to get parent node of the target element: {target_element:?} for patching: {nodes:#?}");
                }
            }

            PatchVariant::InsertAfterNode { nodes } => {
                // we insert the node before this target element
                for for_insert in nodes.into_iter().rev() {
                    let created_element: &Element = for_insert
                        .dyn_ref()
                        .expect("only elements is supported for now");
                    target_element
                        .insert_adjacent_element(intern("afterend"), created_element)
                        .expect("must insert after the target element");
                    Self::dispatch_mount_event(&for_insert);
                }
            }
            PatchVariant::AppendChildren { children } => {
                for child in children.into_iter() {
                    Self::append_child_and_dispatch_mount_event(
                        target_element.unchecked_ref(),
                        &child,
                    );
                }
            }

            PatchVariant::AddAttributes { attrs } => {
                let attrs: Vec<&Attribute<MSG>> = attrs.iter().collect();
                self.set_element_attributes(&target_element, &attrs);
            }
            PatchVariant::RemoveAttributes { attrs } => {
                for attr in attrs.iter() {
                    for att_value in attr.value() {
                        match att_value {
                            AttributeValue::Simple(_) => {
                                Self::remove_element_attribute(&target_element, attr)?;
                            }
                            // it is an event listener
                            AttributeValue::EventListener(_) => {
                                self.remove_event_listener_with_name(attr.name(), &target_element)?;
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
                    self.remove_event_listeners(&target_element)?;
                }
                let first_node = replacement.pop().expect("must have a first node");
                target_element
                    .replace_with_with_node_1(&first_node)
                    .unwrap_or_else(|e| {
                        panic!("unable to replace node with {first_node:?}, {e:?}");
                    });

                Self::dispatch_mount_event(&first_node);

                let first_node_elm: &web_sys::Element = first_node.unchecked_ref();

                for node in replacement.into_iter() {
                    let node_elm: &web_sys::Element = node.unchecked_ref();
                    first_node_elm
                        .insert_adjacent_element(intern("beforebegin"), node_elm)
                        .expect("append child");
                    Self::dispatch_mount_event(node_elm);
                }

                //Note: it is important that root_node points to the original mutable reference here
                // since it can be replaced with a new root Node(the top-level node of the view) when patching
                // if what we are replacing is a root node:
                // we replace the root node here, so that's reference is updated
                // to the newly created node
                if patch_path.path.is_empty() {
                    *self.root_node.borrow_mut() = Some(first_node);
                    #[cfg(feature = "with-debug")]
                    log::info!("the root_node is replaced with {:?}", &self.root_node);
                }
            }
            PatchVariant::RemoveNode => {
                let parent_target = target_element
                    .parent_node()
                    .expect("must have a parent node");
                parent_target
                    .remove_child(&target_element)
                    .expect("must remove target node");
                if target_element.node_type() == Node::ELEMENT_NODE {
                    self.remove_event_listeners(&target_element)?;
                }
            }
            PatchVariant::MoveBeforeNode { node: before_node } => {
                if let Some(parent_target) = target_element.parent_node() {
                    let before_node_parent =
                        before_node.parent_node().expect("must have a parent node");

                    let target_element = parent_target
                        .remove_child(&target_element)
                        .expect("must return the removed element");

                    before_node_parent
                        .insert_before(&target_element, Some(&before_node))
                        .expect("must insert before this node");
                } else {
                    panic!("unable to get the parent node of the target element");
                }
            }

            PatchVariant::MoveAfterNode { node: after_node } => {
                if let Some(parent_target) = target_element.parent_node() {
                    let node_for_moving = parent_target
                        .remove_child(&target_element)
                        .expect("must return the removed element");
                    let element_for_moving: &web_sys::Element =
                        node_for_moving.dyn_ref().expect("an element");
                    let after_element: &web_sys::Element =
                        after_node.dyn_ref().expect("an element");
                    after_element
                        .insert_adjacent_element(intern("afterend"), &element_for_moving)
                        .expect("must insert before this node");
                } else {
                    panic!("unable to get the parent node of the target element");
                }
            }
        }
        Ok(())
    }
}
