use super::*;
use sauron_vdom::{
    AttribValue,
    Text,
};

pub trait ToSyntax {
    fn to_syntax(&self, use_macros: bool, indent: usize) -> String;
}

impl ToSyntax for Node {
    fn to_syntax(&self, use_macros: bool, indent: usize) -> String {
        match self {
            Node::Text(text) => text.to_syntax(use_macros, indent),
            Node::Element(element) => element.to_syntax(use_macros, indent),
        }
    }
}

impl ToSyntax for Text {
    fn to_syntax(&self, _use_macros: bool, _indent: usize) -> String {
        format!("text(\"{}\")", self)
    }
}

impl ToSyntax for Attribute {
    fn to_syntax(&self, use_macros: bool, indent: usize) -> String {
        let mut buffer = String::new();
        if let Some(_ns) = self.namespace {
            buffer += &format!(
                r#"xlink_{}({}),"#,
                self.name.to_string(),
                self.value.to_syntax(use_macros, indent),
            );
        } else {
            buffer += &format!(
                r#"{}({}),"#,
                self.name.to_string(),
                self.value.to_syntax(use_macros, indent)
            );
        }
        buffer
    }
}

impl ToSyntax for AttribValue<Event, ()> {
    fn to_syntax(&self, _use_macros: bool, _indent: usize) -> String {
        match self.get_value() {
            Some(v_str) => {
                match v_str.as_str() {
                    Some(v_str) => {
                        if let Ok(v_str) = v_str.parse::<f64>() {
                            format!("{}", v_str)
                        } else {
                            format!("\"{}\"", v_str)
                        }
                    }
                    None => self.to_string(),
                }
            }
            None => self.to_string(),
        }
    }
}

impl ToSyntax for Element {
    fn to_syntax(&self, use_macros: bool, indent: usize) -> String {
        let mut buffer = String::new();
        if use_macros {
            buffer += &format!("{}!(", self.tag.to_string());
        } else {
            buffer += &format!("{}(", self.tag.to_string());
        }
        if use_macros {
            buffer += "[";
        } else {
            buffer += "vec![";
        }
        for attr in self.attributes().iter() {
            buffer += &format!("{}", attr.to_syntax(use_macros, indent));
        }
        buffer += "],";
        if use_macros {
            buffer += "[";
        } else {
            buffer += "vec![";
        }
        // do not indent if it is only text child node
        if self.is_children_a_node_text() {
            buffer += &self.children[0].to_syntax(use_macros, indent);
        } else {
            // otherwise print all child nodes with each line and indented
            for child in self.children.iter() {
                buffer += &format!(
                    "\n{}{},",
                    sauron_vdom::util::indent(indent + 1),
                    child.to_syntax(use_macros, indent + 1)
                );
            }
        }
        // do not make a new line it if is only a text child node or it has no child nodes
        if !(self.is_children_a_node_text() || self.children.is_empty()) {
            buffer += &format!("\n{}", sauron_vdom::util::indent(indent));
        }
        buffer += &format!("])");
        buffer
    }
}
