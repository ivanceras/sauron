//! Callbacks contains function that can be called at a later time.
//! This is used in containing an event listener attached to an DOM element.
use std::any::TypeId;
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
pub struct Callback<EVENT, MSG> {
    /// the function to be executed
    func: Rc<dyn Fn(EVENT) -> MSG>,
    /// the type type_id of the event this callback will be attached to
    event_type_id: TypeId,
    /// the type_id of the return type of this callback when executed.
    msg_type_id: TypeId,
}

impl<EVENT, F, MSG> From<F> for Callback<EVENT, MSG>
where
    F: Fn(EVENT) -> MSG + 'static,
    MSG: 'static,
    EVENT: 'static,
{
    fn from(func: F) -> Self {
        println!("type_id of F: {:?}", std::any::TypeId::of::<F>());
        println!("type_id of MSG: {:?}", std::any::TypeId::of::<MSG>());
        println!("type_id of EVENT: {:?}", std::any::TypeId::of::<EVENT>());
        Self {
            func: Rc::new(func),
            event_type_id: TypeId::of::<EVENT>(),
            msg_type_id: TypeId::of::<MSG>(),
        }
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
        (self.func)(value.into())
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
        Self {
            func: Rc::clone(&self.func),
            event_type_id: self.event_type_id,
            msg_type_id: self.msg_type_id,
        }
    }
}

/// This is the best approximation of comparison whether 2 callbacks are equal.
///
/// There is no 100% guarante that this is true since we can not compare 2 closures event if they
/// have the same logic.
///
/// This is done by comparing the type_id of the input and type_id of the output.
///
impl<EVENT, MSG> PartialEq for Callback<EVENT, MSG> {
    fn eq(&self, other: &Self) -> bool {
        self.event_type_id == other.event_type_id
            && self.msg_type_id == other.msg_type_id
    }
}
