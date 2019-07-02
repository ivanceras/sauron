use crate::{
    util,
    Attribute,
    Callback,
    Node,
    Value,
};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Element<T, EVENT, MSG>
where
    MSG: 'static,
    EVENT: 'static,
{
    pub tag: T,
    pub attrs: Vec<Attribute<EVENT, MSG>>,
    pub children: Vec<Node<T, EVENT, MSG>>,
    pub namespace: Option<&'static str>,
}

impl<T, EVENT, MSG> Element<T, EVENT, MSG>
where
    EVENT: 'static,
    MSG: 'static,
{
    pub fn with_tag(tag: T) -> Self {
        Element {
            tag,
            attrs: vec![],
            children: vec![],
            namespace: None,
        }
    }

    /// get the attributes that are events
    pub fn events(&self) -> Vec<&Attribute<EVENT, MSG>> {
        self.attrs.iter().filter(|attr| attr.is_event()).collect()
    }

    pub fn get_event(&self, name: &str) -> Option<&Attribute<EVENT, MSG>> {
        self.events()
            .iter()
            .find(|event| event.name == name)
            .map(|event| *event)
    }

    fn attributes_internal(&self) -> Vec<&Attribute<EVENT, MSG>> {
        self.attrs.iter().filter(|attr| !attr.is_event()).collect()
    }

    pub fn attributes(&self) -> Vec<Attribute<EVENT, MSG>> {
        let names = self.get_attributes_name();
        let mut attributes = vec![];
        for name in names {
            if let Some(value) = self.get_attr_value(name) {
                attributes.push(Attribute::with_name_value(name, value));
            }
        }
        attributes
    }

    /// return all the attributes that match the name
    fn get_attributes_with_name(
        &self,
        key: &str,
    ) -> Vec<&Attribute<EVENT, MSG>> {
        self.attributes_internal()
            .iter()
            .filter(|att| att.name == key)
            .map(|att| *att)
            .collect()
    }

    fn get_attributes_name(&self) -> Vec<&'static str> {
        let mut names = self
            .attributes_internal()
            .iter()
            .map(|att| att.name)
            .collect::<Vec<&str>>();
        names.sort();
        names.dedup();
        names
    }

    /// get all the attributes with the same name and merge their value
    pub fn get_attr_value(&self, key: &str) -> Option<Value> {
        let attrs = self.get_attributes_with_name(key);
        if !attrs.is_empty() {
            Some(Self::merge_attributes_values(attrs))
        } else {
            None
        }
    }

    /// merge this all attributes,
    /// this assumes that that this attributes has the same name
    fn merge_attributes_values(attrs: Vec<&Attribute<EVENT, MSG>>) -> Value {
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

    #[inline]
    pub fn add_attributes(&mut self, attrs: Vec<Attribute<EVENT, MSG>>) {
        self.attrs.extend(attrs)
    }

    #[inline]
    pub fn add_children(&mut self, children: Vec<Node<T, EVENT, MSG>>) {
        self.children.extend(children);
    }

    #[inline]
    pub fn add_event_listener(
        &mut self,
        event: &'static str,
        cb: Callback<EVENT, MSG>,
    ) {
        let attr_event = Attribute::new(event, cb.into());
        self.attrs.push(attr_event);
    }

    /// map_callback the return of the callback from MSG to MSG2
    pub(super) fn map_callback<MSG2>(
        self,
        cb: Callback<MSG, MSG2>,
    ) -> Element<T, EVENT, MSG2>
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

    /// check if the children of this node is only 1 and it is a text node
    fn is_children_a_node_text(&self) -> bool {
        self.children.len() == 1 && self.children[0].is_text_node()
    }

    /// make a pretty string representation of this node
    pub(super) fn to_pretty_string(&self, indent: i32) -> String
    where
        T: ToString,
    {
        let mut buffer = String::new();
        buffer += &format!("<{}", self.tag.to_string());

        for attr in self.attributes().iter() {
            buffer += &format!(r#" {}="{}""#, attr.name, attr.value);
        }
        buffer += ">";

        // do not indent if it is only text child node
        if self.is_children_a_node_text() {
            buffer += &self.children[0].to_pretty_string(indent);
        } else {
            // otherwise print all child nodes with each line and indented
            for child in self.children.iter() {
                buffer += &format!(
                    "\n{}{}",
                    util::indent(indent + 1),
                    child.to_pretty_string(indent + 1)
                );
            }
        }
        // do not make a new line it if is only a text child node or it has no child nodes
        if !(self.is_children_a_node_text() || self.children.is_empty()) {
            buffer += &format!("\n{}", util::indent(indent));
        }
        buffer += &format!("</{}>", self.tag.to_string());
        buffer
    }
}

impl<T, EVENT, MSG> From<Element<T, EVENT, MSG>> for Node<T, EVENT, MSG> {
    fn from(v: Element<T, EVENT, MSG>) -> Self {
        Node::Element(v)
    }
}
