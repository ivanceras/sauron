/// This trait is used in the DomUpdater to call the dispatch
/// method when an event occured
///
/// The Program will implement Dispatch instead of sending it to the
/// DomUpdater, this will simplify the amount of generics being defined.
pub trait Dispatch<MSG> {
    /// Executes the implementing dispatch function.
    /// In this case the implementation is the Program
    /// which is responsible for executing the update functions
    /// using the msg supplied.
    /// A new view will then be created and it will be diff to the previous view
    /// which will produce patches.
    /// These patched will then be applied to the browser DOM.
    fn dispatch(&self, msg: MSG);

    /// dispatch multiple msg
    fn dispatch_multiple(&self, msgs: impl IntoIterator<Item = MSG>);
}
