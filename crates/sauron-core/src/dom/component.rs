use crate::Node;
pub use effects::Effects;

mod effects;

/// A component has a view and can update itself.
///
/// The update function returns an effect which can contain
/// follow ups and effects. Follow ups are executed on the next
/// update loop of this component, while the effects are executed
/// on the parent component that mounts it.
pub trait Component<MSG, PMSG> {
    /// Update the model of this component and return
    /// follow up and/or effects that will be executed on the next update loop
    fn update(&mut self, msg: MSG) -> Effects<MSG, PMSG>;

    /// the view of the component
    fn view(&self) -> Node<MSG>;

    /// optionally a Component can specify its own css style
    fn style(&self) -> String {
        String::new()
    }
}

/// A Container have children that is set from the parent component
///
/// It can update its Mode and returns follow ups and/or effects on the next
/// update loop.
///
/// The view in the container is set by the parent component. The container itself
/// can not listen to events on its view
pub trait Container<MSG, PMSG> {
    /// update the model of this component and return follow ups and/or effects
    /// that will be executed on the next update loop.
    fn update(&mut self, msg: MSG) -> Effects<MSG, PMSG>;

    /// The container presents the children passed to it from the parent.
    /// The container can decide how to display the children components here, but
    /// the children nodes here can not trigger Msg that can update this component
    fn view(&self) -> Node<PMSG>;

    /// optionally a Container can specify its own css style
    fn style(&self) -> String {
        String::new()
    }
}

/// Just a view, no events, no update.
/// The properties of the component is set directly from the parent
pub trait View<MSG> {
    /// only returns a view of itself
    fn view(&self) -> Node<MSG>;

    /// optionally a View can specify its own css style
    fn style(&self) -> String {
        String::new()
    }
}
