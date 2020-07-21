//! This module parses literal html returns sauron dom tree

use html5ever::{
    local_name, namespace_url, ns, parse_document, parse_fragment,
    tendril::TendrilSink, QualName,
};
use markup5ever_rcdom::{Handle, NodeData, RcDom};
use sauron::{
    html::{
        attributes,
        attributes::{
            AttributeValue, Style, HTML_ATTRS, HTML_ATTRS_SPECIAL, HTML_STYLES,
        },
        tags::{
            HTML_TAGS, HTML_TAGS_NON_COMMON, HTML_TAGS_WITH_MACRO_NON_COMMON,
        },
        text,
    },
    mt_dom,
    mt_dom::element,
    svg::{
        attributes::{SVG_ATTRS, SVG_ATTRS_SPECIAL, SVG_ATTRS_XLINK},
        tags::{SVG_TAGS, SVG_TAGS_NON_COMMON, SVG_TAGS_SPECIAL},
    },
    Attribute, Node,
};
use std::{fmt, io};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Generic Error {0}")]
    Generic(String),
    #[error("{0}")]
    IoError(#[from] io::Error),
    #[error("{0}")]
    FmtError(#[from] fmt::Error),
}

fn match_tag(tag: &str) -> Option<&'static str> {
    HTML_TAGS
        .iter()
        .chain(HTML_TAGS_NON_COMMON.iter())
        .chain(HTML_TAGS_WITH_MACRO_NON_COMMON.iter())
        .chain(SVG_TAGS.iter())
        .chain(SVG_TAGS_NON_COMMON.iter())
        .find(|item| item.eq_ignore_ascii_case(&tag))
        .map(|item| *item)
        .or_else(|| {
            SVG_TAGS_SPECIAL
                .iter()
                .find(|(_func, item)| item.eq_ignore_ascii_case(&tag))
                .map(|(func, _item)| *func)
        })
}

fn match_attribute(key: &str) -> Option<&'static str> {
    HTML_ATTRS
        .iter()
        .chain(SVG_ATTRS.iter())
        .find(|att| att.eq_ignore_ascii_case(&key))
        .map(|att| *att)
        .or_else(|| {
            HTML_ATTRS_SPECIAL
                .iter()
                .chain(SVG_ATTRS_SPECIAL.iter())
                .chain(SVG_ATTRS_XLINK.iter())
                .find(|(_func, att)| att.eq_ignore_ascii_case(&key))
                .map(|(func, _att)| *func)
        })
}

fn match_style_name(key: &str) -> Option<&'static str> {
    HTML_STYLES
        .iter()
        .find(|name| name.eq_ignore_ascii_case(&key))
        .map(|name| *name)
}

pub fn match_attribute_function(key: &str) -> Option<&'static str> {
    HTML_ATTRS
        .iter()
        .chain(SVG_ATTRS.iter())
        .find(|att| att.eq_ignore_ascii_case(key))
        .map(|att| *att)
        .or_else(|| {
            HTML_ATTRS_SPECIAL
                .iter()
                .chain(SVG_ATTRS_SPECIAL.iter())
                .chain(SVG_ATTRS_XLINK.iter())
                .find(|(func, _att)| func.eq_ignore_ascii_case(key))
                .map(|(func, _att)| *func)
        })
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
                Some(element(html_tag, attributes, children_nodes))
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
