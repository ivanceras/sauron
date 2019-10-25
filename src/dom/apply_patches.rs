use super::{
    ActiveClosure,
    CreatedNode,
};
use crate::{
    dom,
    Dispatch,
    Patch,
};
use js_sys::Function;
use std::{
    collections::{
        HashMap,
        HashSet,
    },
    rc::Rc,
};
use wasm_bindgen::{
    closure::Closure,
    JsCast,
    JsValue,
};
use web_sys::{
    Element,
    Event,
    Node,
    Text,
};

/// Apply all of the patches to our old root node in order to create the new root node
/// that we desire.
/// This is usually used after diffing two virtual nodes.
pub fn patch<N, DSP, MSG>(
    program: &Rc<DSP>,
    root_node: N,
    old_closures: &mut ActiveClosure,
    patches: &[Patch<MSG>],
) -> Result<ActiveClosure, JsValue>
where
    N: Into<Node>,
    MSG: 'static,
    DSP: Dispatch<MSG> + 'static,
{
    let root_node: Node = root_node.into();

    // Closure that were added to the DOM during this patch operation.
    let mut active_closures = HashMap::new();

    // finding the nodes to be patched before hand, instead of calling it
    // in every patch loop.
    let (element_nodes_to_patch, text_nodes_to_patch) =
        find_nodes(root_node, patches);

    for patch in patches {
        let patch_node_idx = patch.node_idx();

        if let Some(element) = element_nodes_to_patch.get(&patch_node_idx) {
            let new_closures =
                apply_element_patch(program, &element, old_closures, &patch)?;
            active_closures.extend(new_closures);
            continue;
        }

        if let Some(text_node) = text_nodes_to_patch.get(&patch_node_idx) {
            apply_text_patch(program, &text_node, &patch)?;
            continue;
        }

        unreachable!("Getting here means we didn't find the element or next node that we were supposed to patch.")
    }

    Ok(active_closures)
}

/// find the nodes to be patched
/// each patch contains a node index, arranged in depth first tree.
///
/// This function is needed for optimization purposes.
/// Instead of finding the nodes each time in the patching process.
/// We find them before hand so as not to keep calling this function for each and every element to
/// be patched.
fn find_nodes<MSG>(
    root_node: Node,
    patches: &[Patch<MSG>],
) -> (HashMap<usize, Element>, HashMap<usize, Text>) {
    let mut cur_node_idx = 0;
    let mut nodes_to_find = HashSet::new();

    for patch in patches {
        nodes_to_find.insert(patch.node_idx());
    }

    find_nodes_recursive(root_node, &mut cur_node_idx, &nodes_to_find)
}

fn find_nodes_recursive(
    root_node: Node,
    cur_node_idx: &mut usize,
    nodes_to_find: &HashSet<usize>,
) -> (HashMap<usize, Element>, HashMap<usize, Text>) {
    let mut element_nodes_to_patch = HashMap::new();
    let mut text_nodes_to_patch = HashMap::new();

    // We use child_nodes() instead of children() because children() ignores text nodes
    let children = root_node.child_nodes();
    let child_node_count = children.length();

    // If the root node matches, mark it for patching
    if nodes_to_find.get(&cur_node_idx).is_some() {
        match root_node.node_type() {
            Node::ELEMENT_NODE => {
                element_nodes_to_patch
                    .insert(*cur_node_idx, root_node.unchecked_into());
            }
            Node::TEXT_NODE => {
                text_nodes_to_patch
                    .insert(*cur_node_idx, root_node.unchecked_into());
            }
            other => unimplemented!("Unsupported root node type: {}", other),
        }
    }

    *cur_node_idx += 1;

    for i in 0..child_node_count {
        let child_node = children.item(i).expect("Expecting a child node");

        match child_node.node_type() {
            Node::ELEMENT_NODE => {
                let child_to_patch = find_nodes_recursive(
                    child_node,
                    cur_node_idx,
                    nodes_to_find,
                );

                element_nodes_to_patch.extend(child_to_patch.0);
                text_nodes_to_patch.extend(child_to_patch.1);
            }
            Node::TEXT_NODE => {
                if nodes_to_find.get(&cur_node_idx).is_some() {
                    text_nodes_to_patch
                        .insert(*cur_node_idx, child_node.unchecked_into());
                }

                *cur_node_idx += 1;
            }
            Node::COMMENT_NODE => {
                // At this time we do not support user entered comment nodes, so if we see a comment
                // then it was a delimiter created by virtual-dom-rs in order to ensure that two
                // neighboring text nodes did not get merged into one by the browser. So we skip
                // over this virtual-dom-rs generated comment node.
            }
            _other => {
                // Ignoring unsupported child node type
                // TODO: What do we do with this situation? Log a warning?
            }
        }
    }

    (element_nodes_to_patch, text_nodes_to_patch)
}

/// Get the "data-sauron-vdom-id" of all the desendent of this node including itself
/// This is needed to free-up the closure that was attached ActiveClosure manually
/// TODO: Make a test when an element is removed, all of it's descendant closure should also be
/// removed as well.
fn get_node_descendant_data_vdom_id(root_element: &Element) -> Vec<u32> {
    let mut data_vdom_id = vec![];

    // TODO: there should be a better way to get the node-id back
    // without having to read from the actual dom node element
    if let Some(vdom_id_str) =
        root_element.get_attribute(super::DATA_SAURON_VDOM_ID)
    {
        let vdom_id = vdom_id_str
            .parse::<u32>()
            .expect("unable to parse sauron_vdom-id");
        data_vdom_id.push(vdom_id);
    }

    let children = root_element.child_nodes();
    let child_node_count = children.length();
    for i in 0..child_node_count {
        let child_node = children.item(i).expect("Expecting a child node");
        match child_node.node_type() {
            Node::ELEMENT_NODE => {
                let child_element = child_node.unchecked_ref::<Element>();
                let child_data_vdom_id =
                    get_node_descendant_data_vdom_id(child_element);
                data_vdom_id.extend(child_data_vdom_id);
            }
            _ => (),
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
    //crate::log!("all descendatant vdom_id: {:#?}", all_descendant_vdom_id);
    for vdom_id in all_descendant_vdom_id {
        //crate::log!("Removing listener for vdom_id: {}", vdom_id);

        let old_closure = old_closures
            .get(&vdom_id)
            .expect("There is no marked with that vdom_id");

        for (event, oc) in old_closure.iter() {
            let func: &Function = oc.as_ref().unchecked_ref();
            node.remove_event_listener_with_callback(event, func)?;
        }

        // remove closure active_closure in dom_updater to free up memory
        old_closures
            .remove(&vdom_id)
            .expect("Unable to remove old closure");
    }
    Ok(())
}

fn apply_element_patch<DSP, MSG>(
    program: &Rc<DSP>,
    node: &Element,
    old_closures: &mut ActiveClosure,
    patch: &Patch<MSG>,
) -> Result<ActiveClosure, JsValue>
where
    MSG: 'static,
    DSP: Dispatch<MSG> + 'static,
{
    let mut active_closures = ActiveClosure::new();
    match patch {
        Patch::AddAttributes(_node_idx, attributes) => {
            for attr in attributes.iter() {
                node.set_attribute(attr.name, &attr.value.to_string())?;
            }

            Ok(active_closures)
        }
        Patch::RemoveAttributes(_node_idx, attributes) => {
            for attrib_name in attributes.iter() {
                node.remove_attribute(attrib_name)?;
            }

            Ok(active_closures)
        }

        // TODO: Shall we also remove the listener first?
        Patch::AddEventListener(node_idx, events) => {
            for event in events.iter() {
                let callback =
                    event.value.get_callback().expect("expecting a callback");
                let closure_wrap: Closure<dyn FnMut(Event)> =
                    dom::create_closure_wrap(program, callback);
                let func: &Function = closure_wrap.as_ref().unchecked_ref();
                node.add_event_listener_with_callback(event.name, func)?;
                let node_id = *node_idx as u32;
                if let Some(closure) = active_closures.get_mut(&node_id) {
                    closure.push((event.name, closure_wrap));
                } else {
                    active_closures
                        .insert(node_id, vec![(event.name, closure_wrap)]);
                }
            }

            Ok(active_closures)
        }
        Patch::RemoveEventListener(_node_idx, _events) => {
            remove_event_listeners(node, old_closures)?;
            Ok(active_closures)
        }
        // THis also removes the associated closures and event listeners to the node being replaced
        // before it is actully replaced in the DOM
        //
        // Note and TODO: This doesn't free the closure and event listeners
        // of the children of this node
        Patch::Replace(_node_idx, new_node) => {
            let created_node = CreatedNode::<Node>::create_dom_node::<DSP, MSG>(
                program, new_node,
            );
            remove_event_listeners(&node, old_closures)?;
            node.replace_with_with_node_1(&created_node.node)?;
            Ok(created_node.closures)
        }
        // This also removes the associated closures and event listener to the truncated chilren
        // before actually removing it from the DOM
        //
        // Note and TODO: This doesn't free the closure and event listeners
        // of the children of this node
        // FIXME: The browser will take handling of removing the event listeners
        // of the children and indirect children of this node ( so we don't have to manually remove
        // them). However, we keep a list of our active closure in dom::ActiveClosure
        // whic needs to be manually remove by traversing of getting the
        // sauron VDOM id for each and remove is from the ActiveClosure hashmap
        Patch::TruncateChildren(_node_idx, num_children_remaining) => {
            let children = node.child_nodes();
            let child_count = children.length();

            // We skip over any separators that we placed between two text nodes
            //   -> `<!--ptns-->`
            //  and trim all children that come after our new desired `num_children_remaining`
            //let mut non_separator_children_found = 0;

            let to_be_remove_len =
                child_count as usize - num_children_remaining;
            for _index in 0..to_be_remove_len {
                let last_child = node.last_child().expect("No more last child");
                let last_element: &Element = last_child.unchecked_ref();
                remove_event_listeners(last_element, old_closures)?;
                // Do not remove comment node
                if last_child.node_type() == Node::COMMENT_NODE {
                    continue;
                }
                node.remove_child(&last_child)
                    .expect("Unable to remove last child");
            }

            Ok(active_closures)
        }
        Patch::AppendChildren(_node_idx, new_nodes) => {
            let parent = &node;
            let mut active_closures = HashMap::new();
            for new_node in new_nodes {
                let created_node = CreatedNode::<Node>::create_dom_node::<
                    DSP,
                    MSG,
                >(program, &new_node);
                parent.append_child(&created_node.node)?;
                active_closures.extend(created_node.closures);
            }

            Ok(active_closures)
        }
        Patch::ChangeText(_node_idx, _new_node) => {
            unreachable!("Elements should not receive ChangeText patches.")
        }
    }
}

fn apply_text_patch<DSP, MSG>(
    program: &Rc<DSP>,
    node: &Text,
    patch: &Patch<MSG>,
) -> Result<(), JsValue>
where
    MSG: 'static,
    DSP: Dispatch<MSG> + 'static,
{
    match patch {
        Patch::ChangeText(_node_idx, new_node) => {
            node.set_node_value(Some(&new_node.text));
        }
        Patch::Replace(_node_idx, new_node) => {
            let created_node = CreatedNode::<Node>::create_dom_node::<DSP, MSG>(
                program, new_node,
            );
            node.replace_with_with_node_1(&created_node.node)?;
        }
        _other => {
            unreachable!(
                "Text nodes should only receive ChangeText or Replace patches."
            )
        }
    };

    Ok(())
}
