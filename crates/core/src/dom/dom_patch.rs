use std::{cell::RefCell, rc::Rc};

use indexmap::IndexMap;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsValue;

use crate::{
    dom::{
        self, dom_node, dom_node::DomInner, Application, DomAttr, DomAttrValue, DomNode, Program,
    },
    vdom::{
        Attribute, AttributeValue, ComponentEventCallback, EventCallback, Patch, PatchType,
        TreePath,
    },
};

/// a Patch where the virtual nodes are all created in the document.
/// This is necessary since the created Node  doesn't contain references
/// as opposed to Patch which contains reference to the vdom, which makes it hard
/// to be included in a struct
#[derive(Debug)]
pub struct DomPatch {
    /// The path to traverse to get to the target_element
    pub patch_path: TreePath,
    /// the target node
    pub target_element: DomNode,
    /// the parent element of the target node
    pub target_parent: DomNode,
    /// the patch variant
    pub patch_variant: PatchVariant,
}

/// patch variant
#[derive(Debug)]
pub enum PatchVariant {
    /// Insert nodes before the target node
    InsertBeforeNode {
        /// nodes to be inserted before the target node
        nodes: Vec<DomNode>,
    },
    /// Insert nodes after the target node
    InsertAfterNode {
        /// the nodes to be inserted after the target node
        nodes: Vec<DomNode>,
    },
    /// Append nodes into the target node
    AppendChildren {
        /// the children nodes to be appended into the target node
        children: Vec<DomNode>,
    },
    /// Add attributes to the target node
    AddAttributes {
        /// the attributes to be added to the target node
        attrs: Vec<DomAttr>,
    },
    /// Remove attributes from the target node
    RemoveAttributes {
        /// the attributes names to be removed
        attrs: Vec<DomAttr>,
    },
    /// Replace the target node with the replacement node
    ReplaceNode {
        /// the replacement node
        replacement: Vec<DomNode>,
    },
    /// Remove the target node
    RemoveNode,
    /// Clear the children of the target node
    ClearChildren,
    /// Move the target node before the node specified in the path location
    MoveBeforeNode {
        /// before the node at this location
        for_moving: Vec<DomNode>,
    },
    /// Move the target node after the node specified in the path location
    MoveAfterNode {
        /// after the node at this location
        for_moving: Vec<DomNode>,
    },
}

impl DomNode {
    pub(crate) fn find_node(&self, path: &mut TreePath) -> Option<DomNode> {
        match &self.inner {
            DomInner::StatefulComponent { .. } => {
                log::info!(
                    "This is a stateful component, should return the element
                inside relative to the child container at this path: {:?}",
                    path
                );
                // just return self and handle its own patches
                Some(self.clone())
            }
            _ => {
                if path.is_empty() {
                    Some(self.clone())
                } else {
                    let idx = path.remove_first();
                    if let Some(children) = self.children() {
                        if let Some(child) = children.get(idx) {
                            child.find_node(path)
                        } else {
                            log::warn!("There is no child at index: {idx}");
                            None
                        }
                    } else {
                        log::warn!("Traversing to a childless node..");
                        None
                    }
                }
            }
        }
    }

    pub(crate) fn find_all_nodes(
        &self,
        nodes_to_find: &[(&TreePath, Option<&&'static str>)],
    ) -> IndexMap<TreePath, (DomNode, DomNode)> {
        let mut nodes_to_patch = IndexMap::with_capacity(nodes_to_find.len());
        for (path, tag) in nodes_to_find {
            let mut traverse_path: TreePath = (*path).clone();
            if let Some(found) = self.find_node(&mut traverse_path) {
                let mut parent_path = path.backtrack();
                let target_parent = self
                    .find_node(&mut parent_path)
                    .expect("must find the parent");
                nodes_to_patch.insert((*path).clone(), (found, target_parent));
            } else {
                log::warn!(
                    "can not find: {:?} {:?} target_node: {:?}",
                    path,
                    tag,
                    &self
                );
                log::info!(
                    "real entire dom: {:#?}",
                    dom_node::render_real_dom_to_string(&self.as_node())
                );
                log::warn!("entire dom: {}", self.render_to_string());
            }
        }
        nodes_to_patch
    }
}

impl<APP> Program<APP>
where
    APP: Application + 'static,
{
    /// get the real DOM target node and make a DomPatch object for each of the Patch
    pub(crate) fn convert_patches(
        &self,
        target_node: &DomNode,
        patches: &[Patch<APP::MSG>],
    ) -> Result<Vec<DomPatch>, JsValue> {
        convert_patches(target_node, patches, self.create_ev_callback())
    }

    /// convert a virtual DOM Patch into a created DOM node Patch
    pub fn convert_patch(
        &self,
        nodes_lookup: &IndexMap<TreePath, (DomNode, DomNode)>,
        target_element: &DomNode,
        target_parent: &DomNode,
        patch: &Patch<APP::MSG>,
    ) -> DomPatch {
        convert_patch(
            nodes_lookup,
            target_element,
            target_parent,
            patch,
            self.create_ev_callback(),
        )
    }
}

/// get the real DOM target node and make a DomPatch object for each of the Patch
pub fn convert_patches<Msg, F>(
    target_node: &DomNode,
    patches: &[Patch<Msg>],
    ev_callback: F,
) -> Result<Vec<DomPatch>, JsValue>
where
    Msg: 'static,
    F: Fn(Msg) + 'static + Clone,
{
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

    let nodes_lookup = target_node.find_all_nodes(&nodes_to_find);

    let dom_patches:Vec<DomPatch> = patches.iter().map(|patch|{
        let patch_path = patch.path();
        let patch_tag = patch.tag();
        if let Some((target_node, target_parent)) = nodes_lookup.get(patch_path) {
            let target_tag = target_node.tag();
            if let (Some(patch_tag), Some(target_tag)) = (patch_tag, target_tag) {
                if **patch_tag != target_tag{
                    panic!(
                        "expecting a tag: {patch_tag:?}, but found: {target_tag:?}"
                    );
                }
            }
            convert_patch(&nodes_lookup, target_node, target_parent, patch, ev_callback.clone())
        } else {
            unreachable!("Getting here means we didn't find the element of next node that we are supposed to patch, patch_path: {:?}, with tag: {:?}", patch_path, patch_tag);
        }
    }).collect();

    Ok(dom_patches)
}

/// convert a virtual DOM Patch into a created DOM node Patch
pub fn convert_patch<Msg, F>(
    nodes_lookup: &IndexMap<TreePath, (DomNode, DomNode)>,
    target_element: &DomNode,
    target_parent: &DomNode,
    patch: &Patch<Msg>,
    ev_callback: F,
) -> DomPatch
where
    Msg: 'static,
    F: Fn(Msg) + 'static + Clone,
{
    let target_element = target_element.clone();
    let target_parent = target_parent.clone();
    let Patch {
        patch_path,
        patch_type,
        ..
    } = patch;

    let patch_path = patch_path.clone();

    match patch_type {
        PatchType::InsertBeforeNode { nodes } => {
            let nodes = nodes
                .iter()
                .map(|for_insert| dom::create_dom_node(for_insert, ev_callback.clone()))
                .collect();
            DomPatch {
                patch_path,
                target_element,
                target_parent,
                patch_variant: PatchVariant::InsertBeforeNode { nodes },
            }
        }
        PatchType::InsertAfterNode { nodes } => {
            let nodes = nodes
                .iter()
                .map(|for_insert| dom::create_dom_node(for_insert, ev_callback.clone()))
                .collect();
            DomPatch {
                patch_path,
                target_element,
                target_parent,
                patch_variant: PatchVariant::InsertAfterNode { nodes },
            }
        }

        PatchType::AddAttributes { attrs } => {
            // we merge the attributes here prior to conversion
            let attrs = Attribute::merge_attributes_of_same_name(attrs.iter().copied());
            DomPatch {
                patch_path,
                target_element,
                target_parent,
                patch_variant: PatchVariant::AddAttributes {
                    attrs: attrs
                        .iter()
                        .map(|a| convert_attr(a, ev_callback.clone()))
                        .collect(),
                },
            }
        }
        PatchType::RemoveAttributes { attrs } => DomPatch {
            patch_path,
            target_element,
            target_parent,
            patch_variant: PatchVariant::RemoveAttributes {
                attrs: attrs
                    .iter()
                    .map(|a| convert_attr(a, ev_callback.clone()))
                    .collect(),
            },
        },

        PatchType::ReplaceNode { replacement } => {
            let replacement = replacement
                .iter()
                .map(|node| dom::create_dom_node(node, ev_callback.clone()))
                .collect();
            DomPatch {
                patch_path,
                target_element,
                target_parent,
                patch_variant: PatchVariant::ReplaceNode { replacement },
            }
        }
        PatchType::RemoveNode => DomPatch {
            patch_path,
            target_element,
            target_parent,
            patch_variant: PatchVariant::RemoveNode,
        },
        PatchType::ClearChildren => DomPatch {
            patch_path,
            target_element,
            target_parent,
            patch_variant: PatchVariant::ClearChildren,
        },
        PatchType::MoveBeforeNode { nodes_path } => {
            let for_moving = nodes_path
                .iter()
                .map(|path| {
                    let (node, _) = nodes_lookup.get(path).expect("must have found the node");
                    node.clone()
                })
                .collect();
            DomPatch {
                patch_path,
                target_element,
                target_parent,
                patch_variant: PatchVariant::MoveBeforeNode { for_moving },
            }
        }
        PatchType::MoveAfterNode { nodes_path } => {
            let for_moving = nodes_path
                .iter()
                .map(|path| {
                    let (node, _) = nodes_lookup.get(path).expect("must have found the node");
                    node.clone()
                })
                .collect();
            DomPatch {
                patch_path,
                target_element,
                target_parent,
                patch_variant: PatchVariant::MoveAfterNode { for_moving },
            }
        }
        PatchType::AppendChildren { children } => {
            let children = children
                .iter()
                .map(|for_insert| dom::create_dom_node(for_insert, ev_callback.clone()))
                .collect();

            DomPatch {
                patch_path,
                target_element,
                target_parent,
                patch_variant: PatchVariant::AppendChildren { children },
            }
        }
    }
}

pub(crate) fn convert_attr<Msg, F>(attr: &Attribute<Msg>, ev_callback: F) -> DomAttr
where
    Msg: 'static,
    F: Fn(Msg) + 'static + Clone,
{
    DomAttr {
        namespace: attr.namespace,
        name: attr.name,
        value: attr
            .value
            .iter()
            .filter_map(|v| convert_attr_value(v, ev_callback.clone()))
            .collect(),
    }
}

fn convert_attr_value<Msg, F>(
    attr_value: &AttributeValue<Msg>,
    ev_callback: F,
) -> Option<DomAttrValue>
where
    Msg: 'static,
    F: Fn(Msg) + 'static,
{
    match attr_value {
        AttributeValue::Simple(v) => Some(DomAttrValue::Simple(v.clone())),
        AttributeValue::Style(v) => Some(DomAttrValue::Style(v.clone())),
        AttributeValue::EventListener(v) => Some(DomAttrValue::EventListener(
            convert_event_listener(v, ev_callback),
        )),
        AttributeValue::ComponentEventListener(v) => Some(DomAttrValue::EventListener(
            convert_component_event_listener(v),
        )),
        AttributeValue::Empty => None,
    }
}

fn convert_event_listener<F, Msg>(
    event_listener: &EventCallback<Msg>,
    callback: F,
) -> Closure<dyn FnMut(web_sys::Event)>
where
    Msg: 'static,
    F: Fn(Msg) + 'static,
{
    let event_listener = event_listener.clone();
    let closure: Closure<dyn FnMut(web_sys::Event)> = Closure::new(move |event: web_sys::Event| {
        let msg = event_listener.emit(dom::Event::from(event));
        callback(msg);
    });
    closure
}

/// TODO: this should not have access to root_node, so it can generically
/// apply patch to any dom node
pub fn apply_dom_patches(
    root_node: Rc<RefCell<Option<DomNode>>>,
    mount_node: Rc<RefCell<Option<DomNode>>>,
    dom_patches: impl IntoIterator<Item = DomPatch>,
) -> Result<(), JsValue> {
    for dom_patch in dom_patches {
        apply_dom_patch(Rc::clone(&root_node), Rc::clone(&mount_node), dom_patch)?;
    }
    Ok(())
}

/// apply a dom patch to this root node,
/// return a new root_node if it would replace the original root_node
/// TODO: this should have no access to root_node, so it can be used in general sense
pub(crate) fn apply_dom_patch(
    root_node: Rc<RefCell<Option<DomNode>>>,
    mount_node: Rc<RefCell<Option<DomNode>>>,
    dom_patch: DomPatch,
) -> Result<(), JsValue> {
    let DomPatch {
        patch_path,
        target_element,
        target_parent,
        patch_variant,
    } = dom_patch;

    match patch_variant {
        PatchVariant::InsertBeforeNode { nodes } => {
            target_parent.insert_before(&target_element, nodes);
        }

        PatchVariant::InsertAfterNode { nodes } => {
            target_parent.insert_after(&target_element, nodes);
        }
        PatchVariant::AppendChildren { children } => {
            target_element.append_children(children);
        }

        PatchVariant::AddAttributes { attrs } => {
            target_element.set_dom_attrs(attrs).unwrap();
        }
        PatchVariant::RemoveAttributes { attrs } => {
            for attr in attrs.iter() {
                for att_value in attr.value.iter() {
                    match att_value {
                        DomAttrValue::Simple(_) => {
                            target_element.remove_dom_attr(attr)?;
                        }
                        // it is an event listener
                        DomAttrValue::EventListener(_) => {
                            let DomInner::Element { listeners, .. } = &target_element.inner else {
                                unreachable!("must be an element");
                            };
                            if let Some(listener) = listeners.borrow_mut().as_mut() {
                                listener.retain(|event, _| *event != attr.name)
                            }
                        }
                        DomAttrValue::Style(_) => {
                            target_element.remove_dom_attr(attr)?;
                        }
                        DomAttrValue::Empty => (),
                    }
                }
            }
        }

        // This also removes the associated closures and event listeners to the node being replaced
        // including the associated closures of the descendant of replaced node
        // before it is actully replaced in the DOM
        // TODO: make root node a Vec
        PatchVariant::ReplaceNode { mut replacement } => {
            let first_node = replacement.remove(0);

            if target_element.is_fragment() {
                assert!(
                    patch_path.is_empty(),
                    "this should only happen to root node"
                );
                let mut mount_node = mount_node.borrow_mut();
                let mount_node = mount_node.as_mut().expect("must have a mount node");
                mount_node.append_children(vec![first_node.clone()]);
                mount_node.append_children(replacement);
            } else {
                if patch_path.path.is_empty() {
                    let mut mount_node = mount_node.borrow_mut();
                    let mount_node = mount_node.as_mut().expect("must have a mount node");
                    mount_node.replace_child(&target_element, first_node.clone());
                } else {
                    target_parent.replace_child(&target_element, first_node.clone());
                }
                //insert the rest
                target_parent.insert_after(&first_node, replacement);
            }
            if patch_path.path.is_empty() {
                *root_node.borrow_mut() = Some(first_node);
            }
        }
        PatchVariant::RemoveNode => {
            target_parent.remove_children(&[&target_element]);
        }
        PatchVariant::ClearChildren => {
            target_element.clear_children();
        }
        PatchVariant::MoveBeforeNode { for_moving } => {
            target_parent.remove_children(&for_moving.iter().collect::<Vec<_>>());
            target_parent.insert_before(&target_element, for_moving);
        }

        PatchVariant::MoveAfterNode { for_moving } => {
            target_parent.remove_children(&for_moving.iter().collect::<Vec<_>>());
            target_parent.insert_after(&target_element, for_moving);
        }
    }
    Ok(())
}

fn convert_component_event_listener(
    component_callback: &ComponentEventCallback,
) -> Closure<dyn FnMut(web_sys::Event)> {
    let component_callback = component_callback.clone();
    let closure: Closure<dyn FnMut(web_sys::Event)> = Closure::new(move |event: web_sys::Event| {
        component_callback.emit(dom::Event::from(event));
    });
    closure
}
