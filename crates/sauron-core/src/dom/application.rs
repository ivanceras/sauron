use crate::{dom::Cmd, vdom::Node};

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

    /// Update the component with a message.
    /// The update function returns a Cmd, which can be executed by the runtime.
    ///
    /// Called each time an action is triggered from the view
    fn update(&mut self, _msg: MSG) -> Cmd<Self, MSG>
    where
        Self: Sized + 'static;

    /// Returns a node on how the component is presented.
    fn view(&self) -> Node<MSG>;

    /// optionally an Application can specify its own css style
    fn style(&self) -> String;

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
/// TODO: Maybe rename to Diagnostics
#[derive(Clone, Debug, PartialEq)]
pub struct Measurements {
    /// The application can name this measurement to determine where this measurement is coming
    /// from.
    pub name: Option<String>,
    /// The number of DOM nodes in this Component
    pub node_count: usize,
    /// Time it took for the Component to build it's view
    pub build_view_took: f64,
    /// Total number of patches applied on this update loop
    pub total_patches: usize,
    /// Time it took for the patching the DOM.
    pub dom_update_took: f64,
    /// Total time it took for the component dispatch
    pub total_time: f64,
}

/// Auto implementation of Application trait for Component that
/// has no external MSG
impl<COMP, MSG> Application<MSG> for COMP
where
    COMP: crate::Component<MSG, ()> + 'static,
    COMP: crate::CustomElement<MSG>,
    MSG: 'static,
{
    fn update(&mut self, msg: MSG) -> Cmd<Self, MSG> {
        let effects = <Self as crate::Component<MSG, ()>>::update(self, msg);
        Cmd::from(effects)
    }

    fn view(&self) -> Node<MSG> {
        <Self as crate::Component<MSG, ()>>::view(self)
    }

    fn style(&self) -> String {
        <Self as crate::Component<MSG, ()>>::style(self)
    }
}

/// Auto implementation of Component trait for Container,
/// which in turn creates an Auto implementation trait for of Application for Container
impl<CONT, MSG> crate::Component<MSG, ()> for CONT
where
    CONT: crate::Container<MSG, ()>,
    CONT: crate::CustomElement<MSG>,
    MSG: 'static,
{
    fn update(&mut self, msg: MSG) -> crate::Effects<MSG, ()> {
        <Self as crate::Container<MSG, ()>>::update(self, msg)
    }

    fn view(&self) -> Node<MSG> {
        // converting the component to container loses ability
        // for the container to contain children components
        <Self as crate::Container<MSG, ()>>::view(self, [])
    }

    fn style(&self) -> String {
        <Self as crate::Container<MSG, ()>>::style(self)
    }
}
