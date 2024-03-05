use sauron::html::lookup::match_tag;
use sauron::prelude::Render;
use sauron::vdom::Node;
use sauron_html_parser::parse_html;

#[test]
fn should_match_tags() {
    assert_eq!(Some("div"), match_tag(&String::from("div")));
    assert_eq!(Some("svg"), match_tag(&String::from("svg")));
    assert_eq!(
        Some("color-profile"),
        match_tag(&String::from("color-profile"))
    );
}

#[test]
fn test_html_child() {
    let html = r#"<article class="side-to-side">
    <div>
        This is div content1
    </div>
    <footer>
        This is footer
    </footer>
</article>"#;
    let expected = "<article class=\"side-to-side\"><div>\n        This is div content1\n    </div><footer>\n        This is footer\n    </footer></article>";
    let node: Node<()> = parse_html(html).ok().flatten().expect("must parse");
    println!("node: {:#?}", node);
    println!("render: {}", node.render_to_string());
    assert_eq!(expected, node.render_to_string());
}

#[test]
fn test_node_list() {
    let html = r#"<!doctype html>
    <html>
        <body>This is body</body>
    </html>"#;
    let expected = "<html><body>This is body</body></html>";
    let node: Node<()> = parse_html(html).ok().flatten().expect("must parse");
    println!("node: {:#?}", node);
    println!("render: {}", node.render_to_string());
    assert_eq!(expected, node.render_to_string());
}
