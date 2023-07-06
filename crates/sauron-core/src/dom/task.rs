use std::future::Future;
use std::pin::Pin;

/// Task is used to do asynchronous operations
pub struct Task<MSG>{
    pub(crate) task: Pin<Box<dyn Future<Output = MSG>>>,
}

impl<MSG> Task<MSG> where MSG: 'static{

    /// create a new task from a function which returns a future
    pub fn new<F>(f:F) -> Self where F: Future<Output = MSG> + 'static{
        Self{
            task: Box::pin(f),
        }
    }

    /// apply a function to the msg to create a different task which has a different msg
    pub fn map_msg<F, MSG2>(self, f: F) -> Task<MSG2>
    where
        F: Fn(MSG) -> MSG2 + 'static,
        MSG2: 'static
    {
        let task = self.task;
        Task::new(async move{
            let msg = task.await;
            f(msg)
        })
    }
}
