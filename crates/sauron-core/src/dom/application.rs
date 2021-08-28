use crate::{Cmd, Node};

/// An Application is the root component of your program.
/// Everything that happens in your application is done here.
///
pub trait Application<MSG>
where
    MSG: 'static,
{
    ///  The application can implement this method where it can modify its initial state.
    ///  This method is called right after the program is mounted into the DOM.
    fn init(&mut self) -> Cmd<Self, MSG>
    where
        Self: Sized + 'static,
    {
        Cmd::none()
    }

    /// Called each time an action is triggered from the view
    fn update(&mut self, _msg: MSG) -> Cmd<Self, MSG>
    where
        Self: Sized + 'static;

    /// Returns a node on how the component is presented.
    fn view(&self) -> Node<MSG>;

    /// optionally an Application can specify its own css style
    fn style(&self) -> String {
        String::new()
    }

    /// This is called after dispatching and updating the dom for the component
    /// This is for diagnostic and performance measurement purposes.
    ///
    /// Warning: DO NOT use for anything else other than the intended purpose
    fn measurements(&self, measurements: Measurements) -> Cmd<Self, MSG>
    where
        Self: Sized + 'static,
    {
        log::debug!("Measurements: {:#?}", measurements);
        Cmd::none().no_render()
    }
}

/// Contains the time it took for the last app update call for the component
#[derive(Clone, Debug, PartialEq)]
pub struct Measurements {
    /// The application can name this measurement to determine where this measurement is coming
    /// from.
    pub name: String,
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
