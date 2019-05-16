use crate::{Cmd,
            Dispatch,
            Node};
use std::{fmt::Debug,
          rc::Rc};

/// The app should implement this trait for it to be handled by the Program
pub trait Component<APP, MSG>
    where MSG: Debug + 'static,
          APP: 'static
{
    fn init(&self) -> Cmd<APP, MSG> {
        Cmd::none()
    }
    /// Called each time an action is triggered from the view
    fn update(&mut self, msg: MSG);
    /// Returns a node on how the component is presented.
    fn view(&self) -> Node<MSG>;
}
