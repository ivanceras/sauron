//! provides functionalities for commands to be executed by the system, such as
//! when the application starts or after the application updates.
//!
use crate::Callback;
use crate::Dispatch;
use crate::Effects;

/// Cmd is a command to be executed by the system.
/// This is returned at the init function of a component and is executed right
/// after instantiation of that component.
/// Cmd required a DSP object which is the Program as an argument
/// The emit function is called with the program argument.
/// The callback is supplied with the program an is then executed/emitted.
#[derive(Clone)]
pub struct Cmd<DSP> {
    /// the functions that would be executed when this Cmd is emited
    pub commands: Vec<Callback<DSP, ()>>,
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
    /// into the [`Measurement`](crate::Measurement) passed in
    /// [`Application::measurement`](crate::Application::measurement)
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
    DSP: Clone + 'static,
{
    /// creates a new Cmd from a function
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(DSP) + 'static,
    {
        Self {
            commands: vec![Callback::from(f)],
            modifier: Default::default(),
        }
    }

    /// creates a unified Cmd which batches all the other Cmds in one.
    pub fn batch(cmds: Vec<Self>) -> Self {
        let should_update_view =
            cmds.iter().any(|c| c.modifier.should_update_view);
        let log_measurements = cmds.iter().any(|c| c.modifier.log_measurements);
        let mut commands = vec![];
        for cmd in cmds {
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

    /// Append more cmd into this cmd and return self
    pub fn append(mut self, cmds: Vec<Self>) -> Self {
        self.modifier.should_update_view =
            cmds.iter().any(|c| c.modifier.should_update_view);
        self.modifier.log_measurements =
            cmds.iter().any(|c| c.modifier.log_measurements);
        for cmd in cmds {
            self.commands.extend(cmd.commands);
        }
        self
    }

    /// A Cmd with no callback, similar to NoOp.
    pub fn none() -> Self {
        Cmd {
            commands: vec![],
            modifier: Default::default(),
        }
    }

    /// Executes the Cmd
    pub fn emit(self, program: &DSP) {
        for cb in self.commands {
            let program_clone = program.clone();
            cb.emit(program_clone);
        }
    }

    /// Modify the Cmd such that whether or not it will update the view set by `should_update_view`
    /// when the cmd is executed in the program
    pub fn should_update_view(mut self, should_update_view: bool) -> Self {
        self.modifier.should_update_view = should_update_view;
        self
    }

    /// Modify the Cmd such that it will not do an update on the view when it is executed
    pub fn no_render(mut self) -> Self {
        self.modifier.should_update_view = false;
        self
    }

    /// Modify the Cmd such that it will log measurement when it is executed
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

impl<DSP> Cmd<DSP> {
    /// batch dispatch this msg on the next update loop
    pub fn batch_msg<MSG>(msg_list: Vec<MSG>) -> Self
    where
        MSG: Clone + 'static,
        DSP: Dispatch<MSG> + Clone + 'static,
    {
        Cmd::new(move |program: DSP| {
            for msg in msg_list.iter() {
                program.dispatch(msg.clone());
            }
        })
    }
}

impl<DSP, MSG> From<Effects<MSG, ()>> for Cmd<DSP>
where
    MSG: Clone + 'static,
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
