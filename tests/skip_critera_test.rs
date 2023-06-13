#![deny(warnings)]
use sauron::*;
use sauron::{html::attributes::*, html::*};

#[test]
fn must_skip_diff() {
    let old: Node<()> = div([skip_criteria("line1")], [text("old here")]);
    let new: Node<()> = div([skip_criteria("line1")], [text("new here")]);

    let patch = diff(&old, &new);
    dbg!(&patch);
    assert_eq!(patch, vec![]);
}
