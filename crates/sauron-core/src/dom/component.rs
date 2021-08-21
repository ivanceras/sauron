use crate::Node;

/// A component has a view and can update itself.
/// Optionally a component can return a succeeding Msg to be done on the next
/// update iteration.
pub trait Component<MSG> {
    /// update itself and can return an optional Msg to be called
    /// on the next update loop.
    fn update(&mut self, msg: MSG) -> Vec<MSG>;

    /// the view of the component
    fn view(&self) -> Node<MSG>;
}

/// Msg that needs to be executed in its component on the next update loop
pub struct Effects<MSG, PMSG> {
    /// Msg that will be executed in its own widget
    pub follow_ups: Vec<MSG>,
    /// PMSG that will be executed in the calling component
    pub effects: Vec<PMSG>,
}

impl<MSG, PMSG> Effects<MSG, PMSG> {
    /// create a new effects with follow_ups and effects
    pub fn new(follow_ups: Vec<MSG>, effects: Vec<PMSG>) -> Self {
        Self {
            follow_ups,
            effects,
        }
    }
    /// create a follow up message, but no effects
    pub fn with_follow_ups(follow_ups: Vec<MSG>) -> Self {
        Self {
            follow_ups,
            effects: vec![],
        }
    }
    /// Create effects with no follow ups.
    pub fn with_effects(effects: Vec<PMSG>) -> Self {
        Self {
            follow_ups: vec![],
            effects,
        }
    }

    /// No effects
    pub fn none() -> Self {
        Self {
            follow_ups: vec![],
            effects: vec![],
        }
    }
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
