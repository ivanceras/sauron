use crate::mt_dom::TreePath;
use sauron::prelude::*;

#[test]
fn comments_next_to_each_other() {
    let old: Node<()> = div(
        vec![],
        vec![comment("hello"), comment("mordor"), comment("hi")],
    );
    let new: Node<()> = div(vec![], vec![comment("hello"), comment("world")]);

    let patch = diff(&old, &new);
    println!("patch: {:#?}", patch);
    assert_eq!(
        patch,
        vec![
            Patch::replace_leaf(
                TreePath::new(vec![0, 1]),
                &leaf::comment("mordor".to_string()),
                &leaf::comment("world".to_string())
            ),
            Patch::remove_node(None, TreePath::new(vec![0, 2]),)
        ]
    );
}
