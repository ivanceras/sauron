#![deny(warnings)]
use sauron::*;

#[test]
fn class_with_bool_value() {
    let old: Node<()> = div(vec![class(false)], vec![]);

    let new = div(vec![class(true)], vec![]);
    assert_eq!(
        diff(&old, &new).unwrap(),
        vec![Patch::add_attributes(
            &"div",
            TreePath::new(vec![]),
            vec![&class(true)]
        )],
        "Should add the new attributes"
    );
}

#[test]
fn parent_of_matching_keyed_are_ignored() {
    let old: Node<()> = ul(
        [class("original")],
        [
            li([key("0")], [text("text0")]),
            li([key("1")], [text("text1")]),
            li([key("2")], [text("text2")]),
        ],
    );

    let new = ul(
        [class("changed")],
        [
            li([key("0")], [text("text0")]),
            li([key("1")], [text("text1")]),
            li([key("2")], [text("text2")]),
        ],
    );
    assert_eq!(
        diff(&old, &new).unwrap(),
        vec![Patch::add_attributes(
            &"ul",
            TreePath::new(vec![]),
            vec![&class("changed")]
        )],
        "Should add the new attributes"
    );
}
