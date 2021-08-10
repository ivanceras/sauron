use crate::{Cmd, Node};

/// Contains the time it took for the last app update call for the component
#[cfg(feature = "with-measure")]
#[derive(Clone, Copy, std::fmt::Debug, PartialEq)]
pub struct Measurements {
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
pub trait Component<MSG>
where
    MSG: 'static,
{
    /// an implementing APP component can have an init function
    /// which executes right after the APP is instantiated by the program
    fn init(&self) -> Cmd<Self, MSG>
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
    #[cfg(feature = "with-measure")]
    fn measurements(&mut self, measurements: Measurements) -> Cmd<Self, MSG>
    where
        Self: Sized + 'static,
    {
        log::debug!("Measurements: {:#?}", measurements);
        Cmd::none()
    }
}
