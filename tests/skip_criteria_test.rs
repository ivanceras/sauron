#![deny(warnings)]
use sauron::html::attributes::*;
use sauron::*;

#[test]
fn must_skip_diff() {
    let old: Node<()> = div([skip_criteria("line1")], [text("old here")]);
    let new: Node<()> = div([skip_criteria("line1")], [text("new here")]);

    let patch = diff(&old, &new);
    dbg!(&patch);
    assert_eq!(patch, vec![]);
}

#[test]
fn must_skip_diff_2() {
    let old: Node<()> = div([skip_criteria(1000)], [text("Regardless of")]);
    let new: Node<()> = div([skip_criteria(1000)], [text("the difference here")]);

    let patch = diff(&old, &new);
    dbg!(&patch);
    assert_eq!(patch, vec![]);
}

#[test]
fn must_diff() {
    let old: Node<()> = div([skip_criteria(1000)], [text("Regardless of")]);
    let new: Node<()> = div([skip_criteria(1001)], [text("the difference here")]);

    let patch = diff(&old, &new);
    dbg!(&patch);
    assert_eq!(patch, vec![Patch::add_attributes(&"div", TreePath::new([]), &[skip_criteria(1001)]),
        Patch::replace_node(None, TreePath::new([0]), &[text("the difference here")])
    ]);
}
