use std::{fmt, rc::Rc};

/// A generic wrapper for a closure in rust where we can transform and pass around.
pub struct Callback<IN, OUT> {
    func: Rc<dyn Fn(IN) -> OUT>,
}

impl<IN, OUT> Callback<IN, OUT> {
    /// This method calls the actual callback.
    pub fn emit(&self, input: IN) -> OUT {
        (self.func)(input)
    }
}

impl<IN, OUT> Callback<IN, OUT>
where
    IN: 'static,
    OUT: 'static,
{
    /// map the out msg of this callback such that `Callback<IN,OUT>` becomes `Callback<IN,OUT2>`
    pub fn map_msg<F, OUT2>(self, cb: F) -> Callback<IN, OUT2>
    where
        F: Fn(OUT) -> OUT2 + Clone + 'static,
        OUT2: 'static,
    {
        let cb_wrap = move |input| {
            let out = self.emit(input);
            cb(out)
        };
        Callback::from(cb_wrap)
    }
}

impl<IN, F, OUT> From<F> for Callback<IN, OUT>
where
    F: Fn(IN) -> OUT + 'static,
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
