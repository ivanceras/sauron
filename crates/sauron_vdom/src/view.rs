use super::Node;

/// A view should be able to return a vdom::Node
/// A dumb view, recreated every time the parent widget calls on update
pub trait View<T> {
    fn view(&self) -> Node<T>;
}

/// A smart widget that its components can be changed
/// before a view is created
pub trait Widget<T>: View<T> {
    fn update(&mut self);
}

/// This is the main app, the app
/// routes whenever there is changes in the store
/// this callback will be called
pub trait Component<T>: Widget<T> + View<T> {
    fn subscribe(&mut self, f: Box<Fn()>);
}
