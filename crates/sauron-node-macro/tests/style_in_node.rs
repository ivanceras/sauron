use sauron::{
    node,
    Node,
    Render,
};

#[test]
fn style_should_be_valid() {
    let node1: Node<()> = node!(<div style="border:1px solid green;"></div>);
    let expected = r#"<div style="border:1px solid green;"></div>"#;
    let mut buffer = String::new();
    node1.render(&mut buffer).expect("must have no error");
    assert_eq!(expected, buffer);
}
