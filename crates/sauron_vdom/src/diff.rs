use crate::{
    Attribute,
    Element,
    Node,
    Patch,
};
use std::{
    cmp::min,
    mem,
};

//TODO: Move this to sauron html specific
/// This is a sauron html specific functionality
/// diff 2 nodes with attribute using `&'static str` instead of generic ATT
pub fn diff<'a, T, EVENT, MSG>(
    old: &'a Node<T, &'static str, EVENT, MSG>,
    new: &'a Node<T, &'static str, EVENT, MSG>,
) -> Vec<Patch<'a, T, &'static str, EVENT, MSG>>
where
    MSG: 'static,
    EVENT: 'static,
    T: PartialEq,
{
    diff_with_key(old, new, &"key")
}

/// Given two Node's generate Patch's that would turn the old virtual node's
/// real DOM node equivalent into the new Node's real DOM node equivalent.
pub fn diff_with_key<'a, T, ATT, EVENT, MSG>(
    old: &'a Node<T, ATT, EVENT, MSG>,
    new: &'a Node<T, ATT, EVENT, MSG>,
    key: &ATT,
) -> Vec<Patch<'a, T, ATT, EVENT, MSG>>
where
    MSG: 'static,
    EVENT: 'static,
    T: PartialEq,
    ATT: PartialEq + Ord + ToString + Clone,
{
    diff_recursive(&old, &new, &mut 0, key)
}

fn diff_recursive<'a, 'b, T, ATT, EVENT, MSG>(
    old: &'a Node<T, ATT, EVENT, MSG>,
    new: &'a Node<T, ATT, EVENT, MSG>,
    cur_node_idx: &'b mut usize,
    key: &ATT,
) -> Vec<Patch<'a, T, ATT, EVENT, MSG>>
where
    MSG: 'static,
    EVENT: 'static,
    T: PartialEq,
    ATT: PartialEq + Ord + ToString + Clone,
{
    let mut patches = vec![];
    // Different enum variants, replace!
    let mut replace = mem::discriminant(old) != mem::discriminant(new);

    if let (Node::Element(old_element), Node::Element(new_element)) = (old, new)
    {
        // Replace if there are different element tags
        if old_element.tag != new_element.tag {
            replace = true;
        }

        // Replace if two elements have different keys
        // TODO: More robust key support. This is just an early stopgap to allow you to force replace
        // an element... say if it's event changed. Just change the key name for now.
        // In the future we want keys to be used to create a Patch::ReOrder to re-order siblings
        let old_key_value = old_element.get_attr_value(key);
        let new_key_value = new_element.get_attr_value(key);
        if let (Some(old_key_value), Some(new_key_value)) =
            (old_key_value, new_key_value)
        {
            // replace if the 2 keys differ
            replace = old_key_value != new_key_value;
        }
    }

    // Handle replacing of a node
    if replace {
        patches.push(Patch::Replace(*cur_node_idx, &new));
        if let Node::Element(old_element_node) = old {
            for child in old_element_node.children.iter() {
                increment_node_idx_for_children(child, cur_node_idx);
            }
        }
        return patches;
    }

    // The following comparison can only contain identical variants, other
    // cases have already been handled above by comparing variant
    // discriminants.
    match (old, new) {
        // We're comparing two text nodes
        (Node::Text(old_text), Node::Text(new_text)) => {
            if old_text != new_text {
                patches.push(Patch::ChangeText(*cur_node_idx, &new_text));
            }
        }

        // We're comparing two element nodes
        (Node::Element(old_element), Node::Element(new_element)) => {
            let attributes_patches =
                diff_attributes(old_element, new_element, cur_node_idx);
            patches.extend(attributes_patches);

            let listener_patches =
                diff_event_listener(old_element, new_element, cur_node_idx);
            patches.extend(listener_patches);

            let old_child_count = old_element.children.len();
            let new_child_count = new_element.children.len();

            if new_child_count > old_child_count {
                let append_patch: Vec<&'a Node<T, ATT, EVENT, MSG>> =
                    new_element.children[old_child_count..].iter().collect();
                patches.push(Patch::AppendChildren(*cur_node_idx, append_patch))
            }

            if new_child_count < old_child_count {
                patches.push(Patch::TruncateChildren(
                    *cur_node_idx,
                    new_child_count,
                ))
            }

            let min_count = min(old_child_count, new_child_count);
            for index in 0..min_count {
                *cur_node_idx += 1;
                let old_child = &old_element
                    .children
                    .get(index)
                    .expect("No old child node");
                let new_child = &new_element
                    .children
                    .get(index)
                    .expect("No new chold node");
                patches.append(&mut diff_recursive(
                    &old_child,
                    &new_child,
                    cur_node_idx,
                    key,
                ))
            }
            if new_child_count < old_child_count {
                for child in old_element.children[min_count..].iter() {
                    increment_node_idx_for_children(child, cur_node_idx);
                }
            }
        }
        (Node::Text(_), Node::Element(_))
        | (Node::Element(_), Node::Text(_)) => {
            unreachable!(
                "Unequal variant discriminants should already have been handled"
            );
        }
    };

    patches
}

// diff the attributes of old element to the new element at this cur_node_idx
fn diff_attributes<'a, 'b, T, ATT, EVENT, MSG>(
    old_element: &'a Element<T, ATT, EVENT, MSG>,
    new_element: &'a Element<T, ATT, EVENT, MSG>,
    cur_node_idx: &'b mut usize,
) -> Vec<Patch<'a, T, ATT, EVENT, MSG>>
where
    MSG: 'static,
    EVENT: 'static,
    ATT: PartialEq + Ord + ToString + Clone,
{
    let mut patches = vec![];
    let mut add_attributes: Vec<Attribute<ATT, EVENT, MSG>> = vec![];
    let mut remove_attributes: Vec<ATT> = vec![];

    for new_attr in new_element.attributes().iter() {
        let old_attr_value = old_element.get_attr_value(&new_attr.name);
        let new_attr_value = new_element.get_attr_value(&new_attr.name);
        if old_attr_value.is_none() || old_attr_value != new_attr_value {
            if let Some(new_attr_value) = new_attr_value {
                add_attributes.push(Attribute {
                    namespace: new_attr.namespace,
                    name: new_attr.name.clone(),
                    value: new_attr_value.into(),
                });
            }
        }
    }

    // if this attribute name does not exist anymore
    // to the new element, remove it
    for old_attr in old_element.attributes().iter() {
        if let Some(_) = new_element.get_attr_value(&old_attr.name) {
            // the attribute still exist in the new element
            // and it must have been changed in add_attributes when they differe
        } else {
            remove_attributes.push(old_attr.name.clone());
        }
    }

    if !add_attributes.is_empty() {
        patches.push(Patch::AddAttributes(*cur_node_idx, add_attributes));
    }
    if !remove_attributes.is_empty() {
        patches.push(Patch::RemoveAttributes(*cur_node_idx, remove_attributes));
    }
    patches
}

// diff the events of the old element compared to the new element at this cur_node_idx
fn diff_event_listener<'a, 'b, T, ATT, EVENT, MSG>(
    old_element: &'a Element<T, ATT, EVENT, MSG>,
    new_element: &'a Element<T, ATT, EVENT, MSG>,
    cur_node_idx: &'b mut usize,
) -> Vec<Patch<'a, T, ATT, EVENT, MSG>>
where
    MSG: 'static,
    EVENT: 'static,
    ATT: PartialEq + Ord + ToString + Clone,
{
    let mut patches = vec![];
    let mut add_event_listener: Vec<&Attribute<ATT, EVENT, MSG>> = vec![];
    let mut remove_event_listener: Vec<ATT> = vec![];

    for new_event in new_element.events().iter() {
        // Only add the event listener when nothing is set on that
        // event yet, since there is no way to compare the functions
        // inside the callback. Comparing the callback is pointless
        // since they are uniquely created at each instantiation of
        // each element on the vdom
        let old_event = old_element.get_event(&new_event.name);
        if old_event.is_none() {
            add_event_listener.push(new_event);
        }
    }

    for old_event in old_element.events().iter() {
        if new_element.get_event(&old_event.name).is_none() {
            remove_event_listener.push(old_event.name.clone());
        }
    }

    if !add_event_listener.is_empty() {
        patches
            .push(Patch::AddEventListener(*cur_node_idx, add_event_listener));
    }
    if !remove_event_listener.is_empty() {
        patches.push(Patch::RemoveEventListener(
            *cur_node_idx,
            remove_event_listener,
        ));
    }
    patches
}

fn increment_node_idx_for_children<T, ATT, EVENT, MSG>(
    old: &Node<T, ATT, EVENT, MSG>,
    cur_node_idx: &mut usize,
) where
    ATT: Clone,
{
    *cur_node_idx += 1;
    if let Node::Element(element_node) = old {
        for child in element_node.children.iter() {
            increment_node_idx_for_children(&child, cur_node_idx);
        }
    }
}
