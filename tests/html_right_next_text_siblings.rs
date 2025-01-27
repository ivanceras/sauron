#![deny(warnings)]
use crate::vdom::TreePath;
use sauron::html::*;
use sauron::*;

#[test]
fn comments_next_to_each_other() {
    let old: Node<()> = div(
        vec![],
        vec![comment("hello"), comment("mordor"), comment("hi")],
    );
    let new: Node<()> = div(vec![], vec![comment("hello"), comment("world")]);

    let patch = diff(&old, &new).unwrap();
    println!("patch: {:#?}", patch);
    assert_eq!(
        patch,
        vec![
            Patch::replace_node(
                None,
                TreePath::new(vec![1]),
                vec![&comment("world".to_string())]
            ),
            Patch::remove_node(None, TreePath::new(vec![2]),)
        ]
    );
}
