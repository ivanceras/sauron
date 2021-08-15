use pulldown_cmark::{Alignment, CodeBlockKind, Event, Options, Parser, Tag};
use sauron_core::{
    html::{
        attributes::{
            attr, checked, class, empty_attr, href, id, r#type, src, title,
        },
        *,
    },
    Node,
};
use std::{
    collections::{HashMap, HashSet},
    iter::FromIterator,
};

/// convert markdown text to Node
pub fn markdown<MSG>(md: &str) -> Node<MSG> {
    MarkdownParser::from_md(md).node()
}

/// process markdown with plugins
pub fn markdown_with_plugins<MSG>(
    md: &str,
    plugins: Plugins<MSG>,
) -> Node<MSG> {
    MarkdownParser::with_plugins(md, plugins).node()
}

/// parse a markdown string and convert it to Vec<Node>
pub fn render_markdown<MSG>(md: &str) -> Vec<Node<MSG>> {
    MarkdownParser::from_md(md).nodes()
}

/// collections of plugins to be run during the processing of markdown
#[allow(missing_debug_implementations)]
pub struct Plugins<MSG> {
    /// this a function where it is run when a code fence block is detected.
    /// Return an optional new node as a result.
    /// Should return none if the plugin can not process it.
    pub code_fence_processor:
        Option<fn(Option<&str>, &str) -> Option<Node<MSG>>>,
    /// this is executed for each node in the inline html
    /// Returns a derivative new node if applicable.
    /// Must return None if it the node isn't suitable to be processed.
    pub inline_html_processor: Option<fn(&Node<MSG>) -> Option<Node<MSG>>>,
}

impl<MSG> Default for Plugins<MSG> {
    fn default() -> Self {
        Self {
            code_fence_processor: None,
            inline_html_processor: None,
        }
    }
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
    #[allow(dead_code)]
    pub(crate) title: Option<String>,
    /// indicates if the text is inside a code block
    in_code_block: bool,
    /// current code fence, ie: it will be `js` if code block is: ```js
    code_fence: Option<String>,
    /// if in a table head , this will convert cell into either th or td
    in_table_head: bool,
    /// a flag if the previous event is inline html or not
    is_prev_inline_html: bool,
    plugins: Plugins<MSG>,
}

impl<MSG> Default for MarkdownParser<MSG> {
    fn default() -> Self {
        MarkdownParser {
            elems: vec![],
            spine: vec![],
            numbers: HashMap::new(),
            is_title_heading: false,
            title: None,
            in_code_block: false,
            code_fence: None,
            in_table_head: false,
            is_prev_inline_html: false,
            plugins: Default::default(),
        }
    }
}

impl<MSG> MarkdownParser<MSG> {
    /// create a markdown parser from a markdown content and the link_lookup replacement
    pub(crate) fn from_md(md: &str) -> Self {
        let mut md_parser = Self::default();
        md_parser.do_parse(md);
        md_parser
    }

    pub(crate) fn with_plugins(md: &str, plugins: Plugins<MSG>) -> Self {
        let mut md_parser = Self::default();
        md_parser.plugins = plugins;
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
            p(vec![], self.elems.clone())
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
                            vec![if let Some(code_fence_processor) =
                                self.plugins.code_fence_processor
                            {
                                let new_node = code_fence_processor(
                                    match self.code_fence {
                                        Some(ref code_fence) => {
                                            Some(code_fence)
                                        }
                                        None => None,
                                    },
                                    &content,
                                );
                                if let Some(new_node) = new_node {
                                    new_node
                                } else {
                                    // the code processor didn't detect it, turn it into a text
                                    // node
                                    text(content)
                                }
                            } else {
                                // no code fence processor just turn it into a text node
                                text(content)
                            }],
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
                        vec![r#type("checkbox"), checked(*value)],
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
            Tag::BlockQuote => blockquote(vec![], vec![]),
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
            Tag::Emphasis => em(vec![], vec![]),
            Tag::Strong => strong(vec![], vec![]),
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
                footer(
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
        let allowed_attributes = HashSet::from_iter(vec!["class"]);
        let clean_html = ammonia::Builder::default()
            .generic_attributes(allowed_attributes)
            .clean(&inline_html)
            .to_string();
        if let Ok(nodes) = sauron_parse::parse_simple(&clean_html) {
            for node in nodes {
                let new_node = self.run_inline_processor(node);
                self.add_node(new_node);
            }
        }
    }

    /// Run a plugin processor to elements in inline html
    /// if it the plugin produces a Node it will be return as is.
    /// If the plugin doesn't produce a node, return the current node
    fn run_inline_processor(&self, mut node: Node<MSG>) -> Node<MSG> {
        if let Some(inline_html_processor) = self.plugins.inline_html_processor
        {
            let new_node = inline_html_processor(&node);
            if let Some(new_node) = new_node {
                return new_node;
            } else {
                if let Some(element) = node.as_element_mut() {
                    let mut new_children = vec![];
                    for child in element.children.drain(..) {
                        let new_child = self.run_inline_processor(child);
                        new_children.push(new_child)
                    }
                    node.add_children_ref_mut(new_children);
                }
                node
            }
        } else {
            node
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

        let expected = "<p>\n    <h3>\n        <a href=\"https://github.com/markdown-it/markdown-it-footnote\" title=\"\">Footnotes</a>\n    </h3>\n    <p>\n        Footnote 1 link\n        <sup class=\"footnote-reference\">\n            <a href=\"#first\">1</a>\n        </sup>\n        .\n    </p>\n    <p>\n        Footnote 2 link\n        <sup class=\"footnote-reference\">\n            <a href=\"#second\">2</a>\n        </sup>\n        .\n    </p>\n    <p>\n        Inline footnote^\n        [\n        Text of inline footnote\n        ]\n         definition.\n    </p>\n    <p>\n        Duplicated footnote reference\n        <sup class=\"footnote-reference\">\n            <a href=\"#second\">2</a>\n        </sup>\n        .\n    </p>\n    <footer class=\"footnote-definition\" id=\"first\">\n        <sup class=\"footnote-label\">1</sup>\n        <p>\n            Footnote \n            <strong>can have markup</strong>\n        </p>\n    </footer>\n    <pre>\n        <code>and multiple paragraphs.\n</code>\n    </pre>\n    <footer class=\"footnote-definition\" id=\"second\">\n        <sup class=\"footnote-label\">2</sup>\n        <p>Footnote text.</p>\n    </footer>\n</p>";
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
"<p>\n    <p>\n        <a href=\"link.html\" title=\"\">Hello</a>\n        \n\n    </p>\n    <img src=\"img.jpeg\"/>\n</p>";
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
        let expected = r#"<p><h1>List</h1><ul><li>list 1</li><li>list 2</li><li>list 3<ul><li>sublist 1<ul><li>some other sublist A</li><li>some other sublist B</li></ul></li><li>sublist 2</li><li>sublist 3</li></ul></li></ul></p>"#;
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

        let expected = r#"<p>
    <h1>An h1 header</h1>
    <p>look like:</p>
    <ul>
        <li>this one</li>
        <li>that one</li>
        <li>the other one</li>
    </ul>
</p>"#;

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
        let expected = r#"<p>
    <p>
        <a href="http://dev.nodeca.com" title="">link text</a>
    </p>
    <p>
        <a href="http://nodeca.github.io/pica/demo/" title="title text!">link with title</a>
    </p>
</p>"#;

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
        let expected = r#"<p>
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
</p>"#;

        let mut buffer = String::new();
        view.render(&mut buffer).unwrap();
        println!("view: {}", buffer);
        assert_eq!(expected, buffer);
    }

    #[test]
    fn test_md_with_svgbob_processor() {
        let md = r#"
This is <b>Markdown</b> with some <i>funky</i> __examples__.
```bob
      .------.       +-------+
      | bob  | *---> | alice |
      `------'       +-------+
```
        "#;
        let node: Node<()> = markdown_with_plugins(
            md,
            Plugins {
                code_fence_processor: Some(|code_fence, code| {
                    if let Some(code_fence) = code_fence {
                        match code_fence {
                            "bob" => {
                                println!("processing svgbob...");
                                let svg =
                                    svgbob::to_svg_string_compressed(code);
                                Some(safe_html(svg))
                            }
                            _ => {
                                println!(
                                    "unrecognized code fence: {}",
                                    code_fence
                                );
                                None
                            }
                        }
                    } else {
                        println!("no code fence");
                        None
                    }
                }),

                ..Default::default()
            },
        );

        let html = node.render_to_string();
        println!("html: {}", html);
        dbg!(&html);
        let expected = r#"<p><p>This is Markdown<b></b> with some funky<i></i> <strong>examples</strong>.</p><pre><code class="bob"><svg xmlns="http://www.w3.org/2000/svg" width="248" height="64"><style>line, path, circle,rect,polygon{stroke:black;stroke-width:2;stroke-opacity:1;fill-opacity:1;stroke-linecap:round;stroke-linejoin:miter;}text{font-family:monospace;font-size:14px;}rect.backdrop{stroke:none;fill:white;}.broken{stroke-dasharray:8;}.filled{fill:black;}.bg_filled{fill:white;}.nofill{fill:white;}.end_marked_arrow{marker-end:url(#arrow);}.start_marked_arrow{marker-start:url(#arrow);}.end_marked_diamond{marker-end:url(#diamond);}.start_marked_diamond{marker-start:url(#diamond);}.end_marked_circle{marker-end:url(#circle);}.start_marked_circle{marker-start:url(#circle);}.end_marked_open_circle{marker-end:url(#open_circle);}.start_marked_open_circle{marker-start:url(#open_circle);}.end_marked_big_open_circle{marker-end:url(#big_open_circle);}.start_marked_big_open_circle{marker-start:url(#big_open_circle);}</style><defs><marker id="arrow" viewBox="-2 -2 8 8" refX="4" refY="2" markerWidth="7" markerHeight="7" orient="auto-start-reverse"><polygon points="0,0 0,4 4,2 0,0"></polygon></marker><marker id="diamond" viewBox="-2 -2 8 8" refX="4" refY="2" markerWidth="7" markerHeight="7" orient="auto-start-reverse"><polygon points="0,2 2,0 4,2 2,4 0,2"></polygon></marker><marker id="circle" viewBox="0 0 8 8" refX="4" refY="4" markerWidth="7" markerHeight="7" orient="auto-start-reverse"><circle cx="4" cy="4" r="2" class="filled"></circle></marker><marker id="open_circle" viewBox="0 0 8 8" refX="4" refY="4" markerWidth="7" markerHeight="7" orient="auto-start-reverse"><circle cx="4" cy="4" r="2" class="bg_filled"></circle></marker><marker id="big_open_circle" viewBox="0 0 8 8" refX="4" refY="4" markerWidth="7" markerHeight="7" orient="auto-start-reverse"><circle cx="4" cy="4" r="3" class="bg_filled"></circle></marker></defs><rect class="backdrop" x="0" y="0" width="248" height="64"></rect><rect x="52" y="8" width="56" height="32" class="solid nofill" rx="4"></rect><text x="66" y="28" >bob</text><rect x="172" y="8" width="64" height="32" class="solid nofill" rx="0"></rect><text x="186" y="28" >alice</text><circle cx="124" cy="24" r="3" class="filled"></circle><g><line x1="128" y1="24" x2="152" y2="24" class="solid"></line><polygon points="152,20 160,24 152,28" class="filled"></polygon></g></svg></code></pre></p>"#;
        assert_eq!(expected, html);
    }
}
