use crate::{
    dispatch::Dispatch,
    Callback,
};
use std::{
    marker::PhantomData,
    rc::Rc,
};

/// Cmd is a command to be executed by the system.
/// This is returned at the init function of a component and is executed right
/// after instantiation of that component.
/// Cmd required a DSP object which is the Program as an argument
/// The emit function is called with the program argument.
/// The callback is supplied with the program an is then executed/emitted.
pub struct Cmd<DSP, MSG>(pub Vec<Callback<Rc<DSP>, ()>>, PhantomData<MSG>)
where
    DSP: Dispatch<MSG> + 'static;

impl<DSP, MSG> Cmd<DSP, MSG>
where
    MSG: 'static,
    DSP: Dispatch<MSG> + 'static,
{
    pub fn new<F>(cmd: F) -> Self
    where
        F: Fn(Rc<DSP>) -> () + 'static,
    {
        let cb: Callback<Rc<DSP>, ()> = cmd.into();
        Cmd(vec![cb], PhantomData)
    }

    pub fn batch(cmds: Vec<Self>) -> Self {
        let mut callbacks = vec![];
        for cmd in cmds {
            callbacks.extend(cmd.0);
        }
        Cmd(callbacks, PhantomData)
    }

    pub fn none() -> Self {
        Cmd(vec![], PhantomData)
    }

    pub fn emit(self, program: &Rc<DSP>) {
        for cb in self.0 {
            let program_clone = Rc::clone(&program);
            cb.emit(program_clone);
        }
    }
}
