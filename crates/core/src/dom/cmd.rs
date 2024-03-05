//! provides functionalities for commands to be executed by the system, such as
//! when the application starts or after the application updates.
//!
use crate::dom::Program;
use crate::dom::{Application, Effects, Modifier, Task};
use wasm_bindgen_futures::spawn_local;

/// Cmd is a command to be executed by the system.
/// This is returned at the init function of a component and is executed right
/// after instantiation of that component.
/// Cmd required a DSP object which is the Program as an argument
/// The emit function is called with the program argument.
/// The callback is supplied with the program an is then executed/emitted.
pub struct Cmd<APP, MSG>
where
    MSG: 'static,
{
    /// the functions that would be executed when this Cmd is emited
    #[allow(clippy::type_complexity)]
    pub(crate) commands: Vec<Box<dyn FnOnce(Program<APP, MSG>)>>,
    pub(crate) modifier: Modifier,
}

impl<APP, MSG> Cmd<APP, MSG>
where
    MSG: 'static,
    APP: Application<MSG>,
{
    /// creates a new Cmd from a function
    pub fn new<F>(f: F) -> Self
    where
        F: FnOnce(Program<APP, MSG>) + 'static,
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
        let mut modifier = Modifier::default();
        for cmd in cmds {
            modifier.coalesce(&cmd.modifier);
            commands.extend(cmd.commands);
        }
        Self { commands, modifier }
    }

    /// Add a cmd
    pub fn push(&mut self, cmd: Self) {
        self.append([cmd])
    }

    /// Append more cmd into this cmd and return self
    pub fn append(&mut self, cmds: impl IntoIterator<Item = Self>) {
        for cmd in cmds {
            self.modifier.coalesce(&cmd.modifier);
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
        self.modifier.measurement_name = name.to_string();
        self.measure()
    }

    /// Executes the Cmd
    pub(crate) fn emit(self, program: Program<APP, MSG>) {
        for cb in self.commands {
            let program_clone = program.clone();
            cb(program_clone);
        }
    }

    /// Tell the runtime to execute subsequent update of the App with the message list.
    /// A single call to update the view is then executed thereafter.
    ///
    pub fn batch_msg(msg_list: impl IntoIterator<Item = MSG>) -> Self {
        let msg_list: Vec<MSG> = msg_list.into_iter().collect();
        Cmd::new(move |mut program| {
            program.dispatch_multiple(msg_list);
        })
    }
}

impl<APP, MSG> From<Effects<MSG, ()>> for Cmd<APP, MSG>
where
    MSG: 'static,
    APP: Application<MSG>,
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

        let mut cmd = Cmd::batch(local.into_iter().map(Cmd::from));
        cmd.modifier = modifier;
        cmd
    }
}

impl<APP, MSG, IN> From<IN> for Cmd<APP, MSG>
where
    MSG: 'static,
    APP: Application<MSG>,
    IN: IntoIterator<Item = Effects<MSG, ()>>,
{
    fn from(effects: IN) -> Self {
        Cmd::batch(effects.into_iter().map(Cmd::from))
    }
}

impl<APP, MSG> From<Task<MSG>> for Cmd<APP, MSG>
where
    MSG: 'static,
    APP: Application<MSG>,
{
    fn from(mut task: Task<MSG>) -> Self {
        Cmd::new(move |mut program| {
            spawn_local(async move {
                while let Some(msg) = task.next().await {
                    program.dispatch(msg)
                }
            });
        })
    }
}
