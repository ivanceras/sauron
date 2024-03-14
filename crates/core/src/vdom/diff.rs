//! provides diffing algorithm which returns patches
use super::{diff_lis, Attribute, Element, Node, Patch, TreePath};
use super::{Tag, KEY, REPLACE, SKIP, SKIP_CRITERIA};
use std::{cmp, mem};

/// Return the patches needed for `old_node` to have the same DOM as `new_node`
///
/// # Agruments
/// * old_node - the old virtual dom node
/// * new_node - the new virtual dom node
/// * key - the literal name of key attribute, ie: "key"
///
/// # Example
/// ```rust
/// use sauron::{diff::*, vdom::element, *};
///
///
/// let old: Node<()> = element(
///     "main",
///     vec![attr("class", "container")],
///     vec![
///         element("div", vec![attr("key", "1")], vec![]),
///         element("div", vec![attr("key", "2")], vec![]),
///     ],
/// );
///
/// let new: Node<()> = element(
///     "main",
///     vec![attr("class", "container")],
///     vec![element("div", vec![attr("key", "2")], vec![])],
/// );
///
/// let diff = diff(&old, &new);
/// assert_eq!(
///     diff,
///     vec![Patch::remove_node(
///         Some(&"div"),
///         TreePath::new(vec![ 0]),
///     )
///     ]
/// );
/// ```
pub fn diff<'a, MSG>(old_node: &'a Node<MSG>, new_node: &'a Node<MSG>) -> Vec<Patch<'a, MSG>> {
    diff_recursive(old_node, new_node, &TreePath::root(), None)
}

fn is_any_keyed<MSG>(nodes: &[Node<MSG>]) -> bool {
    nodes.iter().any(|child| is_keyed_node(child))
}

/// returns true any attributes of this node attribute has key in it
fn is_keyed_node<MSG>(node: &Node<MSG>) -> bool {
    if let Some(attributes) = node.attributes() {
        attributes.iter().any(|att| att.name == *KEY)
    } else {
        false
    }
}

fn should_replace<'a, MSG>(old_node: &'a Node<MSG>, new_node: &'a Node<MSG>) -> bool {
    // replace if they have different enum variants
    if mem::discriminant(old_node) != mem::discriminant(new_node) {
        return true;
    }
    let replace = |_old_node: &'a Node<MSG>, new_node: &'a Node<MSG>| {
        let explicit_replace_attr = new_node
            .first_value(&REPLACE)
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        /*
        let old_callbacks = old_node.get_callbacks();
        let new_callbacks = new_node.get_callbacks();
        let old_node_has_event = !old_callbacks.is_empty();
        let new_node_has_event = !new_callbacks.is_empty();
        // don't recycle when old node has event while new new doesn't have
        let forbid_recycle = old_node_has_event && !new_node_has_event;
        log::debug!("forbit_recycle: {forbid_recycle}");

        // event is added in the new when the old node has no events yet
        let events_added = !old_node_has_event && new_node_has_event;
        log::debug!("events added: {events_added}");

        // replace if the number of callbacks changed as it is not just adding event
        let event_listeners_altered = !events_added && old_callbacks.len() != new_callbacks.len();
        log::debug!("event_listeners_altered: {event_listeners_altered}");

        explicit_replace_attr || forbid_recycle || event_listeners_altered
        */
        explicit_replace_attr
    };
    // handle explicit replace if the Rep fn evaluates to true
    if replace(old_node, new_node) {
        return true;
    }

    // replace if the old key does not match the new key
    if let (Some(old_key), Some(new_key)) =
        (old_node.attribute_value(KEY), new_node.attribute_value(KEY))
    {
        if old_key != new_key {
            return true;
        }
    }
    // replace if they have different element tag
    if let (Node::Element(old_element), Node::Element(new_element)) = (old_node, new_node) {
        // Replace if there are different element tags
        if old_element.tag != new_element.tag {
            return true;
        }
    }
    false
}

/// diff the nodes recursively
pub fn diff_recursive<'a, MSG>(
    old_node: &'a Node<MSG>,
    new_node: &'a Node<MSG>,
    path: &TreePath,
    depth_limit: Option<usize>,
) -> Vec<Patch<'a, MSG>> {
    // return if it has reach depth limit
    if let Some(depth_limit) = depth_limit {
        if path.path.len() == depth_limit {
            return vec![];
        }
    }
    let skip = |old_node: &'a Node<MSG>, new_node: &'a Node<MSG>| {
        let new_skip_criteria = new_node.attribute_value(&SKIP_CRITERIA);
        let old_skip_criteria = old_node.attribute_value(&SKIP_CRITERIA);
        // if old and new skip_criteria didn't change skip diffing this nodes
        match (new_skip_criteria, old_skip_criteria) {
            (Some(new), Some(old)) => new == old,
            _ => new_node
                .first_value(&SKIP)
                .and_then(|v| v.as_bool())
                .unwrap_or(false),
        }
    };
    // skip diffing if the function evaluates to true
    if skip(old_node, new_node) {
        return vec![];
    }

    // replace node and return early
    if should_replace(old_node, new_node) {
        return vec![Patch::replace_node(
            old_node.tag(),
            path.clone(),
            vec![new_node],
        )];
    }

    // skip diffing if they are essentially the same node
    if old_node == new_node {
        return vec![];
    }

    let mut patches = vec![];

    // The following comparison can only contain identical variants, other
    // cases have already been handled above by comparing variant
    // discriminants.
    match (old_node, new_node) {
        (Node::Leaf(old_leaf), Node::Leaf(new_leaf)) => {
            if old_leaf != new_leaf {
                let ct = Patch::replace_node(old_node.tag(), path.clone(), vec![new_node]);
                patches.push(ct);
            }
        }
        // We're comparing two element nodes
        (Node::Element(old_element), Node::Element(new_element)) => {
            let patch = diff_element(old_element, new_element, path, depth_limit);
            patches.extend(patch);
        }
        (Node::Fragment(old_nodes), Node::Fragment(new_nodes)) => {
            // we back track since Fragment is not a real node, but it would still
            // be traversed from the prior call
            let patch = diff_nodes(None, old_nodes, new_nodes, &path.backtrack(), depth_limit);
            patches.extend(patch);
        }
        (Node::NodeList(_old_elements), Node::NodeList(_new_elements)) => {
            panic!("Node list must have already unrolled when creating an element");
        }
        _ => {
            unreachable!("Unequal variant discriminants should already have been handled");
        }
    };

    patches
}

fn diff_element<'a, MSG>(
    old_element: &'a Element<MSG>,
    new_element: &'a Element<MSG>,
    path: &TreePath,
    depth_limit: Option<usize>,
) -> Vec<Patch<'a, MSG>> {
    let mut patches = create_attribute_patches(old_element, new_element, path);

    let more_patches = diff_nodes(
        Some(old_element.tag()),
        &old_element.children(),
        &new_element.children(),
        path,
        depth_limit,
    );

    patches.extend(more_patches);
    patches
}

fn diff_nodes<'a, MSG>(
    old_tag: Option<&'a Tag>,
    old_children: &'a [Node<MSG>],
    new_children: &'a [Node<MSG>],
    path: &TreePath,
    depth_limit: Option<usize>,
) -> Vec<Patch<'a, MSG>> {
    let diff_as_keyed = is_any_keyed(old_children) || is_any_keyed(new_children);

    if diff_as_keyed {
        let keyed_patches =
            diff_lis::diff_keyed_nodes(old_tag, old_children, new_children, path, depth_limit);
        keyed_patches
    } else {
        let non_keyed_patches =
            diff_non_keyed_nodes(old_tag, old_children, new_children, path, depth_limit);
        non_keyed_patches
    }
}

/// In diffing non_keyed nodes,
///  we reuse existing DOM elements as much as possible
///
///  The algorithm used here is very simple.
///
///  If there are more children in the old_element than the new_element
///  the excess children is all removed.
///
///  If there are more children in the new_element than the old_element
///  it will be all appended in the old_element.
fn diff_non_keyed_nodes<'a, MSG>(
    old_element_tag: Option<&'a Tag>,
    old_children: &'a [Node<MSG>],
    new_children: &'a [Node<MSG>],
    path: &TreePath,
    depth_limit: Option<usize>,
) -> Vec<Patch<'a, MSG>> {
    let mut patches = vec![];
    let old_child_count = old_children.len();
    let new_child_count = new_children.len();

    let min_count = cmp::min(old_child_count, new_child_count);
    for index in 0..min_count {
        // if we iterate trough the old elements, a new child_path is created for that iteration
        let child_path = path.traverse(index);

        let old_child = &old_children.get(index).expect("No old_node child node");
        let new_child = &new_children.get(index).expect("No new child node");

        let more_patches = diff_recursive(old_child, new_child, &child_path, depth_limit);
        patches.extend(more_patches);
    }

    // If there are more new child than old_node child, we make a patch to append the excess element
    // starting from old_child_count to the last item of the new_elements
    if new_child_count > old_child_count {
        patches.push(Patch::append_children(
            old_element_tag,
            path.clone(),
            new_children.iter().skip(old_child_count).collect(),
        ));
    }

    if new_child_count < old_child_count {
        let remove_node_patches = old_children
            .iter()
            .skip(new_child_count)
            .enumerate()
            .map(|(i, old_child)| {
                Patch::remove_node(old_child.tag(), path.traverse(new_child_count + i))
            })
            .collect::<Vec<_>>();

        patches.extend(remove_node_patches);
    }

    patches
}

///
/// Note: The performance bottlenecks
///     - allocating new vec
///     - merging attributes of the same name
fn create_attribute_patches<'a, MSG>(
    old_element: &'a Element<MSG>,
    new_element: &'a Element<MSG>,
    path: &TreePath,
) -> Vec<Patch<'a, MSG>> {
    //let new_attributes = new_element.attributes().into_iter().filter(|a|a.has_non_static_value()).collect::<Vec<_>>();
    //let old_attributes = old_element.attributes().into_iter().filter(|a|a.has_non_static_value()).collect::<Vec<_>>();

    let new_attributes = new_element.attributes();
    let old_attributes = old_element.attributes();

    let mut patches = vec![];

    // return early if both attributes are empty
    if old_attributes.is_empty() && new_attributes.is_empty() {
        return vec![];
    }

    let mut add_attributes: Vec<&Attribute<MSG>> = vec![];
    let mut remove_attributes: Vec<&Attribute<MSG>> = vec![];

    let new_attributes_grouped = Attribute::group_attributes_per_name(new_attributes.iter());
    let old_attributes_grouped = Attribute::group_attributes_per_name(old_attributes.iter());

    // for all new elements that doesn't exist in the old elements
    // or the values differ
    // add it to the AddAttribute patches
    for (new_attr_name, new_attrs) in new_attributes_grouped.iter() {
        let old_attr_values = old_attributes_grouped
            .get(new_attr_name)
            .map(|attrs| attrs.iter().map(|attr| &attr.value).collect::<Vec<_>>());

        let new_attr_values = new_attributes_grouped
            .get(new_attr_name)
            .map(|attrs| attrs.iter().map(|attr| &attr.value).collect::<Vec<_>>());

        if let Some(old_attr_values) = old_attr_values {
            let new_attr_values = new_attr_values.expect("must have new attr values");
            if old_attr_values != new_attr_values {
                add_attributes.extend(new_attrs);
            }
        } else {
            add_attributes.extend(new_attrs);
        }
    }

    // if this attribute name does not exist anymore
    // to the new element, remove it
    for (old_attr_name, old_attrs) in old_attributes_grouped.iter() {
        if !new_attributes_grouped.contains_key(old_attr_name) {
            remove_attributes.extend(old_attrs);
        }
    }

    if !add_attributes.is_empty() {
        patches.push(Patch::add_attributes(
            &old_element.tag,
            path.clone(),
            add_attributes,
        ));
    }
    if !remove_attributes.is_empty() {
        patches.push(Patch::remove_attributes(
            &old_element.tag,
            path.clone(),
            remove_attributes,
        ));
    }
    patches
}
