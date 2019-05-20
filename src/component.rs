use crate::{
    Cmd,
    Node,
};
use std::fmt::Debug;

/// The app should implement this trait for it to be handled by the Program
pub trait Component<MSG>
where
    MSG: Debug + 'static,
{
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
