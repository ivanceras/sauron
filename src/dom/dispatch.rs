/// This trait is used in the DomUpdater to call the dispatch
/// method when an event occured
///
/// The Program will implement Dispatch instead of sending it to the
/// DomUpdater, this will simplify the amount of generics being defined.
pub trait Dispatch<MSG> {
    fn dispatch(&self, msg: MSG);
}
