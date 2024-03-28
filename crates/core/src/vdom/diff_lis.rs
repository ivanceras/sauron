//! diff with longest increasing subsequence

use super::diff::diff_recursive;
use super::{AttributeValue, Tag, KEY};
use super::{Node, Patch};
use crate::dom::SkipPath;
use indexmap::IndexMap;

pub fn diff_keyed_nodes<'a, MSG>(
    old_tag: Option<&'a Tag>,
    old_children: &'a [Node<MSG>],
    new_children: &'a [Node<MSG>],
    path: &SkipPath,
) -> Vec<Patch<'a, MSG>> {
    let (patches, offsets) = diff_keyed_ends(old_tag, old_children, new_children, path);

    let (left_offset, right_offset) = match offsets {
        Some(offsets) => offsets,
        None => return patches,
    };

    let mut all_patches = vec![];
    all_patches.extend(patches);

    // Ok, we now hopefully have a smaller range of children in the middle
    // within which to re-order nodes with the same keys, remove old nodes with
    // now-unused keys, and create new nodes with fresh keys.
    let old_end = old_children.len() - right_offset;
    let old_end = if old_end >= left_offset {
        old_end
    } else {
        left_offset
    };

    let old_middle = &old_children[left_offset..old_end];

    let new_end = new_children.len() - right_offset;

    let new_end = if new_end >= left_offset {
        new_end
    } else {
        left_offset
    };

    let new_middle = &new_children[left_offset..new_end];

    if new_middle.is_empty() {
        //remove the old elements
        for (index, old) in old_middle.iter().enumerate() {
            let patch = Patch::remove_node(old.tag(), path.traverse(left_offset + index).path);
            all_patches.push(patch);
        }
    } else if old_middle.is_empty() {
        // there were no old element, so just create the new elements
        if left_offset == 0 {
            // insert at the beginning of the old list
            let foothold = old_children.len() - right_offset;
            let old_tag = old_children[foothold].tag();
            let patch = Patch::insert_before_node(
                old_tag,
                path.traverse(foothold).path,
                new_middle.iter().collect::<Vec<_>>(),
            );
            all_patches.push(patch);
        } else if right_offset == 0 {
            // insert at the end of the old list
            let foothold = old_children.len() - 1;
            let old_tag = old_children[foothold].tag();
            let patch = Patch::insert_after_node(
                old_tag,
                path.traverse(foothold).path,
                new_middle.iter().collect(),
            );
            all_patches.push(patch);
        } else {
            // inserting in the middle
            let foothold = left_offset - 1;
            let old_tag = old_children[foothold].tag();
            let patch = Patch::insert_after_node(
                old_tag,
                path.traverse(foothold).path,
                new_middle.iter().collect(),
            );
            all_patches.push(patch);
        }
    } else {
        let patches = diff_keyed_middle(old_middle, new_middle, left_offset, path);
        all_patches.extend(patches);
    }
    all_patches
}

fn diff_keyed_ends<'a, MSG>(
    old_tag: Option<&'a Tag>,
    old_children: &'a [Node<MSG>],
    new_children: &'a [Node<MSG>],
    path: &SkipPath,
) -> (Vec<Patch<'a, MSG>>, Option<(usize, usize)>) {
    // keep track of the old index that has been matched already
    let mut old_index_matched = vec![];
    let mut all_patches = vec![];

    let mut left_offset = 0;
    for (index, (old, new)) in old_children.iter().zip(new_children.iter()).enumerate() {
        // abort early if we run into nodes with different keys
        if old.attribute_value(KEY) != new.attribute_value(KEY) {
            break;
        }
        let child_path = path.traverse(index);
        // diff the children and add to patches
        let patches = diff_recursive(old, new, &child_path);
        all_patches.extend(patches);
        old_index_matched.push(index);
        left_offset += 1;
    }

    // if that was all of the old children, then create and append the remaining
    // new children and we're finished
    if left_offset == old_children.len() {
        if !new_children[left_offset..].is_empty() {
            let patch = Patch::append_children(
                old_tag,
                path.path.clone(),
                new_children[left_offset..].iter().collect::<Vec<_>>(),
            );
            all_patches.push(patch);
        }
        return (all_patches, None);
    }

    // and if that was all of the new children, then remove all of the remaining
    // old children and we're finished
    if left_offset == new_children.len() {
        for (index, old) in old_children[left_offset..].iter().enumerate() {
            let patch = Patch::remove_node(old.tag(), path.traverse(left_offset + index).path);
            all_patches.push(patch);
        }
        return (all_patches, None);
    }

    // if the shared key is less than either length, then we need to walk backwards
    let mut right_offset = 0;
    for (index, (old, new)) in old_children
        .iter()
        .rev()
        .zip(new_children.iter().rev())
        .enumerate()
    {
        let old_index = old_children.len() - index - 1;
        // break if already matched this old_index or did not matched key
        if old_index_matched.contains(&old_index)
            || old.attribute_value(KEY) != new.attribute_value(KEY)
        {
            break;
        }
        let child_path = path.traverse(old_index);
        let patches = diff_recursive(old, new, &child_path);
        all_patches.extend(patches);
        right_offset += 1;
    }

    (all_patches, Some((left_offset, right_offset)))
}

/// derived from dioxus core/src/diff.rs
fn diff_keyed_middle<'a, MSG>(
    old_children: &'a [Node<MSG>],
    new_children: &'a [Node<MSG>],
    left_offset: usize,
    path: &SkipPath,
) -> Vec<Patch<'a, MSG>> {
    let mut all_patches = vec![];

    let old_children_keys: Vec<_> = old_children
        .iter()
        .map(|c| c.attribute_value(KEY))
        .collect();

    let new_children_keys: Vec<_> = new_children
        .iter()
        .map(|c| c.attribute_value(KEY))
        .collect();

    debug_assert_ne!(new_children_keys.first(), old_children_keys.first());
    debug_assert_ne!(new_children_keys.last(), old_children_keys.last());

    // make a map of old_index -> old_key
    let old_key_to_old_index: IndexMap<usize, &Vec<&AttributeValue<MSG>>> =
        IndexMap::from_iter(old_children_keys.iter().enumerate().filter_map(
            |(old_index, old_key)| old_key.as_ref().map(|old_key| (old_index, old_key)),
        ));

    let mut shared_keys: Vec<Vec<&AttributeValue<MSG>>> = vec![];

    // map each new key to the old key, carrying over the old index
    let new_index_to_old_index: Vec<usize> = new_children
        .iter()
        .map(|new| {
            if let Some(new_key) = new.attribute_value(KEY) {
                let index = old_key_to_old_index
                    .iter()
                    .find_map(|(old_index, old_key)| {
                        if new_key == **old_key {
                            Some(*old_index)
                        } else {
                            None
                        }
                    });
                if let Some(index) = index {
                    shared_keys.push(new_key);
                    index
                } else {
                    u32::MAX as usize
                }
            } else {
                u32::MAX as usize
            }
        })
        .collect();

    // if none of the old keys are reused by the new children,
    // then we remove all the remaining old children and create the new children afresh.
    if shared_keys.is_empty() && old_children.first().is_some() {
        // skip the first one, so we can use it as our foothold for inserting the new children
        for (index, old) in old_children.iter().skip(1).enumerate() {
            let patch = Patch::remove_node(old.tag(), path.traverse(index + 1).path);
            all_patches.push(patch);
        }

        let first = 0;
        log::info!("Creating a replace node in diff_keyed_middle");

        let patch = Patch::replace_node(
            old_children[left_offset + first].tag(),
            path.traverse(left_offset + first).path,
            new_children.iter().collect::<Vec<_>>(),
        );
        all_patches.push(patch);
        return all_patches;
    }

    // remove any old children that are not shared
    for (index, old_child) in old_children.iter().enumerate() {
        if let Some(old_key) = old_child.attribute_value(KEY) {
            if !shared_keys.contains(&old_key) {
                let patch =
                    Patch::remove_node(old_child.tag(), path.traverse(left_offset + index).path);
                all_patches.push(patch);
            }
        } else {
            // also remove the node that has no key
            let patch =
                Patch::remove_node(old_child.tag(), path.traverse(left_offset + index).path);
            all_patches.push(patch);
        }
    }

    // Compute the LIS of this list
    let mut lis_sequence = Vec::with_capacity(new_index_to_old_index.len());

    let mut predecessors = vec![0; new_index_to_old_index.len()];
    let mut starts = vec![0; new_index_to_old_index.len()];

    longest_increasing_subsequence::lis_with(
        &new_index_to_old_index,
        &mut lis_sequence,
        |a, b| a < b,
        &mut predecessors,
        &mut starts,
    );

    // the list_sequence came out from high to low, so we just reverse it back to arrange from low to high
    lis_sequence.reverse();

    // if a new node gets u32 max and is at the end, then it might be part of our LIS (because u32 max is a valid LIS)
    if lis_sequence.last().map(|f| new_index_to_old_index[*f]) == Some(u32::MAX as usize) {
        lis_sequence.pop();
    }

    for idx in lis_sequence.iter() {
        let patches = diff_recursive(
            &old_children[new_index_to_old_index[*idx]],
            &new_children[*idx],
            path,
        );
        all_patches.extend(patches);
    }

    // add mount instruction for the first items not covered by the lis
    let mut move_after_nodes = vec![];
    let last = *lis_sequence.last().unwrap();
    if last < (new_children.len() - 1) {
        let mut new_nodes = vec![];
        let mut node_paths = vec![];
        for (idx, new_node) in new_children[(last + 1)..].iter().enumerate() {
            let new_idx = idx + last + 1;
            let old_index = new_index_to_old_index[new_idx];
            if old_index == u32::MAX as usize {
                new_nodes.push(new_node);
            } else {
                let patches = diff_recursive(&old_children[old_index], new_node, path);
                all_patches.extend(patches);

                node_paths.push(path.traverse(left_offset + old_index).path);
            }
        }
        if !node_paths.is_empty() {
            let patch = Patch::move_after_node(
                old_children[left_offset + last].tag(),
                path.traverse(left_offset + last).path, //target element
                node_paths,
            );
            move_after_nodes.push(patch);
        }
        let old_index = new_index_to_old_index[last];
        let tag = old_children[old_index].tag();
        if !new_nodes.is_empty() {
            let patch = Patch::insert_after_node(
                tag,
                path.traverse(left_offset + old_index).path,
                new_nodes,
            );
            all_patches.push(patch);
        }
    }

    // for each spacing, generate a mount instruction
    let mut lis_iter = lis_sequence.iter().rev();
    let last = *lis_iter.next().unwrap();
    let lowest = lis_iter.min();
    if let Some(next) = lowest {
        let mut new_nodes = vec![];
        for (idx, new_node) in new_children[(next + 1)..last].iter().enumerate() {
            let new_idx = idx + next + 1;
            let old_index = new_index_to_old_index[new_idx];
            if old_index == u32::MAX as usize {
                new_nodes.push(new_node)
            } else {
                let patches = diff_recursive(&old_children[old_index], new_node, path);
                all_patches.extend(patches);
            }
        }

        if !new_nodes.is_empty() {
            let tag = old_children[last].tag();
            let patch =
                Patch::insert_before_node(tag, path.traverse(left_offset + last).path, new_nodes);
            all_patches.push(patch);
        }
    }

    // add mount instruction for the last items not covered by the list
    let mut move_before_nodes = vec![];
    let first_lis = *lis_sequence.first().unwrap();
    if first_lis > 0 {
        let mut new_nodes = vec![];
        let mut node_paths = vec![];
        for (idx, new_node) in new_children[..first_lis].iter().enumerate() {
            let old_index = new_index_to_old_index[idx];
            if old_index == u32::MAX as usize {
                new_nodes.push(new_node);
            } else {
                let patches = diff_recursive(&old_children[old_index], new_node, path);
                all_patches.extend(patches);
                node_paths.push(path.traverse(left_offset + old_index).path);
            }
        }
        if !node_paths.is_empty() {
            let first = 0;
            let patch = Patch::move_before_node(
                old_children[left_offset + first].tag(),
                path.traverse(left_offset + first).path, //target_element
                node_paths,                              //to be move after the target_element
            );
            move_before_nodes.push(patch);
        }

        if !new_nodes.is_empty() {
            let old_index = new_index_to_old_index[first_lis];
            let tag = old_children[old_index].tag();
            let patch = Patch::insert_before_node(
                tag,
                path.traverse(left_offset + old_index).path,
                new_nodes,
            );
            all_patches.push(patch);
        }
    }

    all_patches.extend(move_before_nodes);
    all_patches.extend(move_after_nodes);
    all_patches
}
