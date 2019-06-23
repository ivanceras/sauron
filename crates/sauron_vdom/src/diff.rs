use crate::{
    Attribute,
    Callback,
    Element,
    Node,
    Patch,
};
use std::{
    cmp::min,
    collections::BTreeMap,
    mem,
};

/// Given two Node's generate Patch's that would turn the old virtual node's
/// real DOM node equivalent into the new Node's real DOM node equivalent.
pub fn diff<'a, T, EVENT, MSG>(
    old: &'a Node<T, EVENT, MSG>,
    new: &'a Node<T, EVENT, MSG>,
) -> Vec<Patch<'a, T, EVENT, MSG>>
where
    T: PartialEq + Clone,
    MSG: PartialEq + Clone + 'static,
    EVENT: PartialEq + Clone + 'static,
{
    diff_recursive(&old, &new, &mut 0)
}

fn diff_recursive<'a, 'b, T, EVENT, MSG>(
    old: &'a Node<T, EVENT, MSG>,
    new: &'a Node<T, EVENT, MSG>,
    cur_node_idx: &'b mut usize,
) -> Vec<Patch<'a, T, EVENT, MSG>>
where
    T: PartialEq + Clone,
    MSG: PartialEq + Clone + 'static,
    EVENT: PartialEq + Clone + 'static,
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
        let old_key_value = old_element.get_attrib_value("key");
        let new_key_value = new_element.get_attrib_value("key");
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
                let append_patch: Vec<&'a Node<T, EVENT, MSG>> =
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
            unreachable!("Unequal variant discriminants should already have been handled");
        }
    };

    patches
}

// diff the attributes of old element to the new element at this cur_node_idx
fn diff_attributes<'a, 'b, T, EVENT, MSG>(
    old_element: &'a Element<T, EVENT, MSG>,
    new_element: &'a Element<T, EVENT, MSG>,
    cur_node_idx: &'b mut usize,
) -> Vec<Patch<'a, T, EVENT, MSG>>
where
    MSG: PartialEq + Clone + 'static,
    T: Clone,
    EVENT: PartialEq + Clone + 'static,
{
    let mut patches = vec![];
    let mut add_attributes: Vec<&Attribute<EVENT, MSG>> = vec![];
    let mut remove_attributes: Vec<&str> = vec![];

    for new_attr in new_element.attributes().iter() {
        match old_element.get_attrib_value(new_attr.name) {
            Some(old_attr_val) => {
                if *old_attr_val != new_attr.value {
                    add_attributes.push(new_attr);
                }
            }
            None => {
                add_attributes.push(new_attr);
            }
        };
    }

    for old_attr in old_element.attributes().iter() {
        if add_attributes
            .iter()
            .find(|attr| attr.name == old_attr.name)
            .is_some()
        {
            continue;
        };

        match new_element.get_attr(old_attr.name) {
            Some(ref new_attr) => {
                if new_attr.value != old_attr.value {
                    remove_attributes.push(old_attr.name);
                }
            }
            None => {
                remove_attributes.push(old_attr.name);
            }
        };
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
fn diff_event_listener<'a, 'b, T, EVENT, MSG>(
    old_element: &'a Element<T, EVENT, MSG>,
    new_element: &'a Element<T, EVENT, MSG>,
    cur_node_idx: &'b mut usize,
) -> Vec<Patch<'a, T, EVENT, MSG>>
where
    MSG: PartialEq + Clone + 'static,
    T: Clone,
    EVENT: PartialEq + Clone + 'static,
{
    let mut patches = vec![];
    let mut add_event_listener: Vec<&Attribute<EVENT, MSG>> = vec![];
    let mut remove_event_listener: Vec<&str> = vec![];

    for new_event in new_element.events().iter() {
        // Only add the event listener when nothing is set on that
        // event yet, since there is no way to compare the functions
        // inside the callback. Comparing the callback is pointless
        // since they are uniquely created at each instantiation of
        // each element on the vdom
        let old_event = old_element.get_event(new_event.name);
        if old_event.is_none() {
            add_event_listener.push(new_event);
        }
    }

    for old_event in old_element.events().iter() {
        if add_event_listener
            .iter()
            .find(|event| event.name == old_event.name)
            .is_some()
        {
            continue;
        };

        if new_element.get_event(old_event.name).is_none() {
            remove_event_listener.push(old_event.name);
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

fn increment_node_idx_for_children<T, EVENT, MSG>(
    old: &Node<T, EVENT, MSG>,
    cur_node_idx: &mut usize,
) where
    MSG: Clone,
{
    *cur_node_idx += 1;
    if let Node::Element(element_node) = old {
        for child in element_node.children.iter() {
            increment_node_idx_for_children(&child, cur_node_idx);
        }
    }
}
