use sauron_core::{
    diff,
    html::{attributes::*, events::*, *},
    mt_dom::comment,
    mt_dom::patch::*,
    Attribute, Node, *,
};

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
            ChangeComment::new(
                &"mordor".to_string(),
                TreePath::new(vec![0, 1]),
                &"world".to_string()
            )
            .into(),
            RemoveNode::new(None, TreePath::new(vec![0, 2]),).into()
        ]
    );
}
