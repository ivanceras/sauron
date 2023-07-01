use crate::{dom::Effects, vdom::Node};

/// A component has a view and can update itself.
///
/// The update function returns an effect which can contain
/// follow ups and effects. Follow ups are executed on the next
/// update loop of this component, while the effects are executed
/// on the parent component that mounts it.
pub trait Component<MSG, XMSG> {
    /// Update the model of this component and return
    /// follow up and/or effects that will be executed on the next update loop
    fn update(&mut self, msg: MSG) -> Effects<MSG, XMSG>;

    /// the view of the component
    fn view(&self) -> Node<MSG>;

    /// optionally a Component can specify its own css style
    fn style(&self) -> String {
        String::new()
    }

    /// Component can have component id to identify themselves
    fn get_component_id(&self) -> Option<&String> {
        None
    }
}

/// A Container have children that is set from the parent component
///
/// It can update its Mode and returns follow ups and/or effects on the next
/// update loop.
///
/// The view in the container is set by the parent component. The container itself
/// can not listen to events on its view
pub trait Container<MSG, XMSG> {
    /// update the model of this component and return follow ups and/or effects
    /// that will be executed on the next update loop.
    fn update(&mut self, msg: MSG) -> Effects<MSG, XMSG>;

    /// The container presents the children passed to it from the parent.
    /// The container can decide how to display the children components here, but
    /// the children nodes here can not trigger Msg that can update this component
    fn view(&self, content: impl IntoIterator<Item = Node<XMSG>>) -> Node<MSG>;

    /// optionally a Container can specify its own css style
    fn style(&self) -> String {
        String::new()
    }

    /// containers can append children
    fn append_child(&mut self, child: Node<XMSG>);
}

/// A view is a very simple component that can update itself view,
/// but it can have no effect on the external component that use it.
pub trait View<MSG> {
    fn update(&mut self, msg: MSG) -> Effects<MSG, ()>;
    fn view(&self) -> Node<MSG>;
    fn style(&self) -> String {
        String::new()
    }
}
