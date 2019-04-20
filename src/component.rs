/// The app should implement this trait for it to be handled by the Program
///
pub trait Component<MSG> {
    /// Called each time an action is triggered from the view
    fn update(&mut self, msg: MSG);
    /// Returns a node on how the component is presented.
    fn view(&self) -> Node<MSG>;
    /// any subscription related
    fn subscribe(&self);
}
