#![deny(warnings)]
use rphtml::config::ParseOptions;
use rphtml::parser::Doc;
use rphtml::parser::NodeType;
use rphtml::types::BoxDynError;
use sauron_core::{
    html::{attributes::*, lookup, *},
    vdom::Node,
};
use std::fmt;
use std::io;
use std::ops::Deref;

/// all the possible error when parsing html string
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    /// io error
    #[error("{0}")]
    IoError(#[from] io::Error),
    /// formatting error
    #[error("{0}")]
    FmtError(#[from] fmt::Error),
    /// rphtml specific error
    #[error("{0}")]
    RpHtmlError(#[from] BoxDynError),
    /// the tag is not a valid html
    #[error("Invalid tag: {0}")]
    InvalidTag(String),
}

/// the document is not wrapped with html
pub fn parse_html<MSG>(html: &str) -> Result<Option<Node<MSG>>, ParseError> {
    let doc = Doc::parse(
        html,
        ParseOptions {
            case_sensitive_tagname: false,
            allow_self_closing: true,
            auto_fix_unclosed_tag: true,
            auto_fix_unexpected_endtag: true,
            auto_fix_unescaped_lt: true,
        },
    )?;
    process_node(doc.get_root_node().borrow().deref())
}

fn process_node<MSG>(node: &rphtml::parser::Node) -> Result<Option<Node<MSG>>, ParseError> {
    let content = if let Some(content) = &node.content {
        let content = String::from_iter(content.iter());
        Some(content)
    } else {
        None
    };

    let mut child_nodes = if let Some(childs) = &node.childs {
        childs
            .iter()
            .flat_map(|child| process_node(child.borrow().deref()).ok().flatten())
            .collect()
    } else {
        vec![]
    };

    log::info!("accessing node_type here");
    match node.node_type {
        NodeType::Tag => {
            let tag = &node.meta.as_ref().expect("must have a tag");
            let tag_name = String::from_iter(tag.borrow().name.iter());
            if let Some(html_tag) = lookup::match_tag(&tag_name) {
                let is_self_closing = HTML_SC_TAGS.contains(&html_tag);
                let attributes: Vec<Attribute<MSG>> = tag
                    .borrow()
                    .attrs
                    .iter()
                    .filter_map(|attr| {
                        attr.key
                            .as_ref()
                            .map(|key| {
                                let key = String::from_iter(key.content.iter());
                                if let Some(attr_key) = lookup::match_attribute(&key) {
                                    let value = if let Some(value) = &attr.value {
                                        let value = String::from_iter(value.content.iter());
                                        AttributeValue::Simple(Value::from(value))
                                    } else {
                                        AttributeValue::Empty
                                    };
                                    Some(Attribute::new(None, attr_key, value))
                                } else {
                                    log::warn!("Not a standard html attribute: {}", key);
                                    None
                                }
                            })
                            .flatten()
                    })
                    .collect();

                Ok(Some(html_element(
                    None,
                    html_tag,
                    attributes,
                    child_nodes,
                    is_self_closing,
                )))
            } else {
                log::error!("invalid tag: {}", tag_name);
                Err(ParseError::InvalidTag(tag_name))
            }
        }
        NodeType::Text => {
            let content = content.expect("must have a content");
            Ok(Some(text(content)))
        }
        NodeType::AbstractRoot => {
            let child_nodes_len = child_nodes.len();
            match child_nodes_len {
                0 => Ok(None),
                1 => Ok(Some(child_nodes.remove(0))),
                _ => Ok(Some(node_list(child_nodes))),
            }
        }
        _ => Ok(None),
    }
}
