//! provides functionalities related to patching the DOM in the browser.
use crate::{
    dom::Dispatch,
    dom::{
        created_node,
        created_node::{ActiveClosure, CreatedNode},
    },
    html::attributes::AttributeValue,
    vdom::Patch,
};
use js_sys::Function;
use mt_dom::TreePath;
use std::collections::BTreeMap;
use std::collections::HashMap;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Element, Node};

/// Apply all of the patches to our old root node in order to create the new root node
/// that we desire.
/// This is usually used after diffing two virtual nodes.
///
pub fn patch<DSP, MSG>(
    program: &DSP,
    root_node: &mut Node,
    old_closures: &mut ActiveClosure,
    focused_node: &mut Option<Node>,
    patches: Vec<Patch<MSG>>,
) -> Result<ActiveClosure, JsValue>
where
    MSG: 'static,
    DSP: Clone + Dispatch<MSG> + 'static,
{
    let nodes_to_find: Vec<(&TreePath, Option<&&'static str>)> = patches
        .iter()
        .map(|patch| (patch.path(), patch.tag()))
        .collect();

    let mut paths = vec![];
    for patch in patches.iter() {
        paths.push(patch.path());
    }

    let mut active_closures = HashMap::new();
    let nodes_to_patch = find_all_nodes(root_node, &nodes_to_find);

    for patch in patches.iter() {
        let patch_path = patch.path();
        if let Some(element) = nodes_to_patch.get(patch_path) {
            let new_closures = apply_patch_to_node(
                program,
                root_node,
                element,
                old_closures,
                focused_node,
                patch,
            )?;
            active_closures.extend(new_closures);
        } else {
            unreachable!("Getting here means we didn't find the element of next node that we are supposed to patch, patch_path: {:?}", patch_path);
        }
    }

    Ok(active_closures)
}

fn find_node(node: &Node, path: &mut TreePath) -> Option<Node> {
    if path.is_empty() {
        Some(node.clone())
    } else {
        let idx = path.remove_first();
        let children = node.child_nodes();
        if let Some(child) = children.item(idx as u32) {
            find_node(&child, path)
        } else {
            None
        }
    }
}

fn find_all_nodes(
    node: &Node,
    nodes_to_find: &[(&TreePath, Option<&&'static str>)],
) -> BTreeMap<TreePath, Node> {
    let mut nodes_to_patch: BTreeMap<TreePath, Node> = BTreeMap::new();

    for (path, tag) in nodes_to_find {
        let mut traverse_path: TreePath = (*path).clone();
        if let Some(found) = find_node(node, &mut traverse_path) {
            nodes_to_patch.insert((*path).clone(), found);
        } else {
            log::warn!(
                "can not find: {:?} {:?} root_node: {:?}",
                path,
                tag,
                node
            );
        }
    }
    nodes_to_patch
}

/// Get the "data-sauron-vdom-id" of all the desendent of this node including itself
/// This is needed to free-up the closure that was attached ActiveClosure manually
fn get_node_descendant_data_vdom_id(root_element: &Element) -> Vec<usize> {
    let mut data_vdom_id = vec![];

    // TODO: there should be a better way to get the node-id back
    // without having to read from the actual dom node element
    if let Some(vdom_id_str) =
        root_element.get_attribute(created_node::DATA_VDOM_ID)
    {
        let vdom_id = vdom_id_str
            .parse::<usize>()
            .expect("unable to parse sauron_vdom-id");
        data_vdom_id.push(vdom_id);
    }

    let children = root_element.child_nodes();
    let child_node_count = children.length();
    for i in 0..child_node_count {
        let child_node = children.item(i).expect("Expecting a child node");
        if child_node.node_type() == Node::ELEMENT_NODE {
            let child_element = child_node.unchecked_ref::<Element>();
            let child_data_vdom_id =
                get_node_descendant_data_vdom_id(child_element);
            data_vdom_id.extend(child_data_vdom_id);
        }
    }
    data_vdom_id
}

/// remove all the event listeners for this node
fn remove_event_listeners(
    node: &Element,
    old_closures: &mut ActiveClosure,
) -> Result<(), JsValue> {
    let all_descendant_vdom_id = get_node_descendant_data_vdom_id(node);
    for vdom_id in all_descendant_vdom_id {
        if let Some(old_closure) = old_closures.get(&vdom_id) {
            for (event, oc) in old_closure.iter() {
                let func: &Function = oc.as_ref().unchecked_ref();
                node.remove_event_listener_with_callback(event, func)?;
            }

            // remove closure active_closure in dom_updater to free up memory
            old_closures
                .remove(&vdom_id)
                .expect("Unable to remove old closure");
        } else {
            log::warn!(
                "There is no closure marked with that vdom_id: {}",
                vdom_id
            );
        }
    }
    Ok(())
}

/// remove the event listener which matches the given event name
fn remove_event_listener_with_name(
    event_name: &'static str,
    node: &Element,
    old_closures: &mut ActiveClosure,
) -> Result<(), JsValue> {
    let all_descendant_vdom_id = get_node_descendant_data_vdom_id(node);
    for vdom_id in all_descendant_vdom_id {
        if let Some(old_closure) = old_closures.get_mut(&vdom_id) {
            for (event, oc) in old_closure.iter() {
                if *event == event_name {
                    let func: &Function = oc.as_ref().unchecked_ref();
                    node.remove_event_listener_with_callback(event, func)?;
                }
            }

            old_closure.retain(|(event, _oc)| *event != event_name);

            // remove closure active_closure in dom_updater to free up memory
            if old_closure.is_empty() {
                old_closures
                    .remove(&vdom_id)
                    .expect("Unable to remove old closure");
            }
        } else {
            log::warn!(
                "There is no closure marked with that vdom_id: {}",
                vdom_id
            );
        }
    }
    Ok(())
}

/// apply a the patch to this element node.
/// and return the ActiveClosure that may be attached to that element
///
/// Note: a mutable root_node is passed here
/// for the sole purpose of setting it when the a patch ReplaceNode at 0 is encountered.
#[track_caller]
fn apply_patch_to_node<DSP, MSG>(
    program: &DSP,
    root_node: &mut Node,
    node: &Node,
    old_closures: &mut ActiveClosure,
    focused_node: &mut Option<Node>,
    patch: &Patch<MSG>,
) -> Result<ActiveClosure, JsValue>
where
    MSG: 'static,
    DSP: Clone + Dispatch<MSG> + 'static,
{
    let mut active_closures = ActiveClosure::new();

    match patch {
        Patch::InsertBeforeNode {
            tag,
            patch_path,
            nodes: for_inserts,
        } => {
            // we inser the node before this target element
            let target_element: &Element = node.unchecked_ref();
            if let Some(parent_node) = target_element.parent_node() {
                if let Some(tag) = tag {
                    let target_tag = target_element.tag_name().to_lowercase();
                    if target_tag != **tag {
                        panic!(
                            "expecting a tag: {:?}, but found: {:?}",
                            tag, target_tag
                        );
                    }
                }

                for for_insert in for_inserts {
                    let created_node = CreatedNode::create_dom_node::<DSP, MSG>(
                        program,
                        for_insert,
                        focused_node,
                    );
                    parent_node
                        .insert_before(&created_node.node, Some(target_element))
                        .expect("must remove target node");

                    active_closures.extend(created_node.closures);
                }
            } else {
                panic!("unable to get parent node of the target element: {:?} thas has a tag: {:?} in path: {:?}, for patching: {:#?}", target_element, tag, patch_path, for_inserts);
            }

            Ok(active_closures)
        }

        Patch::InsertAfterNode {
            tag,
            patch_path: _,
            nodes: for_inserts,
        } => {
            // we inser the node before this target element
            let target_element: &Element = node.unchecked_ref();
            if let Some(tag) = tag {
                let target_tag = target_element.tag_name().to_lowercase();
                if target_tag != **tag {
                    panic!(
                        "expecting a tag: {:?}, but found: {:?}",
                        tag, target_tag
                    );
                }
            }

            for for_insert in for_inserts.iter().rev() {
                let created_node = CreatedNode::create_dom_node::<DSP, MSG>(
                    program,
                    for_insert,
                    focused_node,
                );
                let created_element: &Element = created_node
                    .node
                    .dyn_ref()
                    .expect("only elements is supported for now");
                target_element
                    .insert_adjacent_element("afterend", created_element)
                    .expect("must remove target node");
                active_closures.extend(created_node.closures);
            }
            Ok(active_closures)
        }

        Patch::AddAttributes { attrs, .. } => {
            let element: &Element = node.unchecked_ref();
            CreatedNode::set_element_attributes(
                program,
                &mut active_closures,
                element,
                attrs,
            );

            Ok(active_closures)
        }
        Patch::RemoveAttributes { attrs, .. } => {
            let element: &Element = node.unchecked_ref();
            for attr in attrs.iter() {
                for att_value in attr.value() {
                    match att_value {
                        AttributeValue::Simple(_) => {
                            CreatedNode::remove_element_attribute(
                                element, attr,
                            )?;
                        }
                        // it is an event listener
                        AttributeValue::EventListener(_) => {
                            remove_event_listener_with_name(
                                attr.name(),
                                element,
                                old_closures,
                            )?;
                        }
                        AttributeValue::FunctionCall(_)
                        | AttributeValue::Style(_)
                        | AttributeValue::Empty => (),
                    }
                }
            }
            Ok(active_closures)
        }

        // This also removes the associated closures and event listeners to the node being replaced
        // including the associated closures of the descendant of replaced node
        // before it is actully replaced in the DOM
        //
        Patch::ReplaceNode {
            tag: _,
            patch_path,
            replacement,
        } => {
            let element: &Element = node.unchecked_ref();
            // FIXME: performance bottleneck here
            // Each element and it's descendant is created. Each call to dom to create the element
            // has a cost of ~1ms due to bindings in wasm-bindgen, multiple call of 1000 elements can accumulate to 1s time.
            //
            // Possible fix: stringify and process the patch in plain javascript code.
            // That way, all the code is done at once.
            let created_node = CreatedNode::create_dom_node::<DSP, MSG>(
                program,
                replacement,
                focused_node,
            );
            if element.node_type() == Node::ELEMENT_NODE {
                remove_event_listeners(element, old_closures)?;
            }
            element
                .replace_with_with_node_1(&created_node.node)
                .expect("must replace node");

            // if what we are replacing is a root node:
            // we replace the root node here, so that's reference is updated
            // to the newly created node
            if patch_path.path.is_empty() {
                *root_node = created_node.node;
                #[cfg(feature = "with-debug")]
                log::info!("the root_node is replaced with {:?}", root_node);
            }
            active_closures.extend(created_node.closures);
            Ok(active_closures)
        }
        Patch::RemoveNode { .. } => {
            let element: &Element = node.unchecked_ref();
            let parent_node =
                element.parent_node().expect("must have a parent node");
            parent_node
                .remove_child(element)
                .expect("must remove target node");
            if element.node_type() == Node::ELEMENT_NODE {
                let element: &Element = node.unchecked_ref();
                remove_event_listeners(element, old_closures)?;
            }
            Ok(active_closures)
        }
        Patch::AppendChildren {
            tag: _,
            patch_path: _,
            children: new_nodes,
        } => {
            let element: &Element = node.unchecked_ref();
            let mut active_closures = HashMap::new();
            for new_node in new_nodes.iter() {
                let created_node = CreatedNode::create_dom_node::<DSP, MSG>(
                    program,
                    new_node,
                    focused_node,
                );
                element.append_child(&created_node.node)?;
                active_closures.extend(created_node.closures);
            }
            Ok(active_closures)
        }
    }
}
