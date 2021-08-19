//! This module parses literal html returns sauron dom tree

use html5ever::{
    local_name, namespace_url, ns, parse_document, parse_fragment,
    tendril::TendrilSink, QualName,
};
use markup5ever_rcdom::{Handle, NodeData, RcDom};
use once_cell::sync::Lazy;
use sauron_core::{
    html::{
        attributes,
        attributes::{
            AttributeValue, Style, HTML_ATTRS, HTML_ATTRS_SPECIAL, HTML_STYLES,
        },
        html_element_self_closing,
        tags::{
            HTML_SC_TAGS, HTML_TAGS, HTML_TAGS_NON_COMMON,
            HTML_TAGS_WITH_MACRO_NON_COMMON,
        },
        text,
    },
    mt_dom,
    svg::{
        attributes::{SVG_ATTRS, SVG_ATTRS_SPECIAL, SVG_ATTRS_XLINK},
        tags::{SVG_TAGS, SVG_TAGS_NON_COMMON, SVG_TAGS_SPECIAL},
        SVG_NAMESPACE,
    },
    Attribute, Node,
};
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::{fmt, io};
use thiserror::Error;

static ALL_SVG_TAGS: Lazy<HashSet<&&'static str>> = Lazy::new(|| {
    HashSet::from_iter(
        SVG_TAGS
            .iter()
            .chain(SVG_TAGS_NON_COMMON.iter())
            .chain(SVG_TAGS_SPECIAL.iter().map(|(_func, t)| t)),
    )
});

/// All of the html tags, excluding the SVG tags.
/// This is mainly used for checking whether element should be
/// created with namespace or not.
///
/// False negatives are:
///    script; // this conflicts with html::script        , html::tags::script       > svg::tags::script
///    style; // conflics with html::attributes::style    , html::attributes::style  > svg::tags::style
///    text; // conflicts with html::text                 , html::text               > svg::tags::text
///    a;   // conflicts with html::a                     , html::tags::a            > svg::tags::a
///
/// If used inside an svg node, svg elements scuh as text, a, style, script will not work correcly
/// in client-side rendering.
/// However, in server-side rendering it will work just fine.
static ALL_HTML_TAGS: Lazy<HashSet<&&'static str>> = Lazy::new(|| {
    HashSet::from_iter(
        HTML_TAGS
            .iter()
            .chain(HTML_SC_TAGS.iter())
            .chain(HTML_TAGS_NON_COMMON.iter())
            .chain(HTML_TAGS_WITH_MACRO_NON_COMMON.iter()),
    )
});

static SELF_CLOSING_TAGS: Lazy<HashSet<&&'static str>> =
    Lazy::new(|| HashSet::from_iter(HTML_SC_TAGS.iter()));

static ALL_STYLES: Lazy<HashMap<&'static str, &'static str>> =
    Lazy::new(|| HashMap::from_iter(HTML_STYLES.iter().map(|i| *i)));

/// all the possible error when parsing html string
#[derive(Debug, Error)]
pub enum ParseError {
    /// generic parser error, expressed in string
    #[error("Generic Error {0}")]
    Generic(String),
    /// io error
    #[error("{0}")]
    IoError(#[from] io::Error),
    /// formatting error
    #[error("{0}")]
    FmtError(#[from] fmt::Error),
}

fn match_tag(tag: &str) -> Option<&'static str> {
    ALL_HTML_TAGS
        .iter()
        .chain(ALL_SVG_TAGS.iter())
        .find(|item| **item == &tag)
        .map(|item| **item)
}

fn match_attribute(key: &str) -> Option<&'static str> {
    HTML_ATTRS
        .iter()
        .chain(SVG_ATTRS.iter())
        .find(|att| *att == &key)
        .map(|att| *att)
        .or_else(|| {
            HTML_ATTRS_SPECIAL
                .iter()
                .chain(SVG_ATTRS_SPECIAL.iter())
                .chain(SVG_ATTRS_XLINK.iter())
                .find(|(_func, att)| att == &key)
                .map(|(func, _att)| *func)
        })
}

fn match_style_name(key: &str) -> Option<&'static str> {
    ALL_STYLES.get(key).map(|s| *s)
}

/// return the static str of this function name
pub fn match_attribute_function(key: &str) -> Option<&'static str> {
    HTML_ATTRS
        .iter()
        .chain(SVG_ATTRS.iter())
        .find(|att| *att == &key)
        .map(|att| *att)
        .or_else(|| {
            HTML_ATTRS_SPECIAL
                .iter()
                .chain(SVG_ATTRS_SPECIAL.iter())
                .chain(SVG_ATTRS_XLINK.iter())
                .find(|(func, _att)| func == &key)
                .map(|(func, _att)| *func)
        })
}

/// Find the namespace of this tag
/// if the arg tag is an SVG tag, return the svg namespace
/// html tags don't need to have namespace while svg does, otherwise it will not be properly
/// mounted into the DOM
/// # Examples
/// ```rust
/// use sauron_core::prelude::*;
/// use sauron_parse::tag_namespace;
///     assert_eq!(None, tag_namespace("div"));
///     assert_eq!(Some(SVG_NAMESPACE), tag_namespace("rect"));
/// ```
///
/// Limitations: `script`, `style`,and `a` used inside svg will return `None`, as these are also valid html tags.
pub fn tag_namespace(tag: &str) -> Option<&'static str> {
    let is_html = ALL_HTML_TAGS.contains(&tag);
    let is_svg = ALL_SVG_TAGS.contains(&tag);
    if !is_html {
        if is_svg {
            // we return the svg namespace only when the tag is not an html, but an svg tag
            // False negatives:
            // This means that script, style, a and title used inside in svg tag will not work
            // properly, since this 3 tags are valid html tags
            Some(SVG_NAMESPACE)
        } else {
            None
        }
    } else {
        None
    }
}

/// Returns true if this html tag is self closing
pub fn is_self_closing(tag: &str) -> bool {
    SELF_CLOSING_TAGS.contains(&tag)
}

fn extract_attributes<MSG>(
    attrs: &Vec<html5ever::Attribute>,
) -> Vec<Attribute<MSG>> {
    attrs
        .iter()
        .filter_map(|att| {
            let key = att.name.local.to_string();
            let value = att.value.to_string();
            if key == "style" {
                let styles = extract_styles(&value);
                Some(mt_dom::attr("style", AttributeValue::from_styles(styles)))
            } else if let Some(attr) = match_attribute(&key) {
                Some(attributes::attr(attr, value))
            } else {
                log::warn!("Not a standard html attribute: {}", key);
                None
            }
        })
        .collect()
}

/// extract the styles into an arry
/// example: display:flex; flex-direction: column;
fn extract_styles(style: &str) -> Vec<Style> {
    let mut extracted = vec![];
    println!("processing style: {}", style);
    let mut single_styles: Vec<&str> = style.split(";").collect();
    single_styles.retain(|item| !item.trim().is_empty());
    for single in single_styles {
        let key_value: Vec<&str> = single.split(":").collect();
        assert_eq!(key_value.len(), 2);
        let key = key_value[0].trim();
        let value = key_value[1].trim();
        println!("style   [{}] = [{}]", key, value);
        if let Some(match_style) = match_style_name(key) {
            extracted.push(Style::new(match_style, value.to_string().into()));
        }
    }
    extracted
}

fn process_children<MSG>(node: &Handle) -> Vec<Node<MSG>> {
    node.children
        .borrow()
        .iter()
        .filter_map(|child_node| process_node(child_node))
        .collect()
}

fn process_node<MSG>(node: &Handle) -> Option<Node<MSG>> {
    match &node.data {
        NodeData::Text { ref contents } => {
            let text_content = contents.borrow().to_string();
            if text_content.trim().is_empty() {
                None
            } else {
                Some(text(text_content))
            }
        }

        NodeData::Element {
            ref name,
            ref attrs,
            ..
        } => {
            let tag = name.local.to_string();
            if let Some(html_tag) = match_tag(&tag) {
                let children_nodes = process_children(node);
                let attributes = extract_attributes(&attrs.borrow());
                let is_self_closing = HTML_SC_TAGS.contains(&html_tag);
                Some(html_element_self_closing(
                    html_tag,
                    attributes,
                    children_nodes,
                    is_self_closing,
                ))
            } else {
                log::warn!("Invalid tag: {}", tag);
                None
            }
        }
        NodeData::Document => {
            let mut children_nodes = process_children(node);
            let children_len = children_nodes.len();
            if children_len == 1 {
                Some(children_nodes.remove(0))
            } else if children_len == 2 {
                Some(children_nodes.remove(1))
            } else {
                None
            }
        }
        _ => None,
    }
}

/// Parse html string and convert it into sauron Node
pub fn parse<MSG>(html: &str) -> Result<Option<Node<MSG>>, ParseError> {
    let html_start = html.trim_start();
    let parser = if html_start.starts_with("<html")
        || html_start.starts_with("<!DOCTYPE")
    {
        parse_document(RcDom::default(), Default::default())
    } else {
        parse_fragment(
            RcDom::default(),
            Default::default(),
            QualName::new(None, ns!(html), local_name!("div")),
            vec![],
        )
    };

    let dom = parser.one(html);
    let node = process_node(&dom.document);
    Ok(node)
}

/// the document is not wrapped with html
pub fn parse_simple<MSG>(html: &str) -> Result<Vec<Node<MSG>>, ParseError> {
    if let Some(html) = parse(html)? {
        if let Some(element) = html.take_element() {
            assert_eq!(*element.tag(), "html");
            Ok(element.take_children())
        } else {
            Ok(vec![])
        }
    } else {
        Ok(vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sauron_core::{html::div, Render};

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
        let node: Vec<Node<()>> = parse_simple(html).expect("must parse");
        println!("node: {:#?}", node);
        let one = div(vec![], node);
        println!("one: {}", one.render_to_string());
    }

    #[test]
    fn tag_namespace_is_none_in_html_div() {
        assert_eq!(None, tag_namespace("div"));
        assert_eq!(None, tag_namespace("span"));
        assert_eq!(None, tag_namespace("a"));
        assert_eq!(None, tag_namespace("title"));
        assert_eq!(None, tag_namespace("style"));
        assert_eq!(None, tag_namespace("script"));
    }
    #[test]
    fn tag_namespace_in_svg_should_return_svg_namespace() {
        assert_eq!(Some(SVG_NAMESPACE), tag_namespace("svg"));
        assert_eq!(Some(SVG_NAMESPACE), tag_namespace("rect"));
        assert_eq!(Some(SVG_NAMESPACE), tag_namespace("line"));
        assert_eq!(Some(SVG_NAMESPACE), tag_namespace("circle"));
    }
}
