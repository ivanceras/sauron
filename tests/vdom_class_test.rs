use sauron::vdom::{diff::*, patch::*, *};

#[test]
fn class_changed() {
    let old: Node<()> = element(
        "main",
        vec![attr("class", "class1")],
        vec![leaf("Content of class")],
    );

    let new: Node<()> = element(
        "main",
        vec![attr("class", "class2")],
        vec![leaf("Content of class")],
    );

    let diff = diff(&old, &new).unwrap();

    dbg!(&diff);

    assert_eq!(
        diff,
        vec![Patch::add_attributes(
            &"main",
            TreePath::new(vec![]),
            vec![&attr("class", "class2")]
        )]
    );
}

#[test]
fn parent_of_matching_keyed_are_ignored() {
    let old: Node<()> = element(
        "ul",
        [attr("class", "original")],
        [
            element("li", [attr("key", "0")], [leaf("text0")]),
            element("li", [attr("key", "1")], [leaf("text1")]),
            element("li", [attr("key", "2")], [leaf("text2")]),
        ],
    );

    let new: Node<()> = element(
        "ul",
        [attr("class", "changed")],
        [
            element("li", [attr("key", "0")], [leaf("text0")]),
            element("li", [attr("key", "1")], [leaf("text1")]),
            element("li", [attr("key", "2")], [leaf("text2")]),
        ],
    );

    let patches = diff(&old, &new).unwrap();

    assert_eq!(
        patches,
        vec![Patch::add_attributes(
            &"ul",
            TreePath::new(vec![]),
            vec![&attr("class", "changed")]
        )],
        "Should add the new attributes"
    );
}
