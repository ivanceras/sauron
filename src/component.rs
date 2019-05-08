use crate::{Dispatch,
            Node};
use std::rc::Rc;

/// The app should implement this trait for it to be handled by the Program
pub trait Component<MSG> {
    /// The init function of your app
    /// such as fetching data from a rest api to populate the app
    fn init<DSP>(&self, _program: &Rc<DSP>)
        where DSP: Dispatch<MSG> + 'static
    {
    }
    /// Called each time an action is triggered from the view
    fn update(&mut self, msg: MSG);
    /// Returns a node on how the component is presented.
    fn view(&self) -> Node<MSG>;
}
