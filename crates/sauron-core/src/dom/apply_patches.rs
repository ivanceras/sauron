//! provides functionalities related to patching the DOM in the browser.
use crate::DomPatch;
use crate::{
    dom::{created_node, created_node::ActiveClosure, Dispatch},
    vdom::Patch,
};
use js_sys::Function;
use mt_dom::TreePath;
use std::collections::BTreeMap;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Element, Node};

/// Apply all of the patches to our old root node in order to create the new root node
/// that we desire.
/// This is usually used after diffing two virtual nodes.
///
pub fn patch<DSP, MSG>(
    program: &DSP,
    root_node: &mut Node,
    active_closures: &mut ActiveClosure,
    focused_node: &mut Option<Node>,
    patches: Vec<Patch<MSG>>,
) -> Result<(), JsValue>
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

    let nodes_to_patch = find_all_nodes(root_node, &nodes_to_find);

    //TODO: spawn all the apply patch here to to it asynchronously
    // can be done with Promise.all (https://docs.rs/js-sys/0.3.61/js_sys/struct.Promise.html#method.all)
    for patch in patches.iter() {
        let patch_path = patch.path();
        if let Some(target_node) = nodes_to_patch.get(patch_path) {
            // check the tag here if it matches
            let target_element: &Element = target_node.unchecked_ref();
            if let Some(tag) = patch.tag() {
                let target_tag = target_element.tag_name().to_lowercase();
                if target_tag != **tag {
                    panic!(
                        "expecting a tag: {:?}, but found: {:?}",
                        tag, target_tag
                    );
                }
            }

            let dom_patch =
                DomPatch::from_patch(program, target_node, focused_node, patch);
            dom_patch
                .apply(program, active_closures)
                .expect("must apply the dom patch");
        } else {
            unreachable!("Getting here means we didn't find the element of next node that we are supposed to patch, patch_path: {:?}", patch_path);
        }
    }

    Ok(())
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
pub(crate) fn remove_event_listeners(
    node: &Element,
    active_closures: &mut ActiveClosure,
) -> Result<(), JsValue> {
    let all_descendant_vdom_id = get_node_descendant_data_vdom_id(node);
    for vdom_id in all_descendant_vdom_id {
        if let Some(old_closure) = active_closures.get(&vdom_id) {
            for (event, oc) in old_closure.iter() {
                let func: &Function = oc.as_ref().unchecked_ref();
                node.remove_event_listener_with_callback(event, func)?;
            }

            // remove closure active_closure in dom_updater to free up memory
            active_closures
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
pub(crate) fn remove_event_listener_with_name(
    event_name: &'static str,
    node: &Element,
    active_closures: &mut ActiveClosure,
) -> Result<(), JsValue> {
    let all_descendant_vdom_id = get_node_descendant_data_vdom_id(node);
    for vdom_id in all_descendant_vdom_id {
        if let Some(old_closure) = active_closures.get_mut(&vdom_id) {
            for (event, oc) in old_closure.iter() {
                if *event == event_name {
                    let func: &Function = oc.as_ref().unchecked_ref();
                    node.remove_event_listener_with_callback(event, func)?;
                }
            }

            old_closure.retain(|(event, _oc)| *event != event_name);

            // remove closure active_closure in dom_updater to free up memory
            if old_closure.is_empty() {
                active_closures
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
