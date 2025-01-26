use rocket::get;
use rocket::response::stream::TextStream;
use rocket::tokio::time::{interval, Duration};
use rocket::State;
use tokio::sync::watch;

#[derive(Debug)]
pub struct TaskStateChannel {
    pub sender: watch::Sender<String>,
}

impl TaskStateChannel {
    pub fn new() -> Self {
        Self {
            sender: watch::channel("".to_string()).0,
        }
    }
}

/// Produce an infinite series of `"hello"`s, one per second.
#[get("/infinite-hellos")]
pub fn hello(state: &State<TaskStateChannel>) -> TextStream![String] {
    // TextStream! {
    //     let mut interval = interval(Duration::from_secs(1));
    //     loop {
    //         yield "hello";
    //         interval.tick().await;
    //     }
    // }
    let mut rx = state.sender.subscribe();
    TextStream! {
        while let Ok(_) = rx.changed().await {
            let a = rx.borrow().clone();
            yield a;
        }
    }
}
