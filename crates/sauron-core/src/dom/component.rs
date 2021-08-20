use crate::{Cmd, Node, Program};

/// Contains the time it took for the last app update call for the component
#[derive(Clone, Copy, std::fmt::Debug, PartialEq)]
pub struct Measurements {
    /// The number of DOM nodes in this Component
    pub view_node_count: usize,
    /// Time it took for dispatching the Component's update function
    pub update_dispatch_took: f64,
    /// Time it took for the Component to build it's view
    pub build_view_took: f64,
    /// Time it took for the patching the DOM.
    pub dom_update_took: f64,
    /// Total time it took for the component dispatch
    pub total_time: f64,
}

/// The app should implement this trait for it to be handled by the Program
pub trait Application<MSG>
where
    MSG: 'static,
{
    ///  The application can implement this method where it can modify its initial state.
    ///  It also has access to the program which is the executor of the lifecycle of the program.
    ///
    ///  this method is called right after the program is mounted into the DOM.
    fn init(&mut self, _program: Program<Self, MSG>) -> Cmd<Self, MSG>
    where
        Self: Sized + 'static,
    {
        Cmd::none()
    }

    /// optionally a component can specify it's own css style
    fn style(&self) -> Vec<String> {
        vec![]
    }

    /// Called each time an action is triggered from the view
    fn update(&mut self, _msg: MSG) -> Cmd<Self, MSG>
    where
        Self: Sized + 'static;

    /// Returns a node on how the component is presented.
    fn view(&self) -> Node<MSG>;

    /// This is called after dispatching and updating the dom for the component
    fn measurements(&self, measurements: Measurements) -> Cmd<Self, MSG>
    where
        Self: Sized + 'static,
    {
        log::debug!("Measurements: {:#?}", measurements);
        Cmd::no_render()
    }
}

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
    fn update(&mut self, msg: MSG) -> (Vec<MSG>, Vec<PMSG>);

    /// view of this widget.
    fn view(&self) -> Node<MSG>;
}

/// A container where the view is set in the parent Component
/// It can contain children set from the parent Component.
/// But it can NOT listen to events in its children
pub trait Container<MSG, PMSG> {
    /// it can update itself
    fn update(&mut self, msg: MSG) -> (Vec<MSG>, Vec<PMSG>);

    /// but the view is set from the parent component
    fn view(&self) -> Node<PMSG>;
}

/// Just a view, no events, no update.
/// The properties of the component is set directly from the parent
pub trait View<MSG> {
    /// only returns a view of itself
    fn view(&self) -> Node<MSG>;
}
