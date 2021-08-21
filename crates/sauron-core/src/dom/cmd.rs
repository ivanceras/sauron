//! provides functionalities for commands to be executed by the system, such as
//! when the application starts or after the application updates.
//!
use crate::Dispatch;
use crate::Effects;
use std::rc::Rc;

/// Cmd is a command to be executed by the system.
/// This is returned at the init function of a component and is executed right
/// after instantiation of that component.
/// Cmd required a DSP object which is the Program as an argument
/// The emit function is called with the program argument.
/// The callback is supplied with the program an is then executed/emitted.
#[derive(Clone)]
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
        let should_update_view = cmds.iter().any(|c| c.should_update_view);
        let log_measurements = cmds.iter().any(|c| c.log_measurements);
        let mut commands = vec![];
        for cmd in cmds {
            commands.extend(cmd.commands);
        }
        Self {
            commands,
            should_update_view,
            log_measurements,
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

    /// map each item in pmsg_list such that Vec<PMSG> becomes Vec<MSG>
    pub fn map_msgs<F, PMSG, MSG>(pmsg_list: Vec<PMSG>, f: F) -> Self
    where
        DSP: Dispatch<MSG> + Clone + 'static,
        MSG: Clone + 'static,
        PMSG: 'static,
        F: Fn(PMSG) -> MSG + 'static,
    {
        Self::batch_msg(pmsg_list.into_iter().map(f).collect())
    }

    /// Convert effects into Cmd
    pub fn map_follow_ups<F, PMSG, MSG>(
        effects: Effects<MSG, PMSG>,
        f: F,
    ) -> Self
    where
        DSP: Dispatch<PMSG> + Clone + 'static,
        MSG: Clone + 'static,
        PMSG: Clone + 'static,
        F: Fn(MSG) -> PMSG + 'static,
    {
        let Effects {
            follow_ups,
            effects,
        } = effects;

        Cmd::batch_msg(
            effects
                .into_iter()
                .chain(follow_ups.into_iter().map(f))
                .collect(),
        )
    }
}

impl<DSP, MSG> From<Effects<MSG, ()>> for Cmd<DSP>
where
    MSG: Clone + 'static,
    DSP: Dispatch<MSG> + Clone + 'static,
{
    /// Create a Cmd from effects
    fn from(effects: Effects<MSG, ()>) -> Self {
        let Effects {
            follow_ups,
            effects: _,
        } = effects;
        Cmd::batch_msg(follow_ups)
    }
}
