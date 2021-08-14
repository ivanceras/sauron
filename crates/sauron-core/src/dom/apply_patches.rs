//! provides functionalities related to patching the DOM in the browser.
use crate::{
    dom::{
        created_node,
        created_node::{ActiveClosure, CreatedNode},
    },
    html::attributes::AttributeValue,
    mt_dom::patch::{
        AddAttributes, AppendChildren, InsertNode, RemoveAttributes,
        RemoveNode, ReplaceNode,
    },
    Dispatch, Patch,
};
use js_sys::Function;
use std::collections::HashMap;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Element, Node};

/// Apply all of the patches to our old root node in order to create the new root node
/// that we desire.
/// This is usually used after diffing two virtual nodes.
///
/// Note: If Program is None, it is a dumb patch, meaning
/// there is no event listener attached or changed
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
    patch_by_traversal_path(
        program,
        root_node,
        old_closures,
        focused_node,
        patches,
    )
}

/// patch using the tree path traversal instead of node_idx
pub fn patch_by_traversal_path<DSP, MSG>(
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
    #[cfg(feature = "with-measure")]
    let t1 = crate::now();

    let nodes_to_find: Vec<(&[usize], Option<&&'static str>)> = patches
        .iter()
        .map(|patch| (patch.path(), patch.tag()))
        .collect();

    let mut active_closures = HashMap::new();
    let nodes_to_patch =
        find_all_nodes_by_path(root_node.clone(), &nodes_to_find);

    #[cfg(feature = "with-measure")]
    let t2 = {
        let t2 = crate::now();
        log::trace!("finding nodes to patch took: {}ms", t2 - t1);
        t2
    };

    for patch in patches.iter() {
        let patch_path = patch.path();
        if let Some(element) = nodes_to_patch.get(patch_path) {
            let new_closures = apply_patch_to_node(
                program,
                root_node,
                &element,
                old_closures,
                focused_node,
                &patch,
            )?;
            active_closures.extend(new_closures);
        } else {
            unreachable!("Getting here means we didn't find the element of next node that we are supposed to patch, patch_path: {:?} node_idx: {}", patch_path, patch.node_idx());
        }
    }

    #[cfg(feature = "with-measure")]
    let _t3 = {
        let t3 = crate::now();
        log::trace!("actual applying patch took: {}ms", t3 - t2);
        t3
    };
    Ok(active_closures)
}

/// Apply all of the patches to our old root node in order to create the new root node
/// that we desire.
/// This is usually used after diffing two virtual nodes.
///
/// Note: If Program is None, it is a dumb patch, meaning
/// there is no event listener attached or changed
pub fn patch_by_node_idx_traversal<DSP, MSG>(
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
    #[cfg(feature = "with-measure")]
    let t1 = crate::now();

    // Closure that were added to the DOM during this patch operation.
    let mut active_closures = HashMap::new();

    // finding the nodes to be patched before hand, instead of calling it
    // in every patch loop.
    let nodes_to_patch = find_nodes_by_node_idx(root_node, &patches);

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
                root_node,
                &element,
                old_closures,
                focused_node,
                &patch,
            )?;
            active_closures.extend(new_closures);
        } else {
            unreachable!(
            "Getting here means we didn't find the element or next node that we were supposed to patch."
            );
        }
    }

    #[cfg(feature = "with-measure")]
    let _t3 = {
        let t3 = crate::now();
        log::trace!("actual applying patch took: {}ms", t3 - t2);
        t3
    };

    Ok(active_closures)
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
fn find_nodes_by_node_idx<MSG>(
    root_node: &Node,
    patches: &[Patch<MSG>],
) -> HashMap<usize, Node> {
    let mut nodes_to_find: HashMap<usize, Option<&&'static str>> =
        HashMap::new();

    let mut nodes_to_patch: HashMap<usize, Node> = HashMap::new();

    for patch in patches {
        nodes_to_find.insert(patch.node_idx(), patch.tag());
    }

    #[cfg(feature = "with-measure")]
    log::trace!("there are {} nodes_to_find", nodes_to_find.len());

    find_nodes_by_idx_recursive(
        root_node,
        &mut 0,
        &nodes_to_find,
        &mut nodes_to_patch,
    );

    let not_found = list_nodes_not_found(&nodes_to_find, &nodes_to_patch);
    if !not_found.is_empty() {
        log::warn!("These are not found: {:#?}", not_found);
    }

    nodes_to_patch
}

fn list_nodes_not_found(
    nodes_to_find: &HashMap<usize, Option<&&'static str>>,
    nodes_to_patch: &HashMap<usize, Node>,
) -> Vec<usize> {
    let mut not_found = vec![];
    for (idx, tag) in nodes_to_find.iter() {
        if nodes_to_patch.contains_key(idx) {
            // found
        } else {
            log::warn!("not found.. {} - {:?}", idx, tag);
            not_found.push(*idx);
        }
    }
    not_found
}

/// find the html nodes recursively
/// early returns true if all node has been found
/// before completing iterating all the elements
fn find_nodes_by_idx_recursive(
    node: &Node,
    cur_node_idx: &mut usize,
    nodes_to_find: &HashMap<usize, Option<&&'static str>>,
    nodes_to_patch: &mut HashMap<usize, Node>,
) -> bool {
    if nodes_to_find.len() == 0 {
        return true;
    }
    let all_has_been_found = nodes_to_find.len() == nodes_to_patch.len();
    if all_has_been_found {
        return true;
    }
    // Important: We use child_nodes() instead of children() because children() ignores text nodes
    let children = node.child_nodes();
    let child_node_count = children.length();

    // If the root node matches, mark it for patching
    if let Some(_vtag) = nodes_to_find.get(&cur_node_idx) {
        nodes_to_patch.insert(*cur_node_idx, node.clone());
    }

    for i in 0..child_node_count {
        let child_node = children.item(i).expect("Expecting a child node");
        *cur_node_idx += 1;
        if find_nodes_by_idx_recursive(
            &child_node,
            cur_node_idx,
            nodes_to_find,
            nodes_to_patch,
        ) {
            return true;
        }
    }
    false
}

fn find_node_by_path_recursive(
    node: Node,
    path: &mut Vec<usize>,
) -> Option<Node> {
    if path.is_empty() {
        Some(node)
    } else {
        let idx = path.remove(0);
        let children = node.child_nodes();
        if let Some(child) = children.item(idx as u32) {
            find_node_by_path_recursive(child, path)
        } else {
            None
        }
    }
}

fn find_all_nodes_by_path(
    node: Node,
    nodes_to_find: &Vec<(&[usize], Option<&&'static str>)>,
) -> HashMap<Vec<usize>, Node> {
    let mut nodes_to_patch: HashMap<Vec<usize>, Node> =
        HashMap::with_capacity(nodes_to_find.len());

    for (path, tag) in nodes_to_find {
        let mut traverse_path = path.to_vec();
        let root_idx = traverse_path.remove(0);
        assert_eq!(0, root_idx, "path should start at 0");
        if let Some(found) =
            find_node_by_path_recursive(node.clone(), &mut traverse_path)
        {
            nodes_to_patch.insert(path.to_vec(), found);
        } else {
            log::warn!("can not find: {:?} {:?}", path, tag);
        }
    }
    nodes_to_patch
}

/// Get the "data-sauron-vdom-id" of all the desendent of this node including itself
/// This is needed to free-up the closure that was attached ActiveClosure manually
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
///
/// Note: a mutable root_node is passed here
/// for the sole purpose of setting it when the a patch ReplaceNode at 0 is encountered.
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
        Patch::InsertNode(InsertNode {
            tag,
            patch_path,
            node: for_insert,
        }) => {
            let element: &Element = node.unchecked_ref();
            let created_node = CreatedNode::create_dom_node::<DSP, MSG>(
                program,
                &for_insert,
                focused_node,
            );
            let parent_node =
                element.parent_node().unwrap_or_else(||panic!("must have a parent node, tag: {:?}, path: {:?}, for patch: {:#?}", tag, patch_path, for_insert));
            parent_node
                .insert_before(&created_node.node, Some(element))
                .expect("must remove target node");

            Ok(active_closures)
        }
        Patch::AddAttributes(AddAttributes { attrs, .. }) => {
            let element: &Element = node.unchecked_ref();
            CreatedNode::set_element_attributes(
                program,
                &mut active_closures,
                element,
                attrs,
            );

            Ok(active_closures)
        }
        Patch::RemoveAttributes(RemoveAttributes { attrs, .. }) => {
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
        Patch::ReplaceNode(ReplaceNode {
            tag,
            patch_path,
            replacement,
        }) => {
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
            if let Some(tag) = tag {
                let target_tag = element.tag_name().to_lowercase();
                if target_tag != **tag {
                    panic!(
                        "expecting a tag: {:?}, but found: {:?}",
                        tag, target_tag
                    );
                }
            }

            if element.node_type() != Node::TEXT_NODE {
                remove_event_listeners(&element, old_closures)?;
            }
            element
                .replace_with_with_node_1(&created_node.node)
                .expect("must replace node");

            // if what we are replacing is a root node:
            // we replace the root node here, so that's reference is updated
            // to the newly created node
            if patch_path.path == &[0] {
                log::debug!("replacing root node..");
                *root_node = created_node.node;
            }
            Ok(created_node.closures)
        }
        Patch::RemoveNode(RemoveNode { .. }) => {
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
            patch_path: _,
            children: new_nodes,
        }) => {
            let element: &Element = node.unchecked_ref();
            let mut active_closures = HashMap::new();
            for (_append_children_node_idx, new_node) in new_nodes.iter() {
                let created_node = CreatedNode::create_dom_node::<DSP, MSG>(
                    program,
                    &new_node,
                    focused_node,
                );
                element.append_child(&created_node.node)?;
                active_closures.extend(created_node.closures);
            }
            Ok(active_closures)
        }
        Patch::ChangeText(ct) => {
            node.set_node_value(Some(&ct.new.text));
            Ok(active_closures)
        }
    }
}
