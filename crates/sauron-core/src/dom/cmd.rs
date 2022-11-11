//! provides functionalities for commands to be executed by the system, such as
//! when the application starts or after the application updates.
//!
use crate::{Dispatch, Effects};

/// Cmd is a command to be executed by the system.
/// This is returned at the init function of a component and is executed right
/// after instantiation of that component.
/// Cmd required a DSP object which is the Program as an argument
/// The emit function is called with the program argument.
/// The callback is supplied with the program an is then executed/emitted.
pub struct Cmd<DSP> {
    /// the functions that would be executed when this Cmd is emited
    pub commands: Vec<Box<dyn FnOnce(DSP)>>,
    pub(crate) modifier: Modifier,
}

/// These are collections of fields where we can modify the Cmd
/// such as logging measurement or should update the view
#[derive(Clone)]
pub struct Modifier {
    /// this instruct the program whether or not to update the view
    pub should_update_view: bool,
    /// tell the cmd to log the measurements of not.
    /// set only to true for a certain MSG where you want to measure the performance
    /// in the component update function. Otherwise, measurement calls
    /// for other non-trivial functions are also called causing on measurements.
    pub log_measurements: bool,
    /// Set the measurement name for this Cmd.
    /// This is used to distinguish measurements from other measurements in different parts of you
    /// application
    ///
    /// This measurment name will be copied
    /// into the [`Measurements`](crate::dom::Measurements) passed in
    /// [`Application::measurements`](crate::Application::measurements)
    pub measurement_name: String,
}

impl Default for Modifier {
    fn default() -> Self {
        Self {
            // every cmd will update the view by default
            should_update_view: true,
            // every cmd will not log measurement by default
            log_measurements: false,
            // empty string by default
            measurement_name: String::new(),
        }
    }
}

impl<DSP> Cmd<DSP>
where
    DSP: 'static,
{
    /// creates a new Cmd from a function
    pub fn new<F>(f: F) -> Self
    where
        F: FnOnce(DSP) + 'static,
    {
        Self {
            commands: vec![Box::new(f)],
            modifier: Default::default(),
        }
    }

    /// When you need the runtime to perform couple of commands, you can batch
    /// then together.
    pub fn batch(cmds: impl IntoIterator<Item = Self>) -> Self {
        let mut commands = vec![];
        let mut should_update_view = false;
        let mut log_measurements = false;
        for cmd in cmds {
            if cmd.modifier.should_update_view {
                should_update_view = true;
            }
            if cmd.modifier.log_measurements {
                log_measurements = true;
            }
            commands.extend(cmd.commands);
        }
        Self {
            commands,
            modifier: Modifier {
                should_update_view,
                log_measurements,
                ..Default::default()
            },
        }
    }

    /// Add a cmd
    pub fn push(&mut self, cmd: Self) {
        self.append([cmd])
    }

    /// Append more cmd into this cmd and return self
    pub fn append(&mut self, cmds: impl IntoIterator<Item = Self>) {
        for cmd in cmds {
            if cmd.modifier.should_update_view {
                self.modifier.should_update_view = true;
            }
            if cmd.modifier.log_measurements {
                self.modifier.log_measurements = true;
            }
            self.commands.extend(cmd.commands);
        }
    }

    /// Tell the runtime that there are no commands.
    pub fn none() -> Self {
        Cmd {
            commands: vec![],
            modifier: Default::default(),
        }
    }

    /// Modify the Cmd such that whether or not it will update the view set by `should_update_view`
    /// when the cmd is executed in the program
    pub fn should_update_view(mut self, should_update_view: bool) -> Self {
        self.modifier.should_update_view = should_update_view;
        self
    }

    /// Modify the command such that it will not do an update on the view when it is executed.
    pub fn no_render(mut self) -> Self {
        self.modifier.should_update_view = false;
        self
    }

    /// Modify the command such that it will log measurement when it is executed
    pub fn measure(mut self) -> Self {
        self.modifier.log_measurements = true;
        self
    }

    /// Modify the Cmd such that it will log a measuregment when it is executed
    /// The `measurement_name` is set to distinguish the measurements from each other.
    pub fn measure_with_name(mut self, name: &str) -> Self {
        self = self.measure();
        self.modifier.measurement_name = name.to_string();
        self
    }
}

impl<DSP> Cmd<DSP>
where
    DSP: Clone + 'static,
{
    /// Executes the Cmd
    pub fn emit(self, program: &DSP) {
        for cb in self.commands {
            let program_clone = program.clone();
            cb(program_clone);
        }
    }
}

impl<DSP> Cmd<DSP> {
    /// Tell the runtime to execute subsequent update of the App with the message list.
    /// A single call to update the view is then executed thereafter.
    ///
    pub fn batch_msg<MSG>(msg_list: impl IntoIterator<Item = MSG>) -> Self
    where
        MSG: 'static,
        DSP: Dispatch<MSG> + Clone + 'static,
    {
        let msg_list = msg_list.into_iter().collect();
        Cmd::new(move |program: DSP| {
            program.dispatch_multiple(msg_list);
        })
    }
}

impl<DSP, MSG> From<Effects<MSG, ()>> for Cmd<DSP>
where
    MSG: 'static,
    DSP: Dispatch<MSG> + Clone + 'static,
{
    /// Convert Effects that has only follow ups
    fn from(effects: Effects<MSG, ()>) -> Self {
        // we can safely ignore the effects here
        // as there is no content on it.
        let Effects {
            local,
            external: _,
            modifier,
        } = effects;
        let mut cmd = Cmd::batch_msg(local);
        cmd.modifier = modifier;
        cmd
    }
}

impl<DSP, MSG> From<Vec<Effects<MSG, ()>>> for Cmd<DSP>
where
    MSG: 'static,
    DSP: Dispatch<MSG> + Clone + 'static,
{
    fn from(effects: Vec<Effects<MSG, ()>>) -> Self {
        Cmd::from(Effects::merge_all(effects))
    }
}
