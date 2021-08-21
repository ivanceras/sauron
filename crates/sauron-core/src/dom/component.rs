use crate::Node;
pub use effects::Effects;

mod effects;

/// A component has a view and can update itself.
/// Optionally a component can return a succeeding Msg to be done on the next
/// update iteration.
pub trait Component<MSG> {
    /// update itself and can return an optional Msg to be called
    /// on the next update loop.
    fn update(&mut self, msg: MSG) -> Effects<MSG, ()>;

    /// the view of the component
    fn view(&self) -> Node<MSG>;
}

/// A widget has the same capability to a Component.
/// Parent component of a widget can listen to widget events.
///
/// It has a view of it's own and can update itself.
/// It can return an Optional Msg to update itself on the next update loop.
/// Additionally, it can trigger listeners that is hook from the parent component that uses it.
///
/// Widgets can NOT have children set from the parent components.
pub trait Widget<MSG, PMSG> {
    /// update this widget with the msg.
    /// can optionally return a Msg for the next update.
    ///
    /// The Vec<PMSG> is the the msg can optionally be return to the calling component
    /// as a result from triggering the event listeners
    fn update(&mut self, msg: MSG) -> Effects<MSG, PMSG>;

    /// view of this widget.
    fn view(&self) -> Node<MSG>;
}

/// A container where the view is set in the parent Component
/// It can contain children set from the parent Component.
/// But it can NOT listen to events in its children
pub trait Container<MSG, PMSG> {
    /// it can update itself
    fn update(&mut self, msg: MSG) -> Effects<MSG, PMSG>;

    /// but the view is set from the parent component
    fn view(&self) -> Node<PMSG>;
}

/// Just a view, no events, no update.
/// The properties of the component is set directly from the parent
pub trait View<MSG> {
    /// only returns a view of itself
    fn view(&self) -> Node<MSG>;
}
