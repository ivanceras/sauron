#![deny(warnings)]
use sauron::{
    html::{attributes::*, *},
    *,
};
use sauron_macro::node;

#[test]
fn test_disabled() {
    let view: Node<()> = node! { <input type="text" disabled=false/>};
    assert_eq!(view.render_to_string(), r#"<input type="text" />"#);
}

#[test]
fn test_checked() {
    let view: Node<()> = node! { <input type="text" checked=false/>};
    assert_eq!(view.render_to_string(), r#"<input type="text" />"#);
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
fn text_node_in_script_work_as_is() {
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
