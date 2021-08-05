//! Callbacks contains function that can be called at a later time.
//! This is used in containing an event listener attached to an DOM element.
use std::{convert::Into, fmt, rc::Rc};

/// A generic sized representation of a function that can be
/// attached to a Node. The callback will essentially be owned by the element
///
/// Limitations:
/// The callback takes an Fn instead of FnMut,
/// therefore it can not mutate the environment variables
///
/// In effect callbacks attached to DOM events are limited
/// to only passing an MSG to the program and not complex statements.
///
/// Note: It would have been nice to have the inner value be
/// `Rc<FnMut(EVENT) -> MSG> +'a`
/// but there are a lot of issues that this becomes infeasible:
///  1 - wasm_bindgen::Closure requires that 'static references to the closure
///  2 - Accessing `Rc::get_mut` requires that there is no other `Rc` or `Weak` references
///     else where.
///         - We could be iterating on the elements for recursively setting the attributes
///             which is not allowed to have recursive mutable iteration
///         - Attributes of the same name are merged therefore cloning the attributes, hence the
///         callback is necessary.
///
pub struct Callback<EVENT, MSG>(Rc<dyn Fn(EVENT) -> MSG>);

impl<EVENT, F, MSG> From<F> for Callback<EVENT, MSG>
where
    F: Fn(EVENT) -> MSG + 'static,
{
    fn from(func: F) -> Self {
        Callback(Rc::new(func))
    }
}

/// Note:
/// using the #[derive(Debug)] needs EVENT and MSG to also be Debug
///
/// The reason this is manually implemented is, so that EVENT and MSG
/// doesn't need to be Debug as it is part of the Callback objects and are not shown.
impl<EVENT, MSG> fmt::Debug for Callback<EVENT, MSG> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "||{{..}}")
    }
}

impl<EVENT, MSG> Callback<EVENT, MSG>
where
    EVENT: 'static,
    MSG: 'static,
{
    /// This method calls the actual callback.
    pub fn emit<T: Into<EVENT>>(&self, value: T) -> MSG {
        (self.0)(value.into())
    }

    /// map this callback using another callback such that
    /// MSG becomes MSG2
    pub fn map_callback<MSG2>(
        self,
        cb: Callback<MSG, MSG2>,
    ) -> Callback<EVENT, MSG2>
    where
        MSG2: 'static,
    {
        let func_wrap = move |input| {
            let out = self.emit(input);
            cb.emit(out)
        };
        Callback::from(func_wrap)
    }
}

/// Note:
/// using the #[derive(Clone)] needs EVENT and MSG to also be Clone
///
/// The reason this is manually implemented is, so that EVENT and MSG
/// doesn't need to be Clone as it is part of the Callback objects and cloning here
/// is just cloning the pointer of the actual callback function
impl<EVENT, MSG> Clone for Callback<EVENT, MSG> {
    fn clone(&self) -> Self {
        Callback(Rc::clone(&self.0))
    }
}

/// Note:
/// using the #[derive(PartialEq)] needs EVENT and MSG to also be PartialEq.
///
/// The reason this is manually implemented is, so that EVENT and MSG
/// doesn't need to be PartialEq as it is part of the Callback objects and are not compared
///
/// Note: There is no 2 closures are equal, even if they are logically the same.
/// We return true here so as to not make a diff for the callback attribute.
/// Replacing the callback function for each view call is in-efficient.
impl<EVENT, MSG> PartialEq for Callback<EVENT, MSG> {
    fn eq(&self, _rhs: &Self) -> bool {
        true
        // Comparing the callback is only applicable
        // when they are a clone to each other.
        // This defeats the purpose in logically comparing for the
        // diffing algorthmn since all callbacks are effectively called with the closure.into()
        // which are essentially not the same Callback even when they are derived from the same
        // function.
        // Also, no 2 closures are the same.
        //Rc::ptr_eq(&self.0, &rhs.0)
    }
}
