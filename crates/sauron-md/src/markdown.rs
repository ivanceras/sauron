use pulldown_cmark::{Alignment, CodeBlockKind, Event, Options, Parser, Tag};
use sauron_core::{
    html::{
        attributes::{attr, checked, class, href, id, src, title, type_},
        *,
    },
    Node,
};
use std::collections::HashMap;

/// parse markdown and return sauron virtual Node
pub fn markdown<MSG>(src: &str) -> Node<MSG> {
    let mut elems = render_markdown(src);
    if elems.len() == 1 {
        elems.pop().unwrap()
    } else {
        div(vec![], elems)
    }
}

/// Renders a string of Markdown to HTML with the default options (footnotes
/// disabled, tables enabled).
pub fn render_markdown<'a, MSG>(src: &'a str) -> Vec<Node<MSG>> {
    let mut elems = vec![];
    let mut spine = vec![];
    let mut numbers: HashMap<String, usize> = HashMap::new();

    // Add a child to the previous encountered element
    macro_rules! add_child {
        ($child:expr) => {{
            let l = spine.len();
            assert_ne!(l, 0);
            spine[l - 1]
                .as_element_mut()
                .expect("expecting an element")
                .add_children(vec![$child]);
        }};
    }

    for ev in Parser::new_ext(src, Options::all()) {
        match ev {
            Event::Start(tag) => {
                spine.push(make_tag(tag, &mut numbers));
            }
            Event::Text(content) => {
                add_child!(text(content));
            }
            Event::SoftBreak => add_child!(text("\n")),
            Event::HardBreak => add_child!(br(vec![], vec![])),
            Event::Code(code_str) => {
                add_child!(code(vec![], vec![text(code_str)]))
            }
            Event::Html(html) => {
                if let Ok(nodes) = sauron_parse::parse_simple(&html) {
                    for node in nodes {
                        add_child!(node);
                    }
                }
            }
            Event::FootnoteReference(name) => {
                let len = numbers.len() + 1;
                let number = numbers.entry(name.to_string()).or_insert(len);
                add_child!(sup(
                    vec![class("footnote-reference")],
                    vec![a(
                        vec![href(format!("#{}", name))],
                        vec![text(number)]
                    )]
                ));
            }
            Event::Rule => {
                add_child!(hr(vec![], vec![]));
            }
            Event::TaskListMarker(value) => {
                add_child!(input(
                    vec![type_("checkbox"), checked(value)],
                    vec![]
                ));
            }
            Event::End(tag) => {
                let l = spine.len();
                assert!(l >= 1);
                let mut top = spine.pop().unwrap();
                match tag {
                    Tag::CodeBlock(_codeblock) => {
                        top = pre(vec![], vec![top]);
                    }
                    Tag::Table(aligns) => {
                        if let Some(element) = top.as_element_mut() {
                            for r in element.children_mut() {
                                if let Some(r) = r.as_element_mut() {
                                    for (i, c) in
                                        r.children_mut().iter_mut().enumerate()
                                    {
                                        if let Some(tag) = c.as_element_mut() {
                                            match aligns[i] {
                                                Alignment::None => {}
                                                Alignment::Left => tag
                                                    .add_attributes(vec![
                                                        class("text-left"),
                                                    ]),
                                                Alignment::Center => tag
                                                    .add_attributes(vec![
                                                        class("text-center"),
                                                    ]),
                                                Alignment::Right => tag
                                                    .add_attributes(vec![
                                                        class("text-right"),
                                                    ]),
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Tag::TableHead => {
                        if let Some(element) = top.as_element_mut() {
                            for c in element.children_mut() {
                                if let Some(tag) = c.as_element_mut() {
                                    tag.set_tag("th");
                                    tag.add_attributes(vec![attr(
                                        "scope", "col",
                                    )]);
                                }
                            }
                        }
                    }
                    _ => (),
                }
                if l == 1 {
                    elems.push(top);
                } else {
                    spine[l - 2]
                        .as_element_mut()
                        .expect("expecting element")
                        .add_children(vec![top]);
                }
            }
        }
    }
    elems
}

fn make_tag<MSG>(t: Tag, numbers: &mut HashMap<String, usize>) -> Node<MSG> {
    match t {
        Tag::Paragraph => p(vec![], vec![]),
        Tag::Heading(n) => {
            assert!(n > 0);
            assert!(n < 7);
            match n {
                1 => h1(vec![], vec![]),
                2 => h2(vec![], vec![]),
                3 => h3(vec![], vec![]),
                4 => h4(vec![], vec![]),
                5 => h5(vec![], vec![]),
                6 => h6(vec![], vec![]),
                _ => unreachable!(),
            }
        }
        Tag::BlockQuote => blockquote(vec![class("blockquote")], vec![]),
        Tag::CodeBlock(codeblock) => match codeblock {
            CodeBlockKind::Indented => code(vec![], vec![]),
            CodeBlockKind::Fenced(fence) => {
                code(vec![class(fence.to_string())], vec![])
            }
        },
        Tag::List(None) => ul(vec![], vec![]),
        Tag::List(Some(1)) => ol(vec![], vec![]),
        Tag::List(Some(ref start)) => ol(vec![attr("start", *start)], vec![]),
        Tag::Item => li(vec![], vec![]),
        Tag::Table(_alignment) => table(vec![class("table")], vec![]),
        Tag::TableHead => th(vec![], vec![]),
        Tag::TableRow => tr(vec![], vec![]),
        Tag::TableCell => td(vec![], vec![]),
        Tag::Emphasis => span(vec![class("font-italic")], vec![]),
        Tag::Strong => span(vec![class("font-weight-bold")], vec![]),
        Tag::Strikethrough => s(vec![], vec![]),
        Tag::Link(_, ref _href, ref _title) => a(
            vec![href(_href.to_string()), title(_title.to_string())],
            vec![],
        ),
        Tag::Image(_, ref _src, ref _title) => img(
            vec![src(_src.to_string()), title(_title.to_string())],
            vec![],
        ),
        Tag::FootnoteDefinition(name) => {
            let len = numbers.len() + 1;
            let number = *numbers.entry(name.to_string()).or_insert(len);
            div(
                vec![class("footnote-definition"), id(name.to_string())],
                vec![sup(vec![class("footnote-label")], vec![text(number)])],
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sauron_core::Render;

    #[test]
    fn source_code() {
        let md = r#"
```rust
    fn main(){
        println!("Hello world!");
    }
```
        "#;
        let expected = "<pre>\n    <code class=\"rust\">    fn main(){\n        println!(\"Hello world!\");\n    }\n</code>\n</pre>";
        let view: Node<()> = markdown(md);

        let mut buffer = String::new();
        view.render(&mut buffer).unwrap();
        println!("view: {}", buffer);
        assert_eq!(expected, buffer);
    }

    #[test]
    fn code() {
        let md = r#"
This is has some `code` and other..
        "#;
        let expected = "<p>\n    This is has some \n    <code>code</code>\n     and other..\n</p>";
        let view: Node<()> = markdown(md);

        let mut buffer = String::new();
        view.render(&mut buffer).unwrap();
        println!("view: {}", buffer);
        assert_eq!(expected, buffer);
    }

    #[test]
    fn footnotes() {
        let md = r#"
### [Footnotes](https://github.com/markdown-it/markdown-it-footnote)

Footnote 1 link[^first].

Footnote 2 link[^second].

Inline footnote^[Text of inline footnote] definition.

Duplicated footnote reference[^second].

[^first]: Footnote **can have markup**

    and multiple paragraphs.

[^second]: Footnote text.
        "#;

        let expected = "<div>\n    <h3>\n        <a href=\"https://github.com/markdown-it/markdown-it-footnote\" title=\"\">Footnotes</a>\n    </h3>\n    <p>\n        Footnote 1 link\n        <sup class=\"footnote-reference\">\n            <a href=\"#first\">1</a>\n        </sup>\n        .\n    </p>\n    <p>\n        Footnote 2 link\n        <sup class=\"footnote-reference\">\n            <a href=\"#second\">2</a>\n        </sup>\n        .\n    </p>\n    <p>\n        Inline footnote^\n        [\n        Text of inline footnote\n        ]\n         definition.\n    </p>\n    <p>\n        Duplicated footnote reference\n        <sup class=\"footnote-reference\">\n            <a href=\"#second\">2</a>\n        </sup>\n        .\n    </p>\n    <div class=\"footnote-definition\" id=\"first\">\n        <sup class=\"footnote-label\">1</sup>\n        <p>\n            Footnote \n            <span class=\"font-weight-bold\">can have markup</span>\n        </p>\n    </div>\n    <pre>\n        <code>and multiple paragraphs.\n</code>\n    </pre>\n    <div class=\"footnote-definition\" id=\"second\">\n        <sup class=\"footnote-label\">2</sup>\n        <p>Footnote text.</p>\n    </div>\n</div>";
        let view: Node<()> = markdown(md);

        let mut buffer = String::new();
        view.render(&mut buffer).unwrap();
        println!("view: {}", buffer);
        assert_eq!(expected, buffer);
    }

    #[test]
    fn test_md_with_html() {
        let md = r#"
[Hello](link.html)
<img src="img.jpeg"/>"#;

        let expected =
            "<p>\n    <a href=\"link.html\" title=\"\">Hello</a>\n    \n\n    <img src=\"img.jpeg\"></img>\n</p>";
        let view: Node<()> = markdown(md);

        let mut buffer = String::new();
        view.render(&mut buffer).unwrap();
        println!("view: {}", buffer);
        assert_eq!(expected, buffer);
    }

    #[test]
    fn test_md() {
        let md = r#"
An h1 header
============
look like:
  * this one
  * that one
  * the other one"#;
        let view: Node<()> = markdown(md);

        let expected = r#"<div>
    <h1>An h1 header</h1>
    <p>look like:</p>
    <ul>
        <li>this one</li>
        <li>that one</li>
        <li>the other one</li>
    </ul>
</div>"#;

        let mut buffer = String::new();
        view.render(&mut buffer).unwrap();
        println!("view: {}", buffer);
        assert_eq!(expected, buffer);
    }

    #[test]
    fn test_md_links() {
        let md = r#"
[link text](http://dev.nodeca.com)

[link with title](http://nodeca.github.io/pica/demo/ "title text!")"#;
        let view: Node<()> = markdown(md);
        let expected = r#"<div>
    <p>
        <a href="http://dev.nodeca.com" title="">link text</a>
    </p>
    <p>
        <a href="http://nodeca.github.io/pica/demo/" title="title text!">link with title</a>
    </p>
</div>"#;

        let mut buffer = String::new();
        view.render(&mut buffer).unwrap();
        println!("view: {}", buffer);
        assert_eq!(expected, buffer);
    }

    #[test]
    fn test_md_tables() {
        let md = r#"
## Tables

| Option | Description |
|:------ | -----------:|
| data   | path to data files to supply the data that will be passed into templates. |
| engine | engine to be used for processing templates. Handlebars is the default. |
| ext    | extension to be used for dest files. |
}
"#;
        let view: Node<()> = markdown(md);
        let expected = r#"<div>
    <h2>Tables</h2>
    <table class="table">
        <th>
            <th scope="col" class="text-left">Option</th>
            <th scope="col" class="text-right">Description</th>
        </th>
        <tr>
            <td class="text-left">data</td>
            <td class="text-right">path to data files to supply the data that will be passed into templates.</td>
        </tr>
        <tr>
            <td class="text-left">engine</td>
            <td class="text-right">engine to be used for processing templates. Handlebars is the default.</td>
        </tr>
        <tr>
            <td class="text-left">ext</td>
            <td class="text-right">extension to be used for dest files.</td>
        </tr>
        <tr>
            <td class="text-left">}</td>
            <td class="text-right"></td>
        </tr>
    </table>
</div>"#;

        let mut buffer = String::new();
        view.render(&mut buffer).unwrap();
        println!("view: {}", buffer);
        assert_eq!(expected, buffer);
    }
}
