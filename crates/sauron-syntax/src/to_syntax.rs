use sauron::prelude::*;
use sauron_vdom::Text;
use std::{
    fmt,
    fmt::Write,
};

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
            Node::Text(text) => text.to_syntax(buffer, use_macros, indent),
            Node::Element(element) => {
                element.to_syntax(buffer, use_macros, indent)
            }
        }
    }
}

impl ToSyntax for Text {
    fn to_syntax(
        &self,
        buffer: &mut dyn Write,
        _use_macros: bool,
        _indent: usize,
    ) -> fmt::Result {
        write!(buffer, "text(\"{}\")", self)
    }
}

impl<MSG: 'static> ToSyntax for Attribute<MSG> {
    fn to_syntax(
        &self,
        buffer: &mut dyn Write,
        use_macros: bool,
        indent: usize,
    ) -> fmt::Result {
        let matched_attribute_func =
            sauron_parse::match_attribute_function(&self.name()).is_some();
        if let Some(_ns) = self.namespace() {
            if let Some(value) = self.get_value() {
                write!(buffer, "xlink_{}", self.name().to_string(),)?;
                write!(buffer, "(")?;
                value.to_syntax(buffer, use_macros, indent)?;
                write!(buffer, ")")?;
            }
        } else {
            if matched_attribute_func {
                if let Some(value) = self.get_value() {
                    write!(buffer, "{}", self.name().to_string(),)?;
                    write!(buffer, "(")?;
                    value.to_syntax(buffer, use_macros, indent)?;
                    write!(buffer, ")")?;
                }
            } else {
                if let Some(value) = self.get_value() {
                    write!(buffer, r#"attr("{}","#, self.name().to_string(),)?;
                    value.to_syntax(buffer, use_macros, indent)?;
                    write!(buffer, ")")?;
                }
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
            write!(buffer, "{}!(", self.tag.to_string())?;
        } else {
            write!(buffer, "{}(", self.tag.to_string())?;
        }
        if use_macros {
            write!(buffer, "[")?;
        } else {
            write!(buffer, "vec![")?;
        }
        for attr in self.attributes().iter() {
            attr.to_syntax(buffer, use_macros, indent)?;
            write!(buffer, ",")?;
        }
        write!(buffer, "],")?;
        if use_macros {
            write!(buffer, "[")?;
        } else {
            write!(buffer, "vec![")?;
        }
        // do not indent if it is only text child node
        if self.is_children_a_node_text() {
            &self.children[0].to_syntax(buffer, use_macros, indent)?;
        } else {
            // otherwise print all child nodes with each line and indented
            for child in self.children.iter() {
                write!(buffer, "\n{}", sauron_vdom::util::indent(indent + 1),)?;
                child.to_syntax(buffer, use_macros, indent + 1)?;
                write!(buffer, ",")?;
            }
        }
        // do not make a new line it if is only a text child node or it has no child nodes
        if !(self.is_children_a_node_text() || self.children.is_empty()) {
            write!(buffer, "\n{}", sauron_vdom::util::indent(indent))?;
        }
        write!(buffer, "])")?;
        Ok(())
    }
}
