use std::convert::Into;
use std::fmt;
use std::rc::Rc;

#[derive(Clone)]
pub struct Callback<IN>(Rc<dyn Fn(IN)>);

impl<IN, F: Fn(IN) + 'static> From<F> for Callback<IN> {
    fn from(func: F) -> Self {
        Callback(Rc::new(func))
    }
}
impl<IN> fmt::Debug for Callback<IN> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "||{{..}}")
    }
}

impl<IN> Callback<IN> {
    /// This method calls the actual callback.
    pub fn emit<T: Into<IN>>(&self, value: T) {
        (self.0)(value.into());
    }
}

impl<IN> PartialEq for Callback<IN> {
    fn eq(&self, rhs: &Self) -> bool {
        Rc::ptr_eq(&self.0, &rhs.0)
    }
}
