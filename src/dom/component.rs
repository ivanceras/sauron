use crate::{
    Cmd,
    Node,
};

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

    /// Called each time an action is triggered from the view
    fn update(&mut self, msg: MSG) -> Cmd<Self, MSG>
    where
        Self: Sized + 'static;

    /// Returns a node on how the component is presented.
    fn view(&self) -> Node<MSG>;
}
