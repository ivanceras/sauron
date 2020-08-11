use sauron_core::{mt_dom::AttValue, prelude::*};
use std::{fmt, fmt::Write};

/// A trait to convert html string into sauron view syntax
pub trait ToSyntax {
    /// convert the html string into sauron view syntax
    fn to_syntax(
        &self,
        buffer: &mut dyn Write,
        use_macros: bool,
        indent: usize,
    ) -> fmt::Result;
}

impl<MSG: 'static> ToSyntax for Node<MSG> {
    fn to_syntax(
        &self,
        buffer: &mut dyn Write,
        use_macros: bool,
        indent: usize,
    ) -> fmt::Result {
        match self {
            Node::Text(text) => write!(buffer, "text(\"{}\")", text),
            Node::Element(element) => {
                element.to_syntax(buffer, use_macros, indent)
            }
        }
    }
}

impl<MSG: 'static> ToSyntax for Attribute<MSG> {
    fn to_syntax(
        &self,
        buffer: &mut dyn Write,
        use_macros: bool,
        indent: usize,
    ) -> fmt::Result {
        for att_value in self.value() {
            match att_value {
                AttValue::Plain(plain) => match plain {
                    AttributeValue::Simple(simple) => {
                        if let Some(_ns) = self.namespace() {
                            write!(
                                buffer,
                                "xlink_{}",
                                self.name().to_string(),
                            )?;
                            write!(buffer, "(")?;
                            simple.to_syntax(buffer, use_macros, indent)?;
                            write!(buffer, ")")?;
                        } else {
                            let matched_attribute_func =
                                sauron_parse::match_attribute_function(
                                    &self.name(),
                                )
                                .is_some();
                            if matched_attribute_func {
                                write!(buffer, "{}", self.name().to_string(),)?;
                                write!(buffer, "(")?;
                                simple.to_syntax(buffer, use_macros, indent)?;
                                write!(buffer, ")")?;
                            } else {
                                write!(
                                    buffer,
                                    r#"attr("{}","#,
                                    self.name().to_string(),
                                )?;
                                simple.to_syntax(buffer, use_macros, indent)?;
                                write!(buffer, ")")?;
                            }
                        }
                    }
                    AttributeValue::Style(styles_att) => {
                        write!(buffer, "style(\"")?;
                        for s_att in styles_att {
                            write!(buffer, "{};", s_att)?;
                        }
                        write!(buffer, "\")")?;
                    }
                    _ => (),
                },
                _ => (),
            }
        }
        Ok(())
    }
}

impl ToSyntax for Value {
    fn to_syntax(
        &self,
        buffer: &mut dyn Write,
        _use_macros: bool,
        _indent: usize,
    ) -> fmt::Result {
        match self.as_str() {
            Some(v_str) => {
                if let Ok(v_str) = v_str.parse::<f64>() {
                    write!(buffer, "{}", v_str)?;
                } else {
                    write!(buffer, "\"{}\"", v_str)?;
                }
            }
            None => (),
        }
        Ok(())
    }
}

impl<MSG: 'static> ToSyntax for Element<MSG> {
    fn to_syntax(
        &self,
        buffer: &mut dyn Write,
        use_macros: bool,
        indent: usize,
    ) -> fmt::Result {
        if use_macros {
            write!(buffer, "{}!(", self.tag())?;
        } else {
            write!(buffer, "{}(", self.tag())?;
        }
        if use_macros {
            write!(buffer, "[")?;
        } else {
            write!(buffer, "vec![")?;
        }
        for attr in self.get_attributes().iter() {
            attr.to_syntax(buffer, use_macros, indent)?;
            write!(buffer, ",")?;
        }
        write!(buffer, "],")?;
        if use_macros {
            write!(buffer, "[")?;
        } else {
            write!(buffer, "vec![")?;
        }
        let children = self.get_children();
        let first_child = children.get(0);
        let is_first_child_text_node =
            first_child.map(|node| node.is_text()).unwrap_or(false);

        let is_lone_child_text_node =
            children.len() == 1 && is_first_child_text_node;

        if is_lone_child_text_node {
            first_child.unwrap().to_syntax(buffer, use_macros, indent)?;
        } else {
            // otherwise print all child nodes with each line and indented
            for child in self.get_children() {
                write!(buffer, "\n{}", "    ".repeat(indent + 1))?;
                child.to_syntax(buffer, use_macros, indent + 1)?;
                write!(buffer, ",")?;
            }
        }
        // only make a new line if the child is not a text child node and if there are more than 1
        // child
        if !is_lone_child_text_node && !children.is_empty() {
            write!(buffer, "\n{}", "    ".repeat(indent))?;
        }
        write!(buffer, "])")?;
        Ok(())
    }
}
