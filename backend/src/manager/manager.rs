use chrono::DateTime;
use chrono::TimeZone;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgListener;
use sqlx::Pool;
use sqlx::Postgres;
use std::env;

use crate::objects::objects::{Task, TaskState};

#[derive(Debug, Serialize, Deserialize)]
enum TaskOperation {
    Insert,
    Update,
    Delete,
}

impl TaskOperation {
    fn from_str(s: &str) -> Result<Self, Box<dyn std::error::Error>> {
        match s {
            "Insert" => Ok(TaskOperation::Insert),
            "Update" => Ok(TaskOperation::Update),
            "Delete" => Ok(TaskOperation::Delete),
            _ => Err("Invalid TaskOperation".into()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct TaskNotification {
    operation: TaskOperation,
    task_id: i32,
    state: Option<TaskState>,
    created_at: Option<DateTime<Utc>>,
}

pub struct TaskManager {
    pool: Pool<Postgres>,
}

impl TaskManager {
    pub fn new_with_pool(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn new() -> Self {
        Self {
            pool: sqlx::postgres::PgPoolOptions::new()
                .max_connections(5)
                .connect(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
                .await
                .unwrap(),
        }
    }

    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Create a listener
        let mut listener = PgListener::connect_with(&self.pool).await?;

        // Listen for notifications on the "new_task" channel
        listener.listen("task_changes").await?;

        println!("Started listening for new tasks...");

        // Process notifications
        while let Ok(notification) = listener.recv().await {
            let notification: serde_json::Value = serde_json::from_str(notification.payload())?;
            let task_notification: TaskNotification = TaskNotification {
                operation: TaskOperation::from_str(notification["operation"].as_str().unwrap())?,
                task_id: notification["task_id"].as_i64().unwrap() as i32,
                state: Some(TaskState::parse_from(
                    notification["state"].as_str().unwrap(),
                )),
                created_at: Some(Utc::now()),
            };

            match task_notification.operation {
                TaskOperation::Insert => {
                    println!("New task created - ID: {}", task_notification.task_id);
                    self.handle_new_task(task_notification.task_id).await?;
                }
                TaskOperation::Update => {
                    println!(
                        "Task updated - ID: {}, New Status: {:?}",
                        task_notification.task_id, task_notification.state
                    );
                    self.handle_task_update(task_notification.task_id).await?;
                }
                TaskOperation::Delete => {
                    println!("Task deleted - ID: {}", task_notification.task_id);
                    self.handle_task_deletion(task_notification.task_id).await?;
                }
            }
        }

        Ok(())
    }

    async fn handle_new_task(&self, task_id: i32) -> Result<(), Box<dyn std::error::Error>> {
        // let task = Task::get_by_id(&self.pool, task_id).await?;
        // task.execute().await?;

        Ok(())
    }

    async fn handle_task_update(&self, task_id: i32) -> Result<(), Box<dyn std::error::Error>> {
        // let task = Task::get_by_id(&self.pool, task_id).await?;
        // task.execute().await?;

        Ok(())
    }

    async fn handle_task_deletion(&self, task_id: i32) -> Result<(), Box<dyn std::error::Error>> {
        // let task = Task::get_by_id(&self.pool, task_id).await?;
        // task.execute().await?;

        Ok(())
    }
}
