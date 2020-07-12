use crate::{
    util,
    vnode::attribute::Style,
    Attribute,
    Callback,
    Node,
    Value,
};
use std::{
    collections::HashMap,
    fmt,
    fmt::Write,
};

/// TODO: add styles as a field and will be aggregated with styles attribute
/// Represents the element of the virtual node
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Element<T, ATT, EVENT, MSG>
where
    MSG: 'static,
    EVENT: 'static,
    ATT: Clone,
{
    /// the element tag, such as div, a, button
    pub tag: T,
    /// attributes for this element
    pub attrs: Vec<Attribute<ATT, EVENT, MSG>>,
    /// children elements of this element
    pub children: Vec<Node<T, ATT, EVENT, MSG>>,
    /// namespace of this element,
    /// svg elements requires namespace to render correcly in the browser
    pub namespace: Option<&'static str>,
}

impl<T, ATT, EVENT, MSG> Element<T, ATT, EVENT, MSG>
where
    EVENT: 'static,
    MSG: 'static,
    ATT: PartialEq + Ord + ToString + Clone,
{
    /// creates an element node with the supplied tag
    pub fn with_tag(tag: T) -> Self {
        Element {
            tag,
            attrs: vec![],
            children: vec![],
            namespace: None,
        }
    }

    /// returns a refernce to the children of this node
    pub fn get_children(&self) -> &[Node<T, ATT, EVENT, MSG>] {
        &self.children
    }

    /// get the attributes that are events
    pub fn events(&self) -> Vec<&Attribute<ATT, EVENT, MSG>> {
        self.attrs
            .iter()
            .filter(|attr| attr.is_event_listener())
            .collect()
    }

    /// return the event as an attribute which matches the event name.
    pub fn get_event(&self, name: &ATT) -> Option<&Attribute<ATT, EVENT, MSG>> {
        self.events()
            .iter()
            .find(|event| event.name() == name)
            .copied()
    }

    /// return the attributes that are simple value or function call
    fn attributes_internal(&self) -> Vec<&Attribute<ATT, EVENT, MSG>> {
        self.attrs
            .iter()
            .filter(|attr| attr.is_value() || attr.is_func_call())
            .collect()
    }

    /// returns the only the attributes of this element
    /// Note: This does not include the events.
    /// If you need to access the events, use the `attrs` field directly.
    pub fn attributes(&self) -> Vec<Attribute<ATT, EVENT, MSG>> {
        let names = self.get_attributes_name_and_ns();
        let mut attributes = vec![];
        for (name, namespace) in names {
            if let Some(value) = self.get_attr_value(&name) {
                attributes.push(Attribute {
                    namespace,
                    name: Some(name.clone()),
                    value: value.into(),
                });
            }
        }
        attributes
    }

    /// return all the attributes that match the name
    fn get_attributes_with_name(
        &self,
        key: &ATT,
    ) -> Vec<&Attribute<ATT, EVENT, MSG>>
    where
        ATT: PartialEq + Ord,
    {
        self.attributes_internal()
            .iter()
            .filter(|att| att.name() == key)
            .copied()
            .collect()
    }

    /// return all the style attributes
    fn get_styles(&self) -> Vec<&Style<ATT>> {
        self.attrs
            .iter()
            .filter_map(|att| att.get_styles())
            .flatten()
            .collect()
    }

    /// remove from the style attribute if it matches the style name
    pub fn remove_style(&mut self, style_name: &ATT) {
        self.attrs.iter_mut().for_each(|att| {
            att.get_styles_mut()
                .map(|styles| styles.retain(|style| style.name != *style_name));
        });
    }

    /// add a style attribute
    pub fn add_style<V: Into<Value>>(&mut self, style_name: ATT, value: V) {
        let style = Style::new(style_name, value.into());
        self.attrs.push(Attribute::from_styles(vec![style]));
    }

    /// remove the previous style and add this new one
    pub fn set_style<V: Into<Value>>(&mut self, style_name: ATT, value: V) {
        self.remove_style(&style_name);
        self.add_style(style_name, value);
    }

    /// get all the styles attribute and make a single attribute out of it
    pub fn aggregate_styles(&self) -> Option<Attribute<ATT, EVENT, MSG>> {
        let styles = self.get_styles();

        let mut style_names: Vec<&ATT> =
            styles.iter().map(|style| &style.name).collect();
        style_names.sort();
        style_names.dedup();

        if style_names.is_empty() {
            None
        } else {
            let mut map = HashMap::new();
            for style in styles {
                map.insert(style.name.to_string(), style.value.clone());
            }

            let mut new_styles = vec![];
            for name in style_names {
                if let Some(value) = map.get(&name.to_string()) {
                    new_styles.push(Style::new(name.clone(), value.clone()))
                }
            }
            Some(Attribute::from_styles(new_styles))
        }
    }

    /// returns the unique list attribute names and their corresponding namespace
    fn get_attributes_name_and_ns(&self) -> Vec<(&ATT, Option<&'static str>)>
    where
        ATT: PartialEq + Ord,
    {
        let mut names = self
            .attributes_internal()
            .iter()
            .map(|att| (att.name(), att.namespace))
            .collect::<Vec<_>>();
        names.sort();
        names.dedup();
        names
    }

    /// get all the attributes with the same name and merge their value
    pub fn get_attr_value(&self, key: &ATT) -> Option<Value> {
        let attrs = self.get_attributes_with_name(key);
        if !attrs.is_empty() {
            Some(Self::merge_attributes_values(attrs))
        } else {
            None
        }
    }

    /// merge this all attributes,
    /// this assumes that that this attributes has the same name
    fn merge_attributes_values(
        attrs: Vec<&Attribute<ATT, EVENT, MSG>>,
    ) -> Value {
        if attrs.len() == 1 {
            let one_value =
                attrs[0].value.get_value().expect("Should have a value");
            one_value.clone()
        } else {
            let mut merged_value: Value = Value::Vec(vec![]);
            for att in attrs {
                if let Some(v) = att.value.get_value() {
                    merged_value.append(v.clone());
                }
            }
            merged_value
        }
    }

    /// add attributes to this element
    #[inline]
    pub fn add_attributes(&mut self, attrs: Vec<Attribute<ATT, EVENT, MSG>>) {
        self.attrs.extend(attrs)
    }

    /// remove the attributes with this key
    /// TODO: this doesn't take into consideration the style attribute
    pub fn remove_attribute(&mut self, key: &ATT) {
        self.attrs.retain(|att| att.name() != key)
    }

    /// remove the existing values of this attribute
    /// and add the new values
    pub fn set_attributes(&mut self, attrs: Vec<Attribute<ATT, EVENT, MSG>>) {
        attrs
            .iter()
            .for_each(|att| self.remove_attribute(att.name()));
        self.add_attributes(attrs);
    }

    /// add children virtual node to this element
    #[inline]
    pub fn add_children(&mut self, children: Vec<Node<T, ATT, EVENT, MSG>>) {
        self.children.extend(children);
    }

    /// attach a callback to this element
    #[inline]
    pub fn add_event_listener(&mut self, event: ATT, cb: Callback<EVENT, MSG>) {
        let attr_event = Attribute {
            name: Some(event),
            value: cb.into(),
            namespace: None,
        };
        self.attrs.push(attr_event);
    }

    /// map_callback the return of the callback from MSG to MSG2
    pub(super) fn map_callback<MSG2>(
        self,
        cb: Callback<MSG, MSG2>,
    ) -> Element<T, ATT, EVENT, MSG2>
    where
        MSG2: 'static,
    {
        Element {
            tag: self.tag,
            attrs: self
                .attrs
                .into_iter()
                .map(|attr| attr.map_callback(cb.clone()))
                .collect(),
            namespace: self.namespace,
            children: self
                .children
                .into_iter()
                .map(|child| child.map_callback(cb.clone()))
                .collect(),
        }
    }

    /// returns the text if this node has only one child and is a text.
    /// includes: h1, h2..h6, p,
    pub fn eldest_child_text(&self) -> Option<&str> {
        self.children.first().map(|e| e.text()).flatten()
    }

    /// check if the children of this node is only 1 and it is a text node
    pub fn is_children_a_node_text(&self) -> bool {
        self.children.len() == 1 && self.children[0].is_text_node()
    }

    /// make a pretty string representation of this node
    pub(super) fn render(
        &self,
        buffer: &mut dyn Write,
        indent: usize,
    ) -> fmt::Result
    where
        T: ToString,
        ATT: ToString,
    {
        write!(buffer, "<{}", self.tag.to_string())?;

        for attr in self.attributes().iter() {
            write!(buffer, " ")?;
            attr.render(buffer)?;
        }
        if let Some(style_attr) = self.aggregate_styles() {
            write!(buffer, " ")?;
            style_attr.render(buffer)?;
        }
        write!(buffer, ">")?;

        // do not indent if it is only text child node
        if self.is_children_a_node_text() {
            self.children[0].render(buffer, indent)?;
        } else {
            // otherwise print all child nodes with each line and indented
            for child in self.children.iter() {
                write!(buffer, "\n{}", util::indent(indent + 1))?;
                child.render(buffer, indent + 1)?;
            }
        }
        // do not make a new line it if is only a text child node or it has no child nodes
        if !(self.is_children_a_node_text() || self.children.is_empty()) {
            write!(buffer, "\n{}", util::indent(indent))?;
        }
        write!(buffer, "</{}>", self.tag.to_string())?;
        Ok(())
    }
}

impl<T, ATT, EVENT, MSG> From<Element<T, ATT, EVENT, MSG>>
    for Node<T, ATT, EVENT, MSG>
where
    ATT: Clone,
{
    fn from(v: Element<T, ATT, EVENT, MSG>) -> Self {
        Node::Element(v)
    }
}
