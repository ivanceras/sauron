use crate::Node;
pub use effects::Effects;

mod effects;

/// A component has a view and can update itself.
/// Optionally a component can return a succeeding Msg to be done on the next
/// update iteration.
pub trait Component<MSG, PMSG> {
    /// update itself and can return an optional Msg to be called
    /// on the next update loop.
    fn update(&mut self, msg: MSG) -> Effects<MSG, PMSG>;

    /// the view of the component
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
