#![deny(warnings)]
use sauron_vdom::{
    builder::{
        attr,
        on,
    },
    *,
};

#[test]
fn test_replace_node() {
    let old: Node<&'static str, (), ()> = Node::Element(Element {
        tag: "div",
        ..Default::default()
    });
    let new = Node::Element(Element {
        tag: "span",
        ..Default::default()
    });

    let diff = diff(&old, &new);
    assert_eq!(
        diff,
        vec![Patch::Replace(0, &new)],
        "Should replace the first node"
    );
}

#[test]
fn test_simple_diff() {
    let old: Node<&'static str, (), ()> = Node::Element(Element {
        tag: "div",
        attrs: vec![attr("id", "some-id"), attr("class", "some-class")],
        ..Default::default()
    });

    let new: Node<&'static str, (), ()> = Node::Element(Element {
        tag: "div",
        attrs: vec![attr("id", "some-id"), attr("class", "some-class")],
        ..Default::default()
    });

    let diff = diff(&old, &new);
    assert_eq!(diff, vec![])
}

#[test]
fn test_class_changed() {
    let old: Node<&'static str, (), ()> = Node::Element(Element {
        tag: "div",
        attrs: vec![attr("id", "some-id"), attr("class", "some-class")],
        ..Default::default()
    });

    let new = Node::Element(Element {
        tag: "div",
        attrs: vec![attr("id", "some-id"), attr("class", "some-class2")],
        ..Default::default()
    });

    let diff = diff(&old, &new);
    assert_eq!(
        diff,
        vec![Patch::AddAttributes(0, vec![&attr("class", "some-class2")])]
    )
}

#[test]
fn test_class_removed() {
    let old: Node<&'static str, (), ()> = Node::Element(Element {
        tag: "div",
        attrs: vec![attr("id", "some-id"), attr("class", "some-class")],
        ..Default::default()
    });

    let new = Node::Element(Element {
        tag: "div",
        attrs: vec![attr("id", "some-id")],
        ..Default::default()
    });

    let diff = diff(&old, &new);
    assert_eq!(diff, vec![Patch::RemoveAttributes(0, vec!["class"])])
}

#[test]
fn no_change_event() {
    let func = |_| println!("Clicked!");
    let cb: Callback<(), ()> = func.into();
    let old: Node<&'static str, (), ()> = Node::Element(Element {
        tag: "div",
        attrs: vec![on("click", cb.clone())],
        children: vec![],
        namespace: None,
    });

    let new = Node::Element(Element {
        tag: "div",
        attrs: vec![on("click", cb)],
        children: vec![],
        namespace: None,
    });

    let diff = diff(&old, &new);
    assert_eq!(diff, vec![])
}

#[test]
fn add_event() {
    let func = |_| println!("Clicked!");
    let cb: Callback<(), ()> = func.into();

    let old: Node<&'static str, (), ()> = Node::Element(Element {
        tag: "div",
        attrs: vec![],
        children: vec![],
        namespace: None,
    });

    let new = Node::Element(Element {
        tag: "div",
        attrs: vec![on("click", cb.clone())],
        children: vec![],
        namespace: None,
    });

    let diff = diff(&old, &new);
    assert_eq!(
        diff,
        vec![Patch::AddEventListener(0, vec![&on("click", cb)])]
    )
}
