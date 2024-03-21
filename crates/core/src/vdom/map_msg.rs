use crate::vdom::Attribute;
use crate::vdom::AttributeValue;
use crate::vdom::Element;
use crate::vdom::Node;
use crate::vdom::TemplatedView;
use crate::vdom::Leaf;
use std::rc::Rc;

impl<MSG> Node<MSG> {
    /// map the msg of this node such that Node<MSG> becomes Node<MSG2>
    pub fn map_msg<F, MSG2>(self, cb: F) -> Node<MSG2>
    where
        F: Fn(MSG) -> MSG2 + Clone + 'static,
        MSG2: 'static,
        MSG: 'static,
    {
        match self {
            Node::Element(element) => Node::Element(element.map_msg(cb)),
            Node::Leaf(leaf) => Node::Leaf(leaf.map_msg(cb)),
        }
    }
}

impl<MSG> Element<MSG> {
    /// map the msg of this element such that `Element<MSG>` becomes `Element<MSG2>`
    pub fn map_msg<F, MSG2>(self, cb: F) -> Element<MSG2>
    where
        F: Fn(MSG) -> MSG2 + Clone + 'static,
        MSG2: 'static,
        MSG: 'static,
    {
        Element {
            namespace: self.namespace,
            tag: self.tag,
            attrs: self
                .attrs
                .into_iter()
                .map(|attr| attr.map_msg(cb.clone()))
                .collect(),
            children: self
                .children
                .into_iter()
                .map(|child| child.map_msg(cb.clone()))
                .collect(),
            self_closing: self.self_closing,
        }
    }
}

impl<MSG> Attribute<MSG> {
    /// map the msg of this attribute such that `Attribute<MSG>` becomes `Attribute<MSG2>`
    pub fn map_msg<F, MSG2>(self, cb: F) -> Attribute<MSG2>
    where
        F: Fn(MSG) -> MSG2 + Clone + 'static,
        MSG2: 'static,
        MSG: 'static,
    {
        Attribute {
            name: self.name,
            value: self
                .value
                .into_iter()
                .map(|v| v.map_msg(cb.clone()))
                .collect(),
            namespace: self.namespace,
        }
    }
}

impl<MSG> AttributeValue<MSG> {
    /// map the msg of this AttributeValue such that `AttributeValue<MSG>` becomes
    /// `AttributeValue<MSG2>`
    pub fn map_msg<F, MSG2>(self, cb: F) -> AttributeValue<MSG2>
    where
        F: Fn(MSG) -> MSG2 + Clone + 'static,
        MSG2: 'static,
        MSG: 'static,
    {
        match self {
            AttributeValue::FunctionCall(this) => AttributeValue::FunctionCall(this),
            AttributeValue::Simple(this) => AttributeValue::Simple(this),
            AttributeValue::Style(this) => AttributeValue::Style(this),
            AttributeValue::EventListener(this) => AttributeValue::EventListener(this.map_msg(cb)),
            AttributeValue::Empty => AttributeValue::Empty,
        }
    }
}

impl<MSG> Leaf<MSG> {

    /// map the msg of this Leaf such that `Leaf<MSG>` becomes `Leaf<MSG2>`
    pub fn map_msg<F, MSG2>(self, cb: F) -> Leaf<MSG2>
    where
        F: Fn(MSG) -> MSG2 + Clone + 'static,
        MSG2: 'static,
        MSG: 'static,
    {
        match self {
            Self::Text(v) => Leaf::Text(v),
            Self::SafeHtml(v) => Leaf::SafeHtml(v),
            Self::Comment(v) => Leaf::Comment(v),
            Self::DocType(v) => Leaf::DocType(v),
            Self::Fragment(nodes) => Leaf::Fragment(
                nodes
                    .into_iter()
                    .map(|node| node.map_msg(cb.clone()))
                    .collect(),
            ),
            Self::NodeList(node_list) => Leaf::NodeList(
                node_list
                    .into_iter()
                    .map(|node| node.map_msg(cb.clone()))
                    .collect(),
            ),
            Self::StatefulComponent(v) => Leaf::StatefulComponent(v.map_msg(cb)),
            Self::StatelessComponent(v) => Leaf::StatelessComponent(v.map_msg(cb)),
            Self::TemplatedView(v) => Leaf::TemplatedView(v.map_msg(cb)),
        }
    }
}

impl<MSG> TemplatedView<MSG>{

    /// mape the msg of this TemplatedView such that `TemplatedView<MSG>` becomes `TemplatedView<MSG2>`
    pub fn map_msg<F, MSG2>(self, cb: F) -> TemplatedView<MSG2>
    where
        F: Fn(MSG) -> MSG2 + Clone + 'static,
        MSG2: 'static,
        MSG: 'static,
    {
        TemplatedView{
            view: Box::new(self.view.map_msg(cb.clone())),
            template: Rc::new(move||(self.template)().map_msg(cb.clone())),
            skip_diff: self.skip_diff,
        }
    }
}
