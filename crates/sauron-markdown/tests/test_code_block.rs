use sauron::prelude::*;
use sauron_markdown::markdown_with_plugins;
use sauron_markdown::Plugins;

#[test]
fn test_md_with_html_inline_processor() {
    let md = r#"
```rust
fn main(){
    println!("this is real code block here");
}
```
<pre>
    <code>
        struct Foo {
            int bar;
            date baz;
            string quux;
          };

          //somewhere in something
          Array&lt;Foo&gt; foos;
    </code>
</pre>
        "#;
    let plugins = Plugins {
        code_fence_processor: None,
        inline_html_processor: Some(|node| {
            if let Some(tag) = node.tag() {
                match *tag {
                    "code" => {
                        println!(
                            "---->>> YAYY we are looking for this code: {:#?}",
                            node
                        );
                        let children = node
                            .get_children()
                            .expect("code node must have children");
                        let mut buffer = String::new();
                        // we collect all the next
                        for child in children {
                            if let Some(text) = child.text() {
                                buffer += text;
                            }
                        }
                        let hl_code = code(
                            vec![class("highlighted")],
                            vec![
                                text(buffer),
                                p(vec![], vec![text("--highlighted already")]),
                            ],
                        );
                        Some(hl_code)
                    }
                    _ => {
                        println!("we don't process tag: {}", tag);
                        None
                    }
                }
            } else {
                println!("we dont process text nodes..");
                None
            }
        }),
    };
    let node: Node<()> = markdown_with_plugins(md, plugins);

    let html = node.render_to_string();
    println!("html: {}", html);
    dbg!(&html);
    let expected = r#"<p><pre><code class="rust">fn main(){
    println!("this is real code block here");
}
</code></pre><pre><code class="highlighted">
        struct Foo {
            int bar;
            date baz;
            string quux;
          };

          //somewhere in something
          Array<Foo> foos;
    <p>--highlighted already</p></code></pre></p>"#;
    assert_eq!(expected, html);
}
