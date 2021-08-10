//! provides functionalities for commands to be executed by the system, such as
//! when the application starts or after the application updates.
//!
use std::rc::Rc;

/// Cmd is a command to be executed by the system.
/// This is returned at the init function of a component and is executed right
/// after instantiation of that component.
/// Cmd required a DSP object which is the Program as an argument
/// The emit function is called with the program argument.
/// The callback is supplied with the program an is then executed/emitted.
pub struct Cmd<DSP> {
    /// the functions that would be executed when this Cmd is emited
    pub commands: Vec<Rc<dyn Fn(DSP)>>,
    /// this instruct the program whether or not to update the view
    pub should_update_view: bool,
    /// tell the cmd to log the measurements of not.
    /// set only to true for a certain MSG where you want to measure the performance
    /// in the component update function. Otherwise, measurement calls
    /// for other non-trivial functions are also called causing on measurements.
    pub log_measurements: bool,
}

impl<DSP> Cmd<DSP>
where
    DSP: Clone + 'static,
{
    /// creates a new Cmd from a function
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(DSP) + 'static,
    {
        Self {
            commands: vec![Rc::new(f)],
            should_update_view: true,
            log_measurements: false,
        }
    }

    /// creates a unified Cmd which batches all the other Cmds in one.
    pub fn batch(cmds: Vec<Self>) -> Self {
        let mut commands = vec![];
        for cmd in cmds {
            commands.extend(cmd.commands);
        }
        Self {
            commands,
            should_update_view: true,
            log_measurements: false,
        }
    }

    /// A Cmd with no callback, similar to NoOp.
    pub fn none() -> Self {
        Cmd {
            commands: vec![],
            should_update_view: true,
            log_measurements: false,
        }
    }

    /// Executes the Cmd
    pub fn emit(self, program: &DSP) {
        for cb in self.commands {
            let program_clone = program.clone();
            cb(program_clone);
        }
    }

    /// Creates an empty Cmd and specifies that there is NO update will be made to the
    /// view.
    pub fn should_update_view(should_update_view: bool) -> Self {
        Self {
            commands: vec![],
            should_update_view,
            log_measurements: false,
        }
    }

    /// Create a cmd which instruct the program that there is NO update
    /// will be made to the view
    pub fn no_render() -> Self {
        Self::should_update_view(false)
    }

    /// Create a cmd which instruct the program that it should log the measurements
    /// for each part of the dispatch process.
    pub fn measure() -> Self {
        Self {
            commands: vec![],
            should_update_view: true,
            log_measurements: true,
        }
    }
}
