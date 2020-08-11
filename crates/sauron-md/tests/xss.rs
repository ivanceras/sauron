use sauron_core::prelude::*;

#[test]
fn anchor() {
    let md = r#"
<a name="n" href="javascript:alert('xss')">*you*</a>
"#;
    let view: Node<()> = sauron_markdown::markdown(md);
    let mut buffer = String::new();
    view.render(&mut buffer).unwrap();
    println!("view: {}", buffer);
    assert_eq!(
        buffer,
        r#"<p>
    <a rel="noopener noreferrer"></a>
    <span class="font-italic">you</span>
</p>"#
    );
}

#[test]
fn blockqupte_xss() {
    let md = r#"
> hello<a name="n"
> href="javascript:alert('xss')">*you*</a>
"#;
    let view: Node<()> = sauron_markdown::markdown(md);
    let mut buffer = String::new();
    view.render(&mut buffer).unwrap();
    println!("view: {}", buffer);
    assert_eq!(
        buffer,
        r#"<blockquote class="blockquote">
    <p>
        hello
        <a rel="noopener noreferrer"></a>
        href="javascript:alert('xss')"&gt;
        <span class="font-italic">you</span>
    </p>
</blockquote>"#
    );
}
