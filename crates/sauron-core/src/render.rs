//! This contains a trait to be able to render
//! virtual dom into a writable buffer
//!
use crate::{
    html::attributes,
    Attribute,
    Element,
    Node,
};
use std::fmt;

/// render node, elements to a writable buffer
pub trait Render {
    /// render the node to a writable buffer
    fn render(&self, buffer: &mut dyn fmt::Write) -> fmt::Result {
        self.render_with_indent(buffer, 0, &mut Some(0), false)
    }

    /// no new_lines, no indents
    fn render_compressed(&self, buffer: &mut dyn fmt::Write) -> fmt::Result {
        self.render_with_indent(buffer, 0, &mut Some(0), true)
    }
    /// render instance to a writable buffer with indention
    /// node_idx is for debugging purposes
    fn render_with_indent(
        &self,
        buffer: &mut dyn fmt::Write,
        indent: usize,
        node_idx: &mut Option<usize>,
        compressed: bool,
    ) -> fmt::Result;

    /// render compressed html to string
    fn render_to_string(&self) -> String {
        let mut buffer = String::new();
        self.render_compressed(&mut buffer).expect("must render");
        buffer
    }
}

impl<MSG> Render for Node<MSG> {
    fn render_with_indent(
        &self,
        buffer: &mut dyn fmt::Write,
        indent: usize,
        node_idx: &mut Option<usize>,
        compressed: bool,
    ) -> fmt::Result {
        match self {
            Node::Element(element) => {
                element.render_with_indent(buffer, indent, node_idx, compressed)
            }
            Node::Text(text) => write!(buffer, "{}", text),
        }
    }
}

fn extract_inner_html<MSG>(merged_attributes: &[Attribute<MSG>]) -> String {
    log::trace!("extracting inner html");
    // get the func values such as inner html
    merged_attributes
        .iter()
        .flat_map(|attr| {
            let (_callbacks, _plain_values, func_values) =
                attributes::partition_callbacks_from_plain_and_func_calls(
                    &attr,
                );
            log::trace!("attr: {:?}", attr);

            if *attr.name() == "inner_html" {
                log::trace!("it is an inner html");
                attributes::merge_plain_attributes_values(&func_values)
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

impl<MSG> Render for Element<MSG> {
    fn render_with_indent(
        &self,
        buffer: &mut dyn fmt::Write,
        indent: usize,
        node_idx: &mut Option<usize>,
        compressed: bool,
    ) -> fmt::Result {
        write!(buffer, "<{}", self.tag())?;

        let ref_attrs: Vec<&Attribute<MSG>> =
            self.get_attributes().iter().map(|att| att).collect();
        let merged_attributes: Vec<Attribute<MSG>> =
            mt_dom::merge_attributes_of_same_name(&ref_attrs);

        for attr in &merged_attributes {
            write!(buffer, " ")?;
            attr.render_with_indent(buffer, indent, node_idx, compressed)?;
        }

        #[cfg(feature = "with-nodeidx")]
        if let Some(node_idx_) = node_idx {
            let node_idx_attr: Attribute<MSG> =
                crate::prelude::attr("node_idx", *node_idx_);
            write!(buffer, " ")?;
            node_idx_attr
                .render_with_indent(buffer, indent, node_idx, compressed)?;
        }
        if self.self_closing {
            write!(buffer, "/>")?;
        } else {
            write!(buffer, ">")?;
        }

        let children = self.get_children();
        let first_child = children.get(0);
        let is_first_child_text_node =
            first_child.map(|node| node.is_text()).unwrap_or(false);

        let is_lone_child_text_node =
            children.len() == 1 && is_first_child_text_node;

        // do not indent if it is only text child node
        if is_lone_child_text_node {
            node_idx.as_mut().map(|idx| *idx += 1);
            first_child
                .unwrap()
                .render_with_indent(buffer, indent, node_idx, compressed)?;
        } else {
            // otherwise print all child nodes with each line and indented
            for child in self.get_children() {
                node_idx.as_mut().map(|idx| *idx += 1);
                if !compressed {
                    write!(buffer, "\n{}", "    ".repeat(indent + 1))?;
                }
                child.render_with_indent(
                    buffer,
                    indent + 1,
                    node_idx,
                    compressed,
                )?;
            }
        }

        // do not make a new line it if is only a text child node or it has no child nodes
        if !is_lone_child_text_node && !children.is_empty() {
            if !compressed {
                write!(buffer, "\n{}", "    ".repeat(indent))?;
            }
        }

        let inner_html = extract_inner_html(&merged_attributes);
        log::trace!("got iner_html: {}", inner_html);
        if !inner_html.is_empty() {
            write!(buffer, "{}", inner_html)?;
        }

        if !self.self_closing {
            write!(buffer, "</{}>", self.tag())?;
        }
        Ok(())
    }
}

impl<MSG> Render for Attribute<MSG> {
    fn render_with_indent(
        &self,
        buffer: &mut dyn fmt::Write,
        _indent: usize,
        _node_idx: &mut Option<usize>,
        _compressed: bool,
    ) -> fmt::Result {
        let (_callbacks, plain_values, _func_values) =
            attributes::partition_callbacks_from_plain_and_func_calls(&self);
        if let Some(merged_plain_values) =
            attributes::merge_plain_attributes_values(&plain_values)
        {
            write!(buffer, "{}=\"{}\"", self.name(), merged_plain_values)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn test_render_classes() {
        let view: Node<()> =
            div(vec![class("frame"), class("component")], vec![]);
        let expected = r#"<div class="frame component"></div>"#;
        let mut buffer = String::new();
        view.render(&mut buffer).expect("must render");
        assert_eq!(expected, buffer);
    }

    #[test]
    fn test_render_class_flag() {
        let view: Node<()> = div(
            vec![
                class("frame"),
                classes_flag([("component", true), ("layer", false)]),
            ],
            vec![],
        );
        let expected = r#"<div class="frame component"></div>"#;
        let mut buffer = String::new();
        view.render(&mut buffer).expect("must render");
        assert_eq!(expected, buffer);
    }
}
