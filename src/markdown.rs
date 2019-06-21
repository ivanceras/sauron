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
                .as_element()
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
                    top = pre([], [top]);
                } else if let Tag::Table(aligns) = tag {
                    if let Some(element) = top.as_element() {
                        for r in element.children.iter_mut() {
                            if let Some(r) = r.as_element() {
                                for (i, c) in r.children.iter_mut().enumerate()
                                {
                                    if let Some(tag) = c.as_element() {
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
                    if let Some(element) = top.as_element() {
                        for c in element.children.iter_mut() {
                            if let Some(tag) = c.as_element() {
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
                        .as_element()
                        .expect("expecting element")
                        .add_children(vec![top]);
                }
            }
            Event::Text(content) => add_child!(text(content)),
            Event::SoftBreak => add_child!(text("\n")),
            Event::HardBreak => add_child!(br([], [])),
            _ => println!("Unknown event: {:#?}", ev),
        }
    }

    if elems.len() == 1 {
        elems.pop().unwrap()
    } else {
        div([], elems)
    }
}

fn make_tag<MSG>(t: Tag) -> Node<MSG>
where
    MSG: Clone + 'static,
{
    match t {
        Tag::Paragraph => p([], []),
        Tag::Rule => hr([], []),
        Tag::Header(n) => {
            assert!(n > 0);
            assert!(n < 7);
            match n {
                1 => h1([], []),
                2 => h2([], []),
                3 => h3([], []),
                4 => h4([], []),
                5 => h5([], []),
                6 => h6([], []),
                _ => unreachable!(),
            }
        }
        Tag::BlockQuote => blockquote([class("blockquote")], []),
        Tag::CodeBlock(lang) => {
            code(
                [classes_flag([
                    ("html-language", lang.as_ref() == "html"),
                    ("rust-language", lang.as_ref() == "rust"),
                    ("java-language", lang.as_ref() == "java"),
                    ("c-language", lang.as_ref() == "c-language"),
                ])],
                [],
            )
        }
        Tag::List(None) => ul([], []),
        Tag::List(Some(1)) => ol([], []),
        Tag::List(Some(ref start)) => ol([attr("start", *start)], []),
        Tag::Item => li([], []),
        Tag::Table(_) => table([class("table")], []),
        Tag::TableHead => tr([], []),
        Tag::TableRow => tr([], []),
        Tag::TableCell => td([], []),
        Tag::Emphasis => span([class("font-italic")], []),
        Tag::Strong => span([class("font-weight-bold")], []),
        // TODO: parse the html block and convert to sauron node
        Tag::HtmlBlock => div([], []),
        Tag::Strikethrough => s([], []),
        Tag::Link(_, ref _href, ref _title) => {
            a([href(_href.to_string()), title(_title.to_string())], [])
        }
        Tag::Image(_, ref _src, ref _title) => {
            img([src(_src.to_string()), title(_title.to_string())], [])
        }
        Tag::FootnoteDefinition(ref _footnote_id) => span([], []),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
