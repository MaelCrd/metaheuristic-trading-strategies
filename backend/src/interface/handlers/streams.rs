use rocket::get;
use rocket::response::stream::TextStream;
use rocket::State;
use serde::Serialize;
use tokio::sync::watch;

#[derive(Debug, Clone, Serialize)]
pub struct TaskUpdate {
    pub task_id: i32,
    pub state: String,
}

impl std::fmt::Display for TaskUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ \"task_id\": {}, \"state\": \"{}\" }}",
            self.task_id, self.state
        )
    }
}

#[derive(Debug)]
pub struct TaskStateChannel {
    pub sender: watch::Sender<TaskUpdate>,
}

impl TaskStateChannel {
    pub fn new() -> Self {
        Self {
            sender: watch::channel(TaskUpdate {
                task_id: 0,
                state: "created".to_string(),
            })
            .0,
        }
    }
}

/// Send task updates to the client
#[get("/task-updates")]
pub fn hello(state: &State<TaskStateChannel>) -> TextStream![String] {
    let mut rx = state.sender.subscribe();
    TextStream! {
        while let Ok(_) = rx.changed().await {
            let a = rx.borrow().clone();
            yield format!("{}", a).to_string();
        }
    }
}
