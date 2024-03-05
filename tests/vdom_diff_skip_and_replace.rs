#![deny(warnings)]
use sauron::vdom::{diff::diff, patch::*, *};

#[test]
fn force_replace() {
    let old: Node<()> = element("div", vec![attr("class", "[0]"), attr("id", "0")], vec![]);
    let new = element(
        "div",
        vec![attr("class", "[0]"), attr("id", "0"), attr("replace", true)],
        vec![],
    );

    let diff = diff(&old, &new);
    assert_eq!(
        diff,
        vec![Patch::replace_node(
            Some(&"div"),
            TreePath::new(vec![]),
            vec![&new]
        )],
    );
}

#[test]
fn force_skip() {
    let old: Node<()> = element("div", vec![attr("class", "[0]"), attr("id", "0")], vec![]);
    let new = element(
        "div",
        vec![attr("class", "[0]"), attr("id", "0"), attr("skip", true)],
        vec![],
    );

    let diff = diff(&old, &new);
    assert_eq!(diff, vec![],);
}

#[test]
fn skip_in_attribute() {
    let old: Node<()> = element("div", vec![attr("class", "[0]"), attr("id", "0")], vec![]);
    let new = element(
        "div",
        vec![attr("skip", true), attr("class", "[1]"), attr("id", "1")],
        vec![],
    );

    let diff = diff(&old, &new);
    assert_eq!(diff, vec![],);
}

#[test]
fn replace_true_in_attribute_must_replace_old_node_regardless() {
    let old: Node<()> = element("div", vec![attr("class", "[0]"), attr("id", "0")], vec![]);
    let new = element(
        "div",
        vec![attr("replace", true), attr("class", "[1]"), attr("id", "1")],
        vec![],
    );

    let diff = diff(&old, &new);
    assert_eq!(
        diff,
        vec![Patch::replace_node(
            Some(&"div"),
            TreePath::new(vec![]),
            vec![&new]
        )],
    );
}

#[test]
fn replace_and_skip_in_sub_nodes() {
    let old: Node<()> = element(
        "div",
        vec![attr("class", "[0]"), attr("id", "0")],
        vec![
            element(
                "div",
                vec![attr("class", "[0,0]"), attr("id", "1")],
                vec![
                    element(
                        "div",
                        vec![attr("class", "[0,0,0]"), attr("id", "2")],
                        vec![],
                    ),
                    element(
                        "div",
                        vec![attr("class", "[0,0,1]"), attr("id", "3")],
                        vec![],
                    ),
                ],
            ),
            element(
                "div",
                vec![attr("class", "[0,1]"), attr("id", "4")],
                vec![
                    element(
                        "div",
                        vec![attr("class", "[0,1,0]"), attr("id", "5")],
                        vec![],
                    ),
                    element(
                        "div",
                        vec![attr("class", "[0,1,1]"), attr("id", "6")],
                        vec![],
                    ),
                    element(
                        "div",
                        vec![attr("class", "[0,1,2]"), attr("id", "7")],
                        vec![],
                    ),
                ],
            ),
        ],
    );

    let new: Node<()> = element(
        "div",
        vec![attr("class", "[0]"), attr("id", "0")],
        vec![
            element(
                "div",
                vec![
                    attr("skip", true),
                    attr("class", "[0,0]-differs"),
                    attr("id", "1"),
                ],
                vec![
                    element(
                        "div",
                        vec![attr("class", "[0,0,0]"), attr("id", "2")],
                        vec![],
                    ),
                    element(
                        "div",
                        vec![attr("class", "[0,0,1]"), attr("id", "3")],
                        vec![],
                    ),
                ],
            ),
            element(
                "div",
                vec![
                    attr("replace", true),
                    attr("class", "[0,1]"),
                    attr("id", "4"),
                ],
                vec![
                    element(
                        "div",
                        vec![attr("class", "[0,1,0]"), attr("id", "5")],
                        vec![],
                    ),
                    element(
                        "div",
                        vec![attr("class", "[0,1,1]"), attr("id", "6")],
                        vec![],
                    ),
                    element(
                        "div",
                        vec![attr("class", "[0,1,2]"), attr("id", "7")],
                        vec![],
                    ),
                ],
            ),
        ],
    );

    let diff = diff(&old, &new);
    assert_eq!(
        diff,
        vec![Patch::replace_node(
            Some(&"div"),
            TreePath::new(vec![1]),
            vec![&element(
                "div",
                vec![
                    attr("replace", true),
                    attr("class", "[0,1]"),
                    attr("id", "4"),
                ],
                vec![
                    element(
                        "div",
                        vec![attr("class", "[0,1,0]"), attr("id", "5")],
                        vec![],
                    ),
                    element(
                        "div",
                        vec![attr("class", "[0,1,1]"), attr("id", "6")],
                        vec![],
                    ),
                    element(
                        "div",
                        vec![attr("class", "[0,1,2]"), attr("id", "7")],
                        vec![],
                    ),
                ],
            )]
        )],
    );
}
