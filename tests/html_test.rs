#![deny(warnings)]
use crate::mt_dom::TreePath;
use sauron_core::prelude::*;

#[test]
fn test_macros() {
    let html: Node<()> = div(vec![class("class1"), class("class2")], vec![]);
    let attrs = html.get_attributes().unwrap();
    println!("attrs: {:#?}", attrs);
}

#[test]
fn test_macros_trailing_commas() {
    let html: Node<()> = div(vec![class("class1"), class("class2")], vec![]);
    let attrs = html.get_attributes().unwrap();
    println!("attrs: {:#?}", attrs);
}

#[test]
fn test_macros_trailing_commas_in_attributes_only() {
    let html: Node<()> = div(vec![class("class1"), class("class2")], vec![]);
    let attrs = html.get_attributes().unwrap();
    println!("attrs: {:#?}", attrs);
}

#[test]
fn test_macros_trailing_commas_in_children_only() {
    let html: Node<()> = div(
        vec![class("class1"), class("class2")],
        vec![text("This is input")],
    );
    let attrs = html.get_attributes().unwrap();
    println!("attrs: {:#?}", attrs);
}

#[test]
fn test_macros_trailing_commas_in_children_and_params() {
    let html: Node<()> = div(
        vec![class("class1"), class("class2")],
        vec![text("This is input")],
    );
    let attrs = html.get_attributes().unwrap();
    println!("attrs: {:#?}", attrs);
}

#[test]
fn test_macros_trailing_commas_in_attribute_and_children() {
    let html: Node<()> = div(
        vec![class("class1"), class("class2")],
        vec![text("This is input")],
    );
    let attrs = html.get_attributes().unwrap();
    println!("attrs: {:#?}", attrs);
}

#[test]
fn test_macros_with_lines() {
    let html: Node<()> = div(
        vec![class("class1"), class("class2")],
        vec![input(vec![], vec![text("This is an input")])],
    );
    let attrs = html.get_attributes().unwrap();
    println!("attrs: {:#?}", attrs);
}

#[test]
fn simple_builder() {
    let mut div: Element<()> = Element::new(None, "div", vec![], vec![], false);
    div.add_attributes(vec![attr("class", "some-class")]);
    let expected: Element<()> =
        Element::new(None, "div", vec![class("some-class")], vec![], false);

    assert_eq!(div, expected);
}

#[test]
fn builder_with_event() {
    let cb = |x: Event| {
        println!("hello! {:?}", x);
    };
    let mut div: Element<()> = Element::new(None, "div", vec![], vec![], false);
    div.add_attributes(vec![on("click", cb.clone())]);
    let expected: Element<()> =
        Element::new(None, "div", vec![on("click", cb)], vec![], false);

    assert_eq!(
        div, expected,
        "Cloning a callback should only clone the reference"
    );
}

#[test]
fn builder_with_children() {
    let mut div: Element<()> = Element::new(None, "div", vec![], vec![], false);
    div.add_attributes(vec![attr("class", "some-class")]);
    div.add_children(vec![text("Hello")]);
    let expected = Element::new(
        None,
        "div",
        vec![class("some-class")],
        vec![text("Hello")],
        false,
    );

    assert_eq!(div, expected);
}

#[test]
fn replace_node() {
    let old: Node<()> = div(vec![], vec![]);
    let new: Node<()> = span(vec![], vec![]);
    assert_eq!(
        diff(&old, &new),
        vec![Patch::replace_node(
            Some(&"div"),
            TreePath::new(vec![]),
            &span(vec![], vec![])
        )],
        "ReplaceNode the root if the tag changed"
    );

    let old: Node<()> = div(vec![], vec![b(vec![], vec![])]);
    let new: Node<()> = div(vec![], vec![strong(vec![], vec![])]);
    assert_eq!(
        diff(&old, &new),
        vec![Patch::replace_node(
            Some(&"b"),
            TreePath::new(vec![0]),
            &strong(vec![], vec![])
        )],
    );

    let old: Node<()> =
        div(vec![], vec![b(vec![], vec![text("1")]), b(vec![], vec![])]);
    let new: Node<()> =
        div(vec![], vec![i(vec![], vec![text("1")]), i(vec![], vec![])]);
    let patch = diff(&old, &new);

    dbg!(&patch);

    assert_eq!(
        patch,
        vec![
            Patch::replace_node(
                Some(&"b"),
                TreePath::new(vec![0]),
                &i(vec![], vec![text("1")])
            ),
            Patch::replace_node(
                Some(&"b"),
                TreePath::new(vec![1]),
                &i(vec![], vec![])
            ),
        ],
    )
}

#[test]
fn add_children() {
    let old: Node<()> = div(vec![], vec![b(vec![], vec![])]); //{ <div> <b></b> </div> },
    let new = div(
        vec![],
        vec![
            b(vec![], vec![]),
            html_element(None, "new", vec![], vec![], false),
        ],
    ); //{ <div> <b></b> <new></new> </div> },
    assert_eq!(
        dbg!(diff(&old, &new)),
        vec![Patch::append_children(
            &"div",
            TreePath::new(vec![]),
            vec![&html_element(None, "new", vec![], vec![], false)]
        )],
        "Added a new node to the root node",
    )
}

#[test]
fn add_attributes() {
    let old: Node<()> = div(vec![], vec![]);
    let new = div(vec![id("hello")], vec![]);
    assert_eq!(
        diff(&old, &new),
        vec![Patch::add_attributes(
            &"div",
            TreePath::new(vec![]),
            vec![&id("hello")]
        )],
        "Add attributes",
    );

    let old: Node<()> = div(vec![id("foobar")], vec![]);
    let new = div(vec![id("hello")], vec![]);

    assert_eq!(
        diff(&old, &new),
        vec![Patch::add_attributes(
            &"div",
            TreePath::new(vec![]),
            vec![&id("hello")]
        )],
        "Change attribute",
    );
}

#[test]
fn remove_attributes() {
    let old: Node<()> = div(vec![id("hey-there")], vec![]);
    let new = div(vec![], vec![]);
    assert_eq!(
        diff(&old, &new),
        vec![Patch::remove_attributes(
            &"div",
            TreePath::new(vec![]),
            vec![&id("hey-there")]
        )],
        "Remove attributes",
    );
}

#[test]
fn change_attribute() {
    let old: Node<()> = div(vec![id("hey-there")], vec![]);
    let new = div(vec![id("changed")], vec![]);

    assert_eq!(
        diff(&old, &new),
        vec![Patch::add_attributes(
            &"div",
            TreePath::new(vec![]),
            vec![&id("changed")]
        )],
        "Add attributes",
    );
}

#[test]
fn replace_text_node() {
    let old: Node<()> = text("Old"); //{ Old },
    let new = text("New"); //{ New },

    assert_eq!(
        diff(&old, &new),
        vec![Patch::replace_node(
            None,
            TreePath::new(vec![]),
            &text("New")
        )],
        "ReplaceNode text node",
    );
}
