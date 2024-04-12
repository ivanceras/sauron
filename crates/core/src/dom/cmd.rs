use crate::dom::Effects;
use futures::channel::mpsc;
use futures::channel::mpsc::UnboundedReceiver;
use futures::StreamExt;
use std::future::Future;
use std::pin::Pin;
#[cfg(feature = "with-dom")]
use wasm_bindgen::closure::Closure;

/// Cmd is a way to tell the Runtime that something needs to be executed
pub struct Cmd<MSG> {
    /// commands
    pub(crate) commands: Vec<Command<MSG>>,
}

/// encapsulate anything a component can do
pub enum Command<MSG> {
    /// A task with one single resulting MSG
    Action(Action<MSG>),
    #[cfg(feature = "with-dom")]
    /// A task with recurring resulting MSG
    Sub(Sub<MSG>),
}

impl<MSG> Cmd<MSG>
where
    MSG: 'static,
{
    /// maps to a once future
    pub fn new<F>(f: F) -> Self
    where
        F: Future<Output = MSG> + 'static,
    {
        Self::once(f)
    }

    /// Creates a Cmd which expects to be polled only once
    pub fn once<F>(f: F) -> Self
    where
        F: Future<Output = MSG> + 'static,
    {
        Self {
            commands: vec![Command::single(f)],
        }
    }
    /// Creates a Cmd which will be polled multiple times
    pub fn recurring(
        rx: UnboundedReceiver<MSG>,
        event_closure: Closure<dyn FnMut(web_sys::Event)>,
    ) -> Self {
        Self {
            commands: vec![Command::sub(rx, event_closure)],
        }
    }

    /// map the msg of this Cmd such that Cmd<MSG> becomes Cmd<MSG2>.
    pub fn map_msg<F, MSG2>(self, f: F) -> Cmd<MSG2>
    where
        F: Fn(MSG) -> MSG2 + 'static + Clone,
        MSG2: 'static,
    {
        Cmd {
            commands: self
                .commands
                .into_iter()
                .map(|t| t.map_msg(f.clone()))
                .collect(),
        }
    }

    /// batch together multiple Cmd into one task
    pub fn batch(tasks: impl IntoIterator<Item = Self>) -> Self {
        let mut commands = vec![];
        for task in tasks.into_iter() {
            commands.extend(task.commands);
        }
        Self { commands }
    }

    ///
    pub fn none() -> Self {
        Self { commands: vec![] }
    }
}

impl<MSG> From<Effects<MSG, ()>> for Cmd<MSG>
where
    MSG: 'static,
{
    /// Convert Effects that has only follow ups
    fn from(effects: Effects<MSG, ()>) -> Self {
        // we can safely ignore the effects here
        // as there is no content on it.
        let Effects { local, external: _ } = effects;

        Cmd::batch(local.into_iter().map(Cmd::from))
    }
}

impl<MSG> Command<MSG>
where
    MSG: 'static,
{
    ///
    pub fn single<F>(f: F) -> Self
    where
        F: Future<Output = MSG> + 'static,
    {
        Self::Action(Action::new(f))
    }

    ///
    #[cfg(feature = "with-dom")]
    pub fn sub(
        rx: UnboundedReceiver<MSG>,
        event_closure: Closure<dyn FnMut(web_sys::Event)>,
    ) -> Self {
        Self::Sub(Sub {
            receiver: rx,
            event_closure,
        })
    }

    /// apply a function to the msg to create a different task which has a different msg
    pub fn map_msg<F, MSG2>(self, f: F) -> Command<MSG2>
    where
        F: Fn(MSG) -> MSG2 + 'static,
        MSG2: 'static,
    {
        match self {
            Self::Action(task) => Command::Action(task.map_msg(f)),
            #[cfg(feature = "with-dom")]
            Self::Sub(task) => Command::Sub(task.map_msg(f)),
        }
    }

    /// return the next value
    pub async fn next(&mut self) -> Option<MSG> {
        match self {
            Self::Action(task) => task.next().await,
            #[cfg(feature = "with-dom")]
            Self::Sub(task) => task.next().await,
        }
    }
}

/// Action is used to do asynchronous operations
pub struct Action<MSG> {
    task: Pin<Box<dyn Future<Output = MSG>>>,
    /// a marker to indicate if the value of the future is awaited.
    /// any attempt to await it again will error,
    /// saying that the async function is resumed after completion.
    done: bool,
}

impl<MSG> Action<MSG>
where
    MSG: 'static,
{
    /// create a new task from a function which returns a future
    fn new<F>(f: F) -> Self
    where
        F: Future<Output = MSG> + 'static,
    {
        Self {
            task: Box::pin(f),
            done: false,
        }
    }

    /// apply a function to the msg to create a different task which has a different msg
    fn map_msg<F, MSG2>(self, f: F) -> Action<MSG2>
    where
        F: Fn(MSG) -> MSG2 + 'static,
        MSG2: 'static,
    {
        let task = self.task;
        Action::new(async move {
            let msg = task.await;
            f(msg)
        })
    }

    /// get the next value
    async fn next(&mut self) -> Option<MSG> {
        // return None is already done since awaiting it again is an error
        if self.done {
            None
        } else {
            let msg = self.task.as_mut().await;
            // mark as done
            self.done = true;
            Some(msg)
        }
    }
}

impl<F, MSG> From<F> for Action<MSG>
where
    F: Future<Output = MSG> + 'static,
    MSG: 'static,
{
    fn from(f: F) -> Self {
        Action::new(f)
    }
}

#[cfg(feature = "with-dom")]
/// Sub is a recurring operation
pub struct Sub<MSG> {
    pub(crate) receiver: UnboundedReceiver<MSG>,
    /// store the associated closures so it is not dropped before being event executed
    pub(crate) event_closure: Closure<dyn FnMut(web_sys::Event)>,
}

#[cfg(feature = "with-dom")]
impl<MSG> Sub<MSG>
where
    MSG: 'static,
{
    async fn next(&mut self) -> Option<MSG> {
        self.receiver.next().await
    }

    /// apply a function to the msg to create a different task which has a different msg
    fn map_msg<F, MSG2>(self, f: F) -> Sub<MSG2>
    where
        F: Fn(MSG) -> MSG2 + 'static,
        MSG2: 'static,
    {
        let (mut tx, rx) = mpsc::unbounded();
        let Sub {
            mut receiver,
            event_closure,
        } = self;

        crate::dom::spawn_local(async move {
            while let Some(msg) = receiver.next().await {
                tx.start_send(f(msg)).expect("must send");
            }
        });

        Sub {
            receiver: rx,
            event_closure,
        }
    }
}
