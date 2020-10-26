use pulldown_cmark::{Alignment, CodeBlockKind, Event, Options, Parser, Tag};
use sauron_core::{
    html::{
        attributes::{
            attr, checked, class, empty_attr, href, id, src, title, type_,
        },
        *,
    },
    Node,
};
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    iter::FromIterator,
};

/// convert markdown text to Node
pub fn markdown<MSG>(md: &str) -> Node<MSG> {
    MarkdownParser::from_md(md).node()
}

/// parse a markdown string and convert it to Vec<Node>
pub fn render_markdown<MSG>(md: &str) -> Vec<Node<MSG>> {
    MarkdownParser::from_md(md).nodes()
}

pub(crate) struct MarkdownParser<MSG> {
    /// contains the top level elements
    elems: Vec<Node<MSG>>,
    /// the elements that are processed
    /// the top of this element is the currently being processed on
    spine: Vec<Node<MSG>>,
    numbers: HashMap<String, usize>,
    /// if h1 is encountered
    is_title_heading: bool,
    /// if a text inside an h1 is encountered
    pub(crate) title: Option<String>,
    /// indicates if the text is inside a code block
    in_code_block: bool,
    /// current code fence
    code_fence: Option<String>,
    /// if in a table head , this will convert cell into either th or td
    in_table_head: bool,
    /// a flag if the previous event is inline html or not
    is_prev_inline_html: bool,
}

impl<MSG> MarkdownParser<MSG> {
    /// create a markdown parser from a markdown content and the link_lookup replacement
    pub(crate) fn from_md(md: &str) -> Self {
        let mut md_parser = MarkdownParser {
            elems: vec![],
            spine: vec![],
            numbers: HashMap::new(),
            is_title_heading: false,
            title: None,
            in_code_block: false,
            code_fence: None,
            in_table_head: false,
            is_prev_inline_html: false,
        };
        md_parser.do_parse(md);
        md_parser
    }

    /// Add a child node to the previous encountered element.
    /// if spine is empty, add it to the top level elements
    fn add_node(&mut self, child: Node<MSG>) {
        if !self.spine.is_empty() {
            let spine_len = self.spine.len();
            self.spine[spine_len - 1]
                .as_element_mut()
                .expect("expecting an element")
                .add_children(vec![child]);
        } else {
            self.elems.push(child);
        }
    }

    /// return the top-level elements
    pub(crate) fn nodes(&self) -> Vec<Node<MSG>> {
        self.elems.clone()
    }

    /// return 1 node, wrapping the the top-level node where there are more than 1.
    pub(crate) fn node(&self) -> Node<MSG> {
        if self.elems.len() == 1 {
            self.elems[0].clone()
        } else {
            div(vec![], self.elems.clone())
        }
    }

    fn is_inline_html(ev: &Event) -> bool {
        match ev {
            Event::Html(_) => true,
            _ => false,
        }
    }

    /// start parsing the markdown source
    fn do_parse(&mut self, src: &str) {
        // inline html accumulator
        let mut inline_html = String::new();
        let mut options = Options::empty();
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TASKLISTS);

        for ev in Parser::new_ext(src, options) {
            match ev {
                // create a tag and push it to the spine
                Event::Start(ref tag) => {
                    let start = self.make_tag(&tag);
                    self.spine.push(start);
                }
                Event::Text(ref content) => {
                    if self.is_title_heading {
                        self.title = Some(content.to_string());
                    }
                    if self.in_code_block {
                        self.add_node(code(
                            vec![
                                if let Some(ref code_fence) = self.code_fence {
                                    class(code_fence)
                                } else {
                                    empty_attr()
                                },
                            ],
                            vec![text(content)],
                        ));
                    } else {
                        let content = ammonia::clean(&*content);
                        self.add_node(text(content));
                    }
                }
                Event::SoftBreak => self.add_node(text("\n")),
                Event::HardBreak => self.add_node(br(vec![], vec![])),
                Event::Code(ref code_str) => {
                    let code_str = ammonia::clean(&*code_str);
                    self.add_node(code(vec![], vec![text(code_str)]))
                }
                // ISSUE: html is called for each encountered html tags
                // this needs to be accumulated before it can be parse into actual node
                Event::Html(ref html) => {
                    // accumulate the inline html
                    inline_html += &html;
                }
                Event::FootnoteReference(ref name) => {
                    let len = self.numbers.len() + 1;
                    let number: usize =
                        *self.numbers.entry(name.to_string()).or_insert(len);
                    self.add_node(sup(
                        vec![class("footnote-reference")],
                        vec![a(
                            vec![href(format!("#{}", name))],
                            vec![text(number)],
                        )],
                    ));
                }
                Event::Rule => {
                    self.add_node(hr(vec![], vec![]));
                }
                Event::TaskListMarker(ref value) => {
                    self.add_node(input(
                        vec![type_("checkbox"), checked(*value)],
                        vec![],
                    ));
                }
                // end event
                Event::End(ref tag) => self.close_tag(&tag),
            }
            // if inline html is done, process it
            if self.is_prev_inline_html && !Self::is_inline_html(&ev) {
                // not inline html anymore
                self.process_inline_html(&inline_html);
                inline_html.clear();
            }
            self.is_prev_inline_html = Self::is_inline_html(&ev);
        }
        // unprocessed inline html, happens if there is only inline html
        if !inline_html.is_empty() {
            self.process_inline_html(&inline_html);
            inline_html.clear();
        }
    }

    fn make_tag(&mut self, tag: &Tag) -> Node<MSG> {
        match tag {
            Tag::Paragraph => p(vec![], vec![]),
            Tag::Heading(n) => {
                assert!(*n > 0);
                assert!(*n < 7);
                match n {
                    1 => {
                        self.is_title_heading = true;
                        h1(vec![], vec![])
                    }
                    2 => h2(vec![], vec![]),
                    3 => h3(vec![], vec![]),
                    4 => h4(vec![], vec![]),
                    5 => h5(vec![], vec![]),
                    6 => h6(vec![], vec![]),
                    _ => unreachable!(),
                }
            }
            Tag::BlockQuote => blockquote(vec![class("blockquote")], vec![]),
            Tag::CodeBlock(codeblock) => {
                self.in_code_block = true;
                match codeblock {
                    CodeBlockKind::Indented => {
                        self.code_fence = None;
                        pre(vec![], vec![])
                    }
                    CodeBlockKind::Fenced(fence) => {
                        self.code_fence = Some(fence.to_string());
                        pre(vec![], vec![])
                    }
                }
            }
            Tag::List(None) => ul(vec![], vec![]),
            Tag::List(Some(1)) => ol(vec![], vec![]),
            Tag::List(Some(ref start)) => {
                ol(vec![attr("start", *start)], vec![])
            }
            Tag::Item => li(vec![], vec![]),
            Tag::Table(_alignment) => table(vec![], vec![]),
            Tag::TableHead => {
                self.in_table_head = true;
                tr(vec![], vec![])
            }
            Tag::TableRow => {
                self.in_table_head = false;
                tr(vec![], vec![])
            }
            Tag::TableCell => {
                if self.in_table_head {
                    th(vec![], vec![])
                } else {
                    td(vec![], vec![])
                }
            }
            Tag::Emphasis => span(vec![class("font-italic")], vec![]),
            Tag::Strong => span(vec![class("font-weight-bold")], vec![]),
            Tag::Strikethrough => s(vec![], vec![]),
            // replace links using the link_lookup
            Tag::Link(_link_type, ref link_href, ref link_title) => a(
                vec![
                    href(link_href.to_string()),
                    title(link_title.to_string()),
                ],
                vec![],
            ),
            Tag::Image(_link_type, ref image_src, ref image_title) => img(
                vec![
                    src(image_src.to_string()),
                    title(image_title.to_string()),
                ],
                vec![],
            ),
            Tag::FootnoteDefinition(name) => {
                let len = self.numbers.len() + 1;
                let number =
                    self.numbers.entry(name.to_string()).or_insert(len);
                div(
                    vec![class("footnote-definition"), id(name.to_string())],
                    vec![sup(
                        vec![class("footnote-label")],
                        vec![text(number)],
                    )],
                )
            }
        }
    }

    fn close_tag(&mut self, tag: &Tag) {
        let spine_len = self.spine.len();
        assert!(spine_len >= 1);
        let mut top = self.spine.pop().expect("must have one element");

        match tag {
            Tag::Heading(1) => self.is_title_heading = false,
            Tag::CodeBlock(_) => self.in_code_block = false,
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
                                        Alignment::Left => tag.add_attributes(
                                            vec![class("text-left")],
                                        ),
                                        Alignment::Center => tag
                                            .add_attributes(vec![class(
                                                "text-center",
                                            )]),
                                        Alignment::Right => tag.add_attributes(
                                            vec![class("text-right")],
                                        ),
                                    }
                                }
                            }
                        }
                    }
                }
            }
            _ => (),
        }
        self.add_node(top);
    }

    fn process_inline_html(&mut self, inline_html: &str) {
        println!("processing inline html: {}", inline_html);
        let allowed_attributes = HashSet::from_iter(vec!["class"]);
        let clean_html = ammonia::Builder::default()
            .generic_attributes(allowed_attributes)
            .clean(&inline_html)
            .to_string();
        if let Ok(nodes) = sauron_parse::parse_simple(&clean_html) {
            for node in nodes {
                self.add_node(node);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sauron_core::Render;

    #[test]
    fn test_inline_htmls() {
        let md = r#"<article class="side-to-side">
    <div>
        This is div content1
    </div>
    <footer>
        This is footer
    </footer>
</article>"#;

        let view: Node<()> = markdown(md);

        let mut buffer = String::new();
        view.render(&mut buffer).unwrap();
        println!("view: {}", buffer);
        assert_eq!(md, buffer);
    }

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
"<div>\n    <p>\n        <a href=\"link.html\" title=\"\">Hello</a>\n        \n\n    </p>\n    <img src=\"img.jpeg\"/>\n</div>";
        let view: Node<()> = markdown(md);

        let mut buffer = String::new();
        view.render(&mut buffer).unwrap();
        println!("view: {}", buffer);
        assert_eq!(expected, buffer);
    }

    #[test]
    fn test_md_with_image() {
        let md = r#"
[Hello](link.html)
![](img.jpeg "Image title")"#;

        let expected =
            "<p>\n    <a href=\"link.html\" title=\"\">Hello</a>\n    \n\n    <img src=\"img.jpeg\" title=\"Image title\"/>\n</p>";
        let view: Node<()> = markdown(md);

        let mut buffer = String::new();
        view.render(&mut buffer).unwrap();
        println!("view: {}", buffer);
        assert_eq!(expected, buffer);
    }

    #[test]
    fn test_list() {
        let md = r#"
# List
- list 1
- list 2
- list 3
    - sublist 1
        - some other sublist A
        - some other sublist B
    - sublist 2
    - sublist 3
"#;
        let expected = r#"<div><h1>List</h1><ul><li>list 1</li><li>list 2</li><li>list 3<ul><li>sublist 1<ul><li>some other sublist A</li><li>some other sublist B</li></ul></li><li>sublist 2</li><li>sublist 3</li></ul></li></ul></div>"#;
        let view: Node<()> = markdown(md);
        let mut buffer = String::new();
        view.render_compressed(&mut buffer).unwrap();
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
    <table>
        <tr>
            <th class="text-left">Option</th>
            <th class="text-right">Description</th>
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

        let mut buffer = String::new();
        view.render(&mut buffer).unwrap();
        println!("view: {}", buffer);
        assert_eq!(expected, buffer);
    }
}
