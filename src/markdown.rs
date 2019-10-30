use crate::{
    html::{
        attributes::*,
        *,
    },
    Node,
};
/// Original author of this code is [Nathan Ringo](https://github.com/remexre)
/// Source: https://github.com/acmumn/mentoring/blob/master/web-client/src/view/markdown.rs
use pulldown_cmark::{
    Alignment,
    Event,
    Options,
    Parser,
    Tag,
};

/// Renders a string of Markdown to HTML with the default options (footnotes
/// disabled, tables enabled).
pub fn render_markdown<MSG>(src: &str) -> Node<MSG>
where
    MSG: Clone + 'static,
{
    let mut elems = vec![];
    let mut spine = vec![];

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
                spine.push(make_tag(tag));
            }
            Event::End(tag) => {
                // TODO Verify stack end.
                let l = spine.len();
                assert!(l >= 1);
                let mut top = spine.pop().unwrap();
                if let Tag::CodeBlock(_) = tag {
                    top = pre(vec![], vec![top]);
                } else if let Tag::Table(aligns) = tag {
                    if let Some(element) = top.as_element_mut() {
                        for r in element.children.iter_mut() {
                            if let Some(r) = r.as_element_mut() {
                                for (i, c) in r.children.iter_mut().enumerate()
                                {
                                    if let Some(tag) = c.as_element_mut() {
                                        match aligns[i] {
                                            Alignment::None => {}
                                            Alignment::Left => {
                                                tag.add_attributes(vec![class(
                                                    "text-left",
                                                )])
                                            }
                                            Alignment::Center => {
                                                tag.add_attributes(vec![class(
                                                    "text-center",
                                                )])
                                            }
                                            Alignment::Right => {
                                                tag.add_attributes(vec![class(
                                                    "text-right",
                                                )])
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else if let Tag::TableHead = tag {
                    if let Some(element) = top.as_element_mut() {
                        for c in element.children.iter_mut() {
                            if let Some(tag) = c.as_element_mut() {
                                tag.tag = "th";
                                tag.add_attributes(vec![attr("scope", "col")]);
                            }
                        }
                    }
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
            Event::Text(content) => add_child!(text(content)),
            Event::SoftBreak => add_child!(text("\n")),
            Event::HardBreak => add_child!(br(vec![], vec![])),
            Event::InlineHtml(inline) => {
                println!("inline html: {}", inline);
            }
            _ => println!("Unknown event: {:#?}", ev),
        }
    }

    if elems.len() == 1 {
        elems.pop().unwrap()
    } else {
        div(vec![], elems)
    }
}

fn make_tag<MSG>(t: Tag) -> Node<MSG>
where
    MSG: Clone + 'static,
{
    match t {
        Tag::Paragraph => p(vec![], vec![]),
        Tag::Rule => hr(vec![], vec![]),
        Tag::Header(n) => {
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
        Tag::CodeBlock(lang) => {
            code(
                vec![classes_flag(vec![
                    ("html-language", lang.as_ref() == "html"),
                    ("rust-language", lang.as_ref() == "rust"),
                    ("java-language", lang.as_ref() == "java"),
                    ("c-language", lang.as_ref() == "c-language"),
                ])],
                vec![],
            )
        }
        Tag::List(None) => ul(vec![], vec![]),
        Tag::List(Some(1)) => ol(vec![], vec![]),
        Tag::List(Some(ref start)) => ol(vec![attr("start", *start)], vec![]),
        Tag::Item => li(vec![], vec![]),
        Tag::Table(_) => table(vec![class("table")], vec![]),
        Tag::TableHead => tr(vec![], vec![]),
        Tag::TableRow => tr(vec![], vec![]),
        Tag::TableCell => td(vec![], vec![]),
        Tag::Emphasis => span(vec![class("font-italic")], vec![]),
        Tag::Strong => span(vec![class("font-weight-bold")], vec![]),
        // TODO: parse the html block and convert to sauron node
        Tag::HtmlBlock => div(vec![], vec![]),
        Tag::Strikethrough => s(vec![], vec![]),
        Tag::Link(_, ref _href, ref _title) => {
            a(
                vec![href(_href.to_string()), title(_title.to_string())],
                vec![],
            )
        }
        Tag::Image(_, ref _src, ref _title) => {
            img(
                vec![src(_src.to_string()), title(_title.to_string())],
                vec![],
            )
        }
        Tag::FootnoteDefinition(ref _footnote_id) => span(vec![], vec![]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_md_with_html() {
        let md = r#"
[Hello](link.html)
<img src="img.jpeg"/>"#;

        let expected =
            "<p>\n    <a href=\"link.html\" title=\"\">Hello</a>\n    \n\n</p>";
        let view: Node<()> = render_markdown(md);
        assert_eq!(expected, view.to_string())
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
        let view: Node<()> = render_markdown(md);

        let expected = r#"<div>
    <h1>An h1 header</h1>
    <p>look like:</p>
    <ul>
        <li>this one</li>
        <li>that one</li>
        <li>the other one</li>
    </ul>
</div>"#;
        println!("{}", view.to_string());
        assert_eq!(expected, view.to_string());
    }

    #[test]
    fn test_md_links() {
        let md = r#"
[link text](http://dev.nodeca.com)

[link with title](http://nodeca.github.io/pica/demo/ "title text!")"#;
        let view: Node<()> = render_markdown(md);
        let expected = r#"<div>
    <p>
        <a href="http://dev.nodeca.com" title="">link text</a>
    </p>
    <p>
        <a href="http://nodeca.github.io/pica/demo/" title="title text!">link with title</a>
    </p>
</div>"#;
        assert_eq!(expected, view.to_string());
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
        let view: Node<()> = render_markdown(md);
        let expected = r#"<div>
    <h2>Tables</h2>
    <table class="table">
        <tr>
            <th class="text-left" scope="col">Option</th>
            <th class="text-right" scope="col">Description</th>
        </tr>
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
        assert_eq!(expected, view.to_string());
    }
}
