use crate::{Callback,
            Program};
use std::{fmt::Debug,
          rc::Rc};

pub struct Cmd<APP, MSG>(pub Vec<Callback<Rc<Program<APP, MSG>>, ()>>);

impl<APP, MSG> Cmd<APP, MSG>
    where APP: 'static,
          MSG: Debug + 'static
{
    pub fn new<F>(cmd: F) -> Self
        where F: Fn(Rc<Program<APP, MSG>>) + 'static
    {
        let cb: Callback<Rc<Program<APP, MSG>>, ()> = cmd.into();
        Cmd(vec![cb])
    }

    pub fn batch(cmds: Vec<Self>) -> Self {
        let mut callbacks = vec![];
        for cmd in cmds {
            callbacks.extend(cmd.0);
        }
        Cmd(callbacks)
    }

    pub fn none() -> Self {
        Cmd(vec![])
    }

    pub fn emit(self, program: &Rc<Program<APP, MSG>>) {
        for cb in self.0 {
            let program_clone = Rc::clone(&program);
            cb.emit(program_clone);
        }
    }
}
