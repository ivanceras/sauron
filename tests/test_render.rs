#![deny(warnings)]
use sauron::{
    html::{attributes::*, *},
    mt_dom::patch::*,
    *,
};

#[test]
fn test_render() {
    let view1: Node<()> = main(
        vec![class("container")],
        vec![article(vec![inner_html("<h1>Lorep Ipsum</h1>")], vec![])],
    );

    let expected = r#"<main class="container"><article ><h1>Lorep Ipsum</h1></article></main>"#;

    assert_eq!(expected, view1.render_to_string());
}

#[test]
fn test_self_closing_tag() {
    let view1: Node<()> = main(
        vec![class("container")],
        vec![
            input(vec![r#type("text")], vec![]),
            img(vec![src("image1.jpg")], vec![]),
        ],
    );

    let expected = r#"<main class="container"><input type="text"/><img src="image1.jpg"/></main>"#;

    assert_eq!(expected, view1.render_to_string());
}

#[test]
fn test_inner_html_patch() {
    let view1: Node<()> =
        main(vec![class("container")], vec![article(vec![], vec![])]);

    let view2: Node<()> = main(
        vec![class("container")],
        vec![article(vec![inner_html("<h1>Lorep Ipsum</h1>")], vec![])],
    );

    let patch = diff(&view1, &view2);
    assert_eq!(
        patch,
        vec![AddAttributes::new(
            &"article",
            TreePath::start_at(1, vec![0, 0]),
            vec![&inner_html("<h1>Lorep Ipsum</h1>")]
        )
        .into()]
    );
}

#[test]
fn test_inner_html_removed() {
    let view1: Node<()> = main(
        vec![class("container")],
        vec![article(vec![inner_html("<h1>Lorep Ipsum</h1>")], vec![])],
    );

    let view2: Node<()> =
        main(vec![class("container")], vec![article(vec![], vec![])]);

    let patch = diff(&view1, &view2);
    assert_eq!(
        patch,
        vec![RemoveAttributes::new(
            &"article",
            TreePath::start_at(1, vec![0, 0]),
            vec![&inner_html("<h1>Lorep Ipsum</h1>")]
        )
        .into()]
    );
}

#[test]
fn text_node_in_script_work_as_is() {
    use sauron::prelude::*;
    let serialized_state = "hello world";
    let view: Node<()> = node! {
        <html lang="en">
            <head>
               <meta http-equiv="Content-type" content="text/html; charset=utf-8"/>
                <script type="module">
                    {text!("
                              import init, {{ main }} from '/pkg/client.js';
                              async function start() {{
                                await init();
                                let app_state = String.raw`{}`;
                                main(app_state);
                              }}
                              start();
                        ",serialized_state)}
                </script>
            </head>
        </html>
    };
    let expected = r#"<html lang="en"><head><meta http-equiv="Content-type" content="text/html; charset=utf-8"/><script type="module">
                              import init, { main } from '/pkg/client.js';
                              async function start() {
                                await init();
                                let app_state = String.raw`hello world`;
                                main(app_state);
                              }
                              start();
                        </script></head></html>"#;
    let result = view.render_to_string();
    println!("result: {}", result);
    assert_eq!(expected, result)
}
