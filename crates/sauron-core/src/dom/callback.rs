use std::{convert::Into, fmt, rc::Rc};

/// A generic wrapper for a closure in rust where we can transform and pass around.
pub struct Callback<IN, OUT> {
    func: Rc<dyn Fn(IN) -> OUT>,
}

impl<IN, F, OUT> From<F> for Callback<IN, OUT>
where
    F: Fn(IN) -> OUT + 'static,
    OUT: 'static,
    IN: 'static,
{
    fn from(func: F) -> Self {
        Self {
            func: Rc::new(func),
        }
    }
}

impl<IN, OUT> fmt::Debug for Callback<IN, OUT> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "||{{..}}")
    }
}

impl<IN, OUT> Callback<IN, OUT>
where
    IN: 'static,
    OUT: 'static,
{
    /// This method calls the actual callback.
    pub fn emit<T: Into<IN>>(&self, value: T) -> OUT {
        (self.func)(value.into())
    }
}

impl<IN, OUT> Clone for Callback<IN, OUT> {
    fn clone(&self) -> Self {
        Self {
            func: Rc::clone(&self.func),
        }
    }
}

impl<IN, OUT> PartialEq for Callback<IN, OUT> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.func, &other.func)
    }
}
