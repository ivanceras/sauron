//! This contains a trait to be able to render
//! virtual dom into a writable buffer
//!
use crate::vdom::Style;
use crate::vdom::Value;
use crate::{
    vdom::GroupedAttributeValues,
    vdom::{Attribute, Element, Leaf, Node},
};
use std::fmt;

const DEFAULT_INDENT_SIZE: usize = 2;

/// add an indent if applicable
fn maybe_indent(buffer: &mut dyn fmt::Write, indent: usize, compressed: bool) -> fmt::Result {
    if !compressed {
        write!(
            buffer,
            "\n{}",
            " ".repeat(DEFAULT_INDENT_SIZE).repeat(indent)
        )?;
    }
    Ok(())
}

impl<MSG> Node<MSG> {
    // ISSUE: sublte difference in `render` and `render_to_string`:
    //  - flow content element such as span will treat the whitespace in between them as html text
    //  node
    //  Example:
    //  in `render`
    //  ```html
    //     <span>hello</span>
    //     <span> world</span>
    //  ```
    //     will displayed as "hello  world"
    //
    //  where us `render_to_string`
    //  ```html
    //  <span>hello</span><span> world</span>
    //  ```
    //  will result to a desirable output: "hello world"
    //
    /// render the node to a writable buffer
    pub fn render_with_indent(
        &self,
        buffer: &mut dyn fmt::Write,
        indent: usize,
        compressed: bool,
    ) -> fmt::Result {
        match self {
            Node::Element(element) => element.render_with_indent(buffer, indent, compressed),
            Node::Leaf(leaf) => leaf.render(buffer),
            Node::Fragment(nodes) => {
                for node in nodes {
                    node.render_with_indent(buffer, indent, compressed)?;
                }
                Ok(())
            }
            Node::NodeList(node_list) => {
                for node in node_list {
                    node.render_with_indent(buffer, indent, compressed)?;
                }
                Ok(())
            }
        }
    }

    /// render the node to a writable buffer
    pub fn render(&self, buffer: &mut dyn fmt::Write) -> fmt::Result {
        self.render_with_indent(buffer, 0, false)
    }

    /// no new_lines, no indents
    fn render_compressed(&self, buffer: &mut dyn fmt::Write) -> fmt::Result {
        self.render_with_indent(buffer, 0, true)
    }

    /// render compressed html to string
    pub fn render_to_string(&self) -> String {
        let mut buffer = String::new();
        self.render_compressed(&mut buffer).expect("must render");
        buffer
    }

    /// render to string with nice indention
    pub fn render_to_string_pretty(&self) -> String {
        let mut buffer = String::new();
        self.render(&mut buffer).expect("must render");
        buffer
    }
}

impl<MSG> Leaf<MSG> {
    /// render leaf nodes
    pub fn render(&self, buffer: &mut dyn fmt::Write) -> fmt::Result {
        match self {
            Leaf::Text(text) => {
                write!(buffer, "{text}")
            }
            Leaf::SafeHtml(html) => {
                //TODO: html escape this one
                write!(buffer, "{html}")
            }
            Leaf::Comment(comment) => {
                write!(buffer, "<!--{comment}-->")
            }
            Leaf::DocType(doctype) => {
                write!(buffer, "<!doctype {doctype}>")
            }
            Leaf::StatefulComponent(comp) => {
                write!(buffer, "<!-- stateful component -->")
            }
            Leaf::StatelessComponent(comp) => comp.view.render(buffer),
        }
    }
}

fn extract_inner_html<MSG>(merged_attributes: &[Attribute<MSG>]) -> String {
    merged_attributes
        .iter()
        .flat_map(|attr| {
            let GroupedAttributeValues {
                listeners: _,
                plain_values: _,
                styles: _,
                function_calls,
            } = Attribute::group_values(attr);

            if attr.name == "inner_html" {
                Value::merge_to_string(function_calls)
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

impl<MSG> Element<MSG> {
    /// render element nodes
    pub fn render_with_indent(
        &self,
        buffer: &mut dyn fmt::Write,
        indent: usize,
        compressed: bool,
    ) -> fmt::Result {
        write!(buffer, "<{}", self.tag())?;

        println!("original attributes: {:#?}", self.attributes());
        let merged_attributes: Vec<Attribute<MSG>> =
            Attribute::merge_attributes_of_same_name(self.attributes().iter());
        println!("merged_attributes: {:#?}", merged_attributes);

        for attr in &merged_attributes {
            write!(buffer, " ")?;
            attr.render(buffer)?;
        }

        if self.self_closing {
            write!(buffer, "/>")?;
        } else {
            write!(buffer, ">")?;
        }

        let children = self.children();
        let first_child = children.first();
        let is_first_child_text_node = first_child.map(|node| node.is_text()).unwrap_or(false);

        let is_lone_child_text_node = children.len() == 1 && is_first_child_text_node;

        // do not indent if it is only text child node
        if is_lone_child_text_node {
            first_child
                .unwrap()
                .render_with_indent(buffer, indent, compressed)?;
        } else {
            // otherwise print all child nodes with each line and indented
            for child in self.children() {
                maybe_indent(buffer, indent + 1, compressed)?;
                child.render_with_indent(buffer, indent + 1, compressed)?;
            }
        }

        // do not make a new line it if is only a text child node or it has no child nodes
        if !is_lone_child_text_node && !children.is_empty() {
            maybe_indent(buffer, indent, compressed)?;
        }

        let inner_html = extract_inner_html(&merged_attributes);
        if !inner_html.is_empty() {
            write!(buffer, "{inner_html}")?;
        }

        if !self.self_closing {
            write!(buffer, "</{}>", self.tag())?;
        }
        Ok(())
    }
}

impl<MSG> Attribute<MSG> {
    /// render attributes
    fn render(&self, buffer: &mut dyn fmt::Write) -> fmt::Result {
        let GroupedAttributeValues {
            listeners: _,
            plain_values,
            styles,
            function_calls: _,
        } = Attribute::group_values(self);

        println!("plain values: {:#?}", plain_values);
        println!("styles: {:#?}", styles);

        // These are attribute values which specifies the state of the element
        // regardless of it's value.
        // This is counter-intuitive to what we are trying to do, therefore
        // we use something that if the value is false, we skip the attribute from being part
        // of the render which then satisfies our intent to the the browser behavior.
        //
        // https://html.spec.whatwg.org/multipage/form-control-infrastructure.html#attr-fe-disabled
        let boolean_attributes = ["open", "checked", "disabled"];

        let bool_value: bool = plain_values
            .first()
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        // skip this attribute if the boolean attributes evaluates to false
        let should_skip_attribute = boolean_attributes.contains(self.name()) && !bool_value;

        if !should_skip_attribute {
            if let Some(merged_plain_values) = Value::merge_to_string(plain_values) {
                println!("values: {}", merged_plain_values);
                write!(buffer, "{}=\"{}\"", self.name(), merged_plain_values)?;
            }
            if let Some(merged_styles) = Style::merge_to_string(styles) {
                println!("merged styles: {}", merged_styles);
                write!(buffer, "{}=\"{}\"", self.name(), merged_styles)?;
            }
        }
        Ok(())
    }

    /// render compressed html to string
    pub fn render_to_string(&self) -> String {
        let mut buffer = String::new();
        self.render(&mut buffer).expect("must render");
        buffer
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::html::{attributes::*, *};

    #[test]
    fn test_render_comments() {
        let view: Node<()> = div(vec![], vec![comment("comment1"), comment("comment2")]);

        assert_eq!(
            view.render_to_string(),
            "<div><!--comment1--><!--comment2--></div>"
        );
    }

    #[test]
    fn test_render_text_siblings_should_be_separated_with_comments() {
        let view: Node<()> = div(vec![], vec![text("text1"), text("text2")]);

        assert_eq!(
            view.render_to_string(),
            "<div>text1<!--separator-->text2</div>"
        );
    }

    #[test]
    fn test_render_classes() {
        let view: Node<()> = div(vec![class("frame"), class("component")], vec![]);
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
