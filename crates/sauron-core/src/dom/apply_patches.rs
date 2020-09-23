//! provides functionalities related to patching the DOM in the browser.
use crate::{
    dom::{
        created_node,
        created_node::{
            ActiveClosure,
            CreatedNode,
        },
    },
    mt_dom::{
        patch::{
            AddAttributes,
            AppendChildren,
            InsertNode,
            RemoveAttributes,
            RemoveNode,
            ReplaceNode,
        },
        AttValue,
    },
    Dispatch,
    Patch,
};
use js_sys::Function;
use mt_dom::NodeIdx;
use std::{
    collections::{
        BTreeMap,
        HashMap,
    },
    iter::FromIterator,
};
use wasm_bindgen::{
    JsCast,
    JsValue,
};
use web_sys::{
    Element,
    Node,
};

/// Apply all of the patches to our old root node in order to create the new root node
/// that we desire.
/// This is usually used after diffing two virtual nodes.
///
/// Note: If Program is None, it is a dumb patch, meaning
/// there is no event listener attached or changed
pub fn patch<N, DSP, MSG>(
    program: Option<&DSP>,
    root_node: N,
    old_closures: &mut ActiveClosure,
    patches: Vec<Patch<MSG>>,
    node_idx_lookup: &mut BTreeMap<NodeIdx, Node>,
) -> Result<ActiveClosure, JsValue>
where
    N: Into<Node>,
    MSG: 'static,
    DSP: Clone + Dispatch<MSG> + 'static,
{
    #[cfg(feature = "with-measure")]
    let t1 = crate::now();
    let root_node: Node = root_node.into();

    #[cfg(feature = "with-debug")]
    log::trace!("patches: {:#?}", patches);

    // Closure that were added to the DOM during this patch operation.
    let mut active_closures = HashMap::new();

    // finding the nodes to be patched before hand, instead of calling it
    // in every patch loop.
    #[cfg(not(feature = "with-nodeidx-lookup"))]
    let nodes_to_patch = find_nodes(root_node, &patches);

    #[cfg(feature = "with-nodeidx-lookup")]
    let nodes_to_patch = find_nodes_from_lookup(&patches, node_idx_lookup);

    log::trace!("nodes to patch: {:#?}", nodes_to_patch);

    log::trace!("node_idx_lookup: {:#?}", node_idx_lookup);

    #[cfg(feature = "with-measure")]
    let t2 = {
        let t2 = crate::now();
        log::trace!("finding nodes to patch took: {}ms", t2 - t1);
        t2
    };

    for patch in patches.iter() {
        let patch_node_idx = patch.node_idx();

        if let Some(element) = nodes_to_patch.get(&patch_node_idx) {
            let new_closures = apply_patch_to_node(
                program,
                &element,
                old_closures,
                &patch,
                node_idx_lookup,
            )?;
            active_closures.extend(new_closures);
        } else {
            unreachable!(
            "Getting here means we didn't find the element or next node that we were supposed to patch."
            );
        }
    }

    log::trace!("AFTER PATCHING node_idx_lookup: {:#?}", node_idx_lookup);

    #[cfg(feature = "with-measure")]
    let _t3 = {
        let t3 = crate::now();
        log::trace!("actual applying patch took: {}ms", t3 - t2);
        t3
    };

    Ok(active_closures)
}

fn find_nodes_from_lookup<MSG>(
    patches: &[Patch<MSG>],
    node_idx_lookup: &BTreeMap<NodeIdx, Node>,
) -> HashMap<usize, Node> {
    HashMap::from_iter(patches.iter().filter_map(|patch| {
        let node_idx = patch.node_idx();
        if let Some(found) = node_idx_lookup.get(&node_idx) {
            Some((node_idx, found.clone()))
        } else {
            panic!("wasn't able to find: {}", node_idx);
            //None
        }
    }))
}

/// find the nodes to be patched
/// each patch contains a node index, arranged in depth first tree.
///
/// This function is needed for optimization purposes.
/// Instead of finding the nodes each time in the patching process.
/// We find them before hand so as not to keep calling this function for each and every element to
/// be patched.
///
/// This is also IMPORTANT such that changes to the Dom tree
/// such as removal and insertion of nodes
/// will not change to NodeIdx we need to find, since
/// we already get a reference to these nodes prior to applying any of the patches.
/// TODO: took a lot of time to lookup for nodes to find
///
/// Note:
/// Worst case scenario:
/// Finding the node that is in the bottom part of the html tree
/// will take a long time, since it has to traverse to each of the
/// elements are it's descendant before it could reach the elements in the bottom.
///
/// Complexity: O(n), where n is the total number of html nodes
fn find_nodes<MSG>(
    root_node: Node,
    patches: &[Patch<MSG>],
) -> HashMap<usize, Node> {
    let mut nodes_to_find = HashMap::new();

    let mut nodes_to_patch = HashMap::new();

    for patch in patches {
        nodes_to_find.insert(patch.node_idx(), patch.tag());
    }
    #[cfg(feature = "with-measure")]
    log::trace!("there are {} nodes_to_find", nodes_to_find.len());
    find_nodes_recursive(
        root_node,
        &mut 0,
        &nodes_to_find,
        &mut nodes_to_patch,
    );
    nodes_to_patch
}

/// find the html nodes recursively
/// early returns true if all node has been found
/// before completing iterating all the elements
fn find_nodes_recursive(
    node: Node,
    cur_node_idx: &mut usize,
    nodes_to_find: &HashMap<usize, Option<&&'static str>>,
    nodes_to_patch: &mut HashMap<usize, Node>,
) -> bool {
    //let t1 = crate::now();
    if nodes_to_find.len() == 0 {
        return true;
    }
    let all_has_been_found = nodes_to_find.len() == nodes_to_patch.len();
    if all_has_been_found {
        log::trace!("all has been found..");
        return true;
    }
    // Important: We use child_nodes() instead of children() because children() ignores text nodes
    let children = node.child_nodes();
    let child_node_count = children.length();

    // If the root node matches, mark it for patching
    if let Some(_vtag) = nodes_to_find.get(&cur_node_idx) {
        nodes_to_patch.insert(*cur_node_idx, node);
    }

    //let t2 = crate::now();
    //log::trace!("find node recursive part1 took: {}ms", t2 - t1);

    for i in 0..child_node_count {
        let child_node = children.item(i).expect("Expecting a child node");
        *cur_node_idx += 1;
        if find_nodes_recursive(
            child_node,
            cur_node_idx,
            nodes_to_find,
            nodes_to_patch,
        ) {
            return true;
        }
    }
    false
    //let t3 = crate::now();
    //log::trace!("find node recursive part2 took: {}ms", t3 - t2);
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
        root_element.get_attribute(created_node::DATA_SAURON_VDOM_ID)
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
        if let Node::ELEMENT_NODE = child_node.node_type() {
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
fn apply_patch_to_node<DSP, MSG>(
    program: Option<&DSP>,
    node: &Node,
    old_closures: &mut ActiveClosure,
    patch: &Patch<MSG>,
    node_idx_lookup: &mut BTreeMap<NodeIdx, Node>,
) -> Result<ActiveClosure, JsValue>
where
    MSG: 'static,
    DSP: Clone + Dispatch<MSG> + 'static,
{
    let mut active_closures = ActiveClosure::new();

    match patch {
        Patch::InsertNode(InsertNode {
            tag: _,
            node_idx,
            node: for_insert,
        }) => {
            let element: &Element = node.unchecked_ref();
            let mut cur_node_idx = *node_idx;
            let created_node =
                CreatedNode::<Node>::create_dom_node_opt::<DSP, MSG>(
                    program,
                    &for_insert,
                    &mut cur_node_idx,
                    node_idx_lookup,
                );
            let parent_node =
                element.parent_node().expect("must have a parent node");
            parent_node
                .insert_before(&created_node.node, Some(element))
                .expect("must remove target node");

            Ok(active_closures)
        }
        Patch::AddAttributes(AddAttributes {
            tag,
            node_idx,
            attrs,
        }) => {
            let element: &Element = node.unchecked_ref();
            log::trace!("Expecting element with tag {:?}", tag);
            log::trace!(
                "at node_idx: {}, \n
                adding attributes {:#?} \n
                for element: {:?}",
                node_idx,
                attrs,
                element
            );
            CreatedNode::<Node>::set_element_attributes(
                program,
                &mut active_closures,
                element,
                attrs,
            );

            Ok(active_closures)
        }
        Patch::RemoveAttributes(RemoveAttributes {
            tag: _,
            node_idx: _,
            attrs,
        }) => {
            let element: &Element = node.unchecked_ref();
            for attr in attrs.iter() {
                for att_value in attr.value() {
                    match att_value {
                        AttValue::Plain(_) => {
                            element.remove_attribute(attr.name())?;
                        }
                        // it is an event listener
                        AttValue::Callback(_) => {
                            remove_event_listener_with_name(
                                attr.name(),
                                element,
                                old_closures,
                            )?;
                        }
                    }
                }
            }

            Ok(active_closures)
        }

        // THis also removes the associated closures and event listeners to the node being replaced
        // including the associated closures of the descendant of replaced node
        // before it is actully replaced in the DOM
        //
        Patch::ReplaceNode(ReplaceNode {
            tag: _,
            node_idx,
            replacement,
        }) => {
            let mut cur_node_idx = *node_idx;
            log::error!("replacing node at {}", node_idx);
            log::error!("DO WE REINDEX IN THESE CASE?");
            log::error!("original node was: {:#?}", node);
            log::error!("replacment is: {:#?}", replacement);
            let element: &Element = node.unchecked_ref();
            let created_node =
                CreatedNode::<Node>::create_dom_node_opt::<DSP, MSG>(
                    program,
                    replacement,
                    &mut cur_node_idx,
                    node_idx_lookup,
                );
            if element.node_type() != Node::TEXT_NODE {
                remove_event_listeners(&element, old_closures)?;
            }
            element.replace_with_with_node_1(&created_node.node)?;
            Ok(created_node.closures)
        }
        Patch::RemoveNode(RemoveNode { tag: _, node_idx }) => {
            log::error!("Removing node: {}", node_idx);
            let element: &Element = node.unchecked_ref();
            let parent_node =
                element.parent_node().expect("must have a parent node");
            if element.node_type() == Node::COMMENT_NODE {
                //do not remove comment nodes
            } else {
                parent_node
                    .remove_child(element)
                    .expect("must remove target node");
                if element.node_type() != Node::TEXT_NODE {
                    let element: &Element = node.unchecked_ref();
                    remove_event_listeners(&element, old_closures)?;
                }
            }
            Ok(active_closures)
        }
        Patch::AppendChildren(AppendChildren {
            tag: _,
            node_idx,
            children: new_nodes,
        }) => {
            let element: &Element = node.unchecked_ref();
            let mut active_closures = HashMap::new();
            let mut cur_node_idx = *node_idx;
            for new_node in new_nodes {
                cur_node_idx += 1;
                log::warn!("appending child at : {}", cur_node_idx);
                let created_node =
                    CreatedNode::<Node>::create_dom_node_opt::<DSP, MSG>(
                        program,
                        &new_node,
                        &mut cur_node_idx,
                        node_idx_lookup,
                    );
                element.append_child(&created_node.node)?;
                active_closures.extend(created_node.closures);
            }

            Ok(active_closures)
        }
        Patch::ChangeText(ct) => {
            node.set_node_value(Some(ct.get_new()));
            Ok(active_closures)
        }
    }
}
