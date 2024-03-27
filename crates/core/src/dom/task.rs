use crate::dom::dom_node::EventClosure;
use crate::dom::spawn_local;
use futures::channel::mpsc;
use futures::channel::mpsc::UnboundedReceiver;
use futures::StreamExt;
use std::future::Future;
use std::pin::Pin;
use wasm_bindgen::closure::Closure;

/// encapsulate anything a component can do
pub enum Task<MSG> {
    /// A task with one single resulting MSG
    Single(SingleTask<MSG>),
    /// A task with recurring resulting MSG
    Recurring(RecurringTask<MSG>),
}

impl<MSG> Task<MSG>
where
    MSG: 'static,
{
    ///
    pub fn single<F>(f: F) -> Self
    where
        F: Future<Output = MSG> + 'static,
    {
        Self::Single(SingleTask::new(f))
    }

    /// apply a function to the msg to create a different task which has a different msg
    pub fn map_msg<F, MSG2>(self, f: F) -> Task<MSG2>
    where
        F: Fn(MSG) -> MSG2 + 'static,
        MSG2: 'static,
    {
        match self {
            Self::Single(task) => Task::Single(task.map_msg(f)),
            Self::Recurring(task) => Task::Recurring(task.map_msg(f)),
        }
    }

    /// return the next value
    pub async fn next(&mut self) -> Option<MSG> {
        match self {
            Self::Single(task) => task.next().await,
            Self::Recurring(task) => task.next().await,
        }
    }
}

/// SingleTask is used to do asynchronous operations
pub struct SingleTask<MSG> {
    task: Pin<Box<dyn Future<Output = MSG>>>,
    /// a marker to indicate if the value of the future is awaited.
    /// any attempt to await it again will error,
    /// saying that the async function is resumed after completion.
    done: bool,
}

impl<MSG> SingleTask<MSG>
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
    fn map_msg<F, MSG2>(self, f: F) -> SingleTask<MSG2>
    where
        F: Fn(MSG) -> MSG2 + 'static,
        MSG2: 'static,
    {
        let task = self.task;
        SingleTask::new(async move {
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

impl<F, MSG> From<F> for SingleTask<MSG>
where
    F: Future<Output = MSG> + 'static,
    MSG: 'static,
{
    fn from(f: F) -> Self {
        SingleTask::new(f)
    }
}

pub struct RecurringTask<MSG> {
    pub(crate) receiver: UnboundedReceiver<MSG>,
    /// store the associated closures so it is not dropped before being event executed
    pub(crate) event_closures: Vec<EventClosure>,
    pub(crate) closures: Vec<Closure<dyn FnMut()>>,
}

impl<MSG> RecurringTask<MSG>
where
    MSG: 'static,
{
    async fn next(&mut self) -> Option<MSG> {
        self.receiver.next().await
    }

    /// apply a function to the msg to create a different task which has a different msg
    fn map_msg<F, MSG2>(self, f: F) -> RecurringTask<MSG2>
    where
        F: Fn(MSG) -> MSG2 + 'static,
        MSG2: 'static,
    {
        let (mut tx, rx) = mpsc::unbounded();
        let RecurringTask {
            mut receiver,
            event_closures,
            closures,
        } = self;
        spawn_local(async move {
            while let Some(msg) = receiver.next().await {
                tx.start_send(f(msg)).expect("must send");
            }
        });
        RecurringTask {
            receiver: rx,
            event_closures,
            closures,
        }
    }
}
