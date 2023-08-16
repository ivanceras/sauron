use crate::dom::dom_node::find_all_nodes;
use crate::dom::dom_node::intern;
use crate::dom::{Application, Program};
use crate::vdom::{Attribute, AttributeValue, Patch, PatchType};
use mt_dom::TreePath;
use std::collections::BTreeMap;
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
        for_moving: Vec<Node>,
    },
    /// Move the target node after the node specified in the path location
    MoveAfterNode {
        /// after the node at this location
        for_moving: Vec<Node>,
    },
}

impl<APP, MSG> Program<APP, MSG>
where
    MSG: 'static,
    APP: Application<MSG> + 'static,
{
    /// get the real DOM target node and make a DomPatch object for each of the Patch
    pub(crate) fn convert_patches(
        &self,
        patches: &[Patch<MSG>],
    ) -> Result<Vec<DomPatch<MSG>>, JsValue> {
        let nodes_to_find: Vec<(&TreePath, Option<&&'static str>)> = patches
            .iter()
            .map(|patch| (patch.path(), patch.tag()))
            .chain(
                patches
                    .iter()
                    .flat_map(|patch| patch.node_paths())
                    .map(|path| (path, None)),
            )
            .collect();

        let nodes_lookup = find_all_nodes(
            self.root_node
                .borrow()
                .as_ref()
                .expect("must have a root node"),
            &nodes_to_find,
        );

        let dom_patches:Vec<DomPatch<MSG>> = patches.iter().map(|patch|{
            let patch_path = patch.path();
            let patch_tag = patch.tag();
            if let Some(target_node) = nodes_lookup.get(patch_path) {
                let target_element: &Element = target_node.unchecked_ref();
                if let Some(tag) = patch_tag {
                    let target_tag = target_element.tag_name().to_lowercase();
                    if target_tag != **tag {
                        panic!(
                            "expecting a tag: {tag:?}, but found: {target_tag:?}"
                        );
                    }
                }
                self.convert_patch(&nodes_lookup, target_element, patch)
            } else {
                unreachable!("Getting here means we didn't find the element of next node that we are supposed to patch, patch_path: {:?}, with tag: {:?}", patch_path, patch_tag);
            }
        }).collect();

        Ok(dom_patches)
    }
    /// convert a virtual DOM Patch into a created DOM node Patch
    pub fn convert_patch(
        &self,
        nodes_lookup: &BTreeMap<TreePath, Node>,
        target_element: &Element,
        patch: &Patch<MSG>,
    ) -> DomPatch<MSG> {
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
            PatchType::MoveBeforeNode { nodes_path } => {
                let for_moving: Vec<Node> = nodes_path
                    .iter()
                    .map(|path| {
                        nodes_lookup
                            .get(path)
                            .expect("must have found the node")
                            .clone()
                    })
                    .collect();
                DomPatch {
                    patch_path,
                    target_element,
                    patch_variant: PatchVariant::MoveBeforeNode { for_moving },
                }
            }
            PatchType::MoveAfterNode { nodes_path } => {
                let for_moving: Vec<Node> = nodes_path
                    .iter()
                    .map(|path| {
                        nodes_lookup
                            .get(path)
                            .expect("must have found the node")
                            .clone()
                    })
                    .collect();
                DomPatch {
                    patch_path,
                    target_element,
                    patch_variant: PatchVariant::MoveAfterNode { for_moving },
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

    pub(crate) fn apply_dom_patch(&mut self, dom_patch: DomPatch<MSG>) -> Result<(), JsValue> {
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
                let first_node = replacement.pop().expect("must have a first node");
                if target_element.node_type() == Node::DOCUMENT_FRAGMENT_NODE {
                    // if we are patching a fragment mode in the top-level document
                    // it has no access to it's parent other than accessing the mount-node itself
                    if patch_path.is_empty() {
                        let mount_node = self.mount_node();
                        Self::clear_children(&mount_node);
                        mount_node
                            .append_child(&first_node)
                            .expect("must append child");
                        Self::dispatch_mount_event(&first_node);

                        for node in replacement.into_iter() {
                            let node_elm: &web_sys::Element = node.unchecked_ref();
                            mount_node.append_child(node_elm).expect("append child");
                            Self::dispatch_mount_event(node_elm);
                        }
                    } else {
                        // the diffing algorithmn doesn't concern with fragment, instead it test the nodes contain in the fragment as if it where a list of nodes
                        unreachable!("patching a document fragment other than the root_node should not happen");
                    }
                } else {
                    if target_element.node_type() == Node::ELEMENT_NODE {
                        self.remove_event_listeners(&target_element)?;
                    }
                    //let first_node = replacement.pop().expect("must have a first node");
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
            PatchVariant::MoveBeforeNode { for_moving } => {
                if let Some(target_parent) = target_element.parent_node() {
                    for move_node in for_moving {
                        let move_node_parent = move_node
                            .parent_node()
                            .expect("node for moving must have parent");
                        let move_node = move_node_parent
                            .remove_child(&move_node)
                            .expect("must remove child");
                        target_parent
                            .insert_before(&move_node, Some(&target_element))
                            .expect("must insert before this node");
                    }
                } else {
                    panic!("unable to get the parent node of the target element");
                }
            }

            PatchVariant::MoveAfterNode { for_moving } => {
                for move_node in for_moving {
                    let move_node_parent = move_node
                        .parent_node()
                        .expect("node for moving must have parent");
                    let to_move_node = move_node_parent
                        .remove_child(&move_node)
                        .expect("must remove child");

                    let to_move_element: &web_sys::Element =
                        to_move_node.dyn_ref().expect("an element");
                    target_element
                        .insert_adjacent_element(intern("afterend"), to_move_element)
                        .expect("must insert before this node");
                }
            }
        }
        Ok(())
    }
}
