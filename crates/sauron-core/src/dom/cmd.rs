//! provides functionalities for commands to be executed by the system, such as
//! when the application starts or after the application updates.
//!
use std::marker::PhantomData;
use std::rc::Rc;

/// Cmd is a command to be executed by the system.
/// This is returned at the init function of a component and is executed right
/// after instantiation of that component.
/// Cmd required a DSP object which is the Program as an argument
/// The emit function is called with the program argument.
/// The callback is supplied with the program an is then executed/emitted.
pub struct Cmd<DSP, MSG>(pub Vec<Rc<dyn Fn(DSP)>>, PhantomData<MSG>);

impl<DSP, MSG> Cmd<DSP, MSG>
where
    MSG: 'static,
    DSP: Clone + 'static,
{
    /// creates a new Cmd from a function
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(DSP) + 'static,
    {
        Cmd(vec![Rc::new(f)], PhantomData)
    }

    /// creates a unified Cmd which batches all the other Cmds in one.
    pub fn batch(cmds: Vec<Self>) -> Self {
        let mut callbacks = vec![];
        for cmd in cmds {
            callbacks.extend(cmd.0);
        }
        Cmd(callbacks, PhantomData)
    }

    /// A Cmd with no callback, similar to NoOp.
    pub fn none() -> Self {
        Cmd(vec![], PhantomData)
    }

    /// Executes the Cmd
    pub fn emit(self, program: &DSP) {
        for cb in self.0 {
            let program_clone = program.clone();
            cb(program_clone);
        }
    }
}
