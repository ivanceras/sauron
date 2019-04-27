use crate::{Callback,
            Element,
            Event,
            Node,
            Patch,
            Value};
use std::{cmp::min,
          collections::BTreeMap,
          mem};

/// Given two Node's generate Patch's that would turn the old virtual node's
/// real DOM node equivalent into the new Node's real DOM node equivalent.
pub fn diff<'a, T, MSG>(old: &'a Node<T, MSG>,
                        new: &'a Node<T, MSG>)
                        -> Vec<Patch<'a, T, MSG>>
    where T: PartialEq
{
    diff_recursive(&old, &new, &mut 0)
}

fn diff_recursive<'a, 'b, T, MSG>(old: &'a Node<T, MSG>,
                                  new: &'a Node<T, MSG>,
                                  cur_node_idx: &'b mut usize)
                                  -> Vec<Patch<'a, T, MSG>>
    where T: PartialEq
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
        if old_element.attrs.get("key").is_some()
           && old_element.attrs.get("key") != new_element.attrs.get("key")
        {
            replace = true;
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
                let append_patch: Vec<&'a Node<T, MSG>> =
                    new_element.children[old_child_count..].iter().collect();
                patches.push(Patch::AppendChildren(*cur_node_idx, append_patch))
            }

            if new_child_count < old_child_count {
                patches.push(Patch::TruncateChildren(*cur_node_idx,
                                                     new_child_count))
            }

            let min_count = min(old_child_count, new_child_count);
            for index in 0..min_count {
                *cur_node_idx += 1;
                let old_child = &old_element.children
                                            .get(index)
                                            .expect("No old child node");
                let new_child = &new_element.children
                                            .get(index)
                                            .expect("No new chold node");
                patches.append(&mut diff_recursive(&old_child,
                                                   &new_child,
                                                   cur_node_idx))
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
fn diff_attributes<'a, 'b, T, MSG>(old_element: &'a Element<T, MSG>,
                                   new_element: &'a Element<T, MSG>,
                                   cur_node_idx: &'b mut usize)
                                   -> Vec<Patch<'a, T, MSG>> {
    let mut patches = vec![];
    let mut add_attributes: BTreeMap<&str, &Value> = BTreeMap::new();
    let mut remove_attributes: Vec<&str> = vec![];

    // TODO: -> split out into func
    for (new_attr_name, new_attr_val) in new_element.attrs.iter() {
        match old_element.attrs.get(new_attr_name) {
            Some(ref old_attr_val) => {
                if old_attr_val != &new_attr_val {
                    add_attributes.insert(new_attr_name, new_attr_val);
                }
            }
            None => {
                add_attributes.insert(new_attr_name, new_attr_val);
            }
        };
    }

    // TODO: -> split out into func
    for (old_attr_name, old_attr_val) in old_element.attrs.iter() {
        if add_attributes.get(&old_attr_name[..]).is_some() {
            continue;
        };

        match new_element.attrs.get(old_attr_name) {
            Some(ref new_attr_val) => {
                if new_attr_val != &old_attr_val {
                    remove_attributes.push(old_attr_name);
                }
            }
            None => {
                remove_attributes.push(old_attr_name);
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
fn diff_event_listener<'a, 'b, T, MSG>(old_element: &'a Element<T, MSG>,
                                       new_element: &'a Element<T, MSG>,
                                       cur_node_idx: &'b mut usize)
                                       -> Vec<Patch<'a, T, MSG>> {
    let mut patches = vec![];
    let mut add_event_listener: BTreeMap<&str, &Callback<Event, MSG>> =
        BTreeMap::new();
    let mut remove_event_listener: Vec<&str> = vec![];

    for (new_event_name, new_event_cb) in new_element.events.iter() {
        // Only add the event listener when nothing is set on that
        // event yet, since there is no way to compare the functions
        // inside the callback. Comparing the callback is pointless
        // since they are uniquely created at each instantiation of
        // each element on the vdom
        if old_element.events.get(new_event_name).is_none() {
            add_event_listener.insert(new_event_name, new_event_cb);
        }
    }

    for (old_event_name, _old_event_cb) in old_element.events.iter() {
        if add_event_listener.get(&old_event_name[..]).is_some() {
            continue;
        };

        if new_element.events.get(old_event_name).is_none() {
            remove_event_listener.push(old_event_name);
        }
    }

    if !add_event_listener.is_empty() {
        patches.push(Patch::AddEventListener(*cur_node_idx,
                                             add_event_listener));
    }
    if !remove_event_listener.is_empty() {
        patches.push(Patch::RemoveEventListener(*cur_node_idx,
                                                remove_event_listener));
    }
    patches
}

fn increment_node_idx_for_children<T, MSG>(old: &Node<T, MSG>,
                                           cur_node_idx: &mut usize) {
    *cur_node_idx += 1;
    if let Node::Element(element_node) = old {
        for child in element_node.children.iter() {
            increment_node_idx_for_children(&child, cur_node_idx);
        }
    }
}

#[cfg(test)]
mod tests {
    #![deny(warnings)]
    use super::*;
    use crate::*;
    use maplit::btreemap;

    #[test]
    fn test_replace_node() {
        let old =
            Node::Element::<&'static str, ()>(Element { tag: "div",
                                                        ..Default::default() });
        let new =
            Node::Element::<&'static str, ()>(Element { tag: "span",
                                                        ..Default::default() });

        let diff = diff::diff(&old, &new);
        assert_eq!(diff,
                   vec![Patch::Replace(0, &new)],
                   "Should replace the first node");
    }

    #[test]
    fn test_simple_diff() {
        let old =
            Node::Element::<&'static str, ()>(Element { tag: "div",
                                                        attrs: btreemap! {
                                                            "id" => "some-id".into(),
                                                            "class" => "some-class".into(),
                                                        },
                                                        ..Default::default() });

        let new =
            Node::Element::<&'static str, ()>(Element { tag: "div",
                                                        attrs: btreemap! {
                                                            "id" => "some-id".into(),
                                                            "class" => "some-class".into(),
                                                        },
                                                        ..Default::default() });

        let diff = diff(&old, &new);
        assert_eq!(diff, vec![])
    }

    #[test]
    fn test_class_changed() {
        let old =
            Node::Element::<&'static str, ()>(Element { tag: "div",
                                                        attrs: btreemap! {
                                                            "id" => "some-id".into(),
                                                            "class" => "some-class".into(),
                                                        },
                                                        ..Default::default() });

        let new =
            Node::Element::<&'static str, ()>(Element { tag: "div",
                                                        attrs: btreemap! {
                                                            "id" => "some-id".into(),
                                                            "class" => "some-class2".to_string().into(),
                                                        },
                                                        ..Default::default() });

        let diff = diff(&old, &new);
        let class2 = Value::String("some-class2".to_string());
        assert_eq!(diff,
                   vec![Patch::AddAttributes(0,
                                             btreemap! {
                                             "class" => &class2,
                                             })])
    }

    #[test]
    fn test_class_removed() {
        let old =
            Node::Element::<&'static str, ()>(Element { tag: "div",
                                                        attrs: btreemap! {
                                                            "id" => "some-id".into(),
                                                            "class" => "some-class".into(),
                                                        },
                                                        ..Default::default() });

        let new =
            Node::Element::<&'static str, ()>(Element { tag: "div",
                                                        attrs: btreemap! {
                                                            "id" => "some-id".into(),
                                                        },
                                                        ..Default::default() });

        let diff = diff(&old, &new);
        assert_eq!(diff, vec![Patch::RemoveAttributes(0, vec!["class"])])
    }

    #[test]
    fn no_change_event() {
        let func = |_| println!("Clicked!");
        let cb: Callback<Event, ()> = func.into();
        let old: Node<&'static str, ()> =
            Node::Element(Element { tag: "div",
                                    events: btreemap! {
                                        "click" => cb.clone(),
                                    },
                                    attrs: BTreeMap::new(),
                                    children: vec![],
                                    namespace: None });

        let new = Node::Element(Element { tag: "div",
                                          events: btreemap! {
                                              "click" => cb,
                                          },
                                          attrs: BTreeMap::new(),
                                          children: vec![],
                                          namespace: None });

        let diff = diff(&old, &new);
        assert_eq!(diff, vec![])
    }

    #[test]
    fn add_event() {
        let func = |_| println!("Clicked!");
        let cb: Callback<Event, ()> = func.into();

        let old: Node<&'static str, ()> =
            Node::Element(Element { tag: "div",
                                    attrs: BTreeMap::new(),
                                    events: BTreeMap::new(),
                                    children: vec![],
                                    namespace: None });

        let new = Node::Element(Element { tag: "div",
                                          events: btreemap! {
                                              "click" => cb.clone(),
                                          },
                                          attrs: BTreeMap::new(),
                                          children: vec![],
                                          namespace: None });

        let diff = diff(&old, &new);
        assert_eq!(diff,
                   vec![Patch::AddEventListener(0, btreemap! {"click" => &cb})])
    }
}
