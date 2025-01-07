use chrono::DateTime;
use chrono::TimeZone;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::Pool;
use sqlx::Postgres;
use std::collections::HashMap;
use std::env;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use super::threads::ThreadStatus;
use crate::interface::handlers::tasks;
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

struct TasksProcessor {
    pending_tasks: Arc<Mutex<Vec<i32>>>,
    cancelling_tasks: Arc<Mutex<Vec<i32>>>,
    running_tasks: Arc<Mutex<Vec<i32>>>,
}

impl TasksProcessor {
    fn new() -> Self {
        Self {
            pending_tasks: Arc::new(Mutex::new(Vec::<i32>::new())),
            cancelling_tasks: Arc::new(Mutex::new(Vec::<i32>::new())),
            running_tasks: Arc::new(Mutex::new(Vec::<i32>::new())),
        }
    }

    fn add_pending_task(&self, task_id: i32) -> bool {
        // Add if not already present
        if !self.pending_tasks.lock().unwrap().contains(&task_id) {
            self.pending_tasks.lock().unwrap().push(task_id);
            return true;
        }
        false
    }

    fn add_cancelling_task(&self, task_id: i32) -> bool {
        // Add if not already present
        if !self.cancelling_tasks.lock().unwrap().contains(&task_id) {
            self.cancelling_tasks.lock().unwrap().push(task_id);
            return true;
        }
        false
    }

    fn add_running_task(&self, task_id: i32) -> bool {
        // Add if not already present
        if !self.running_tasks.lock().unwrap().contains(&task_id) {
            self.running_tasks.lock().unwrap().push(task_id);
            return true;
        }
        false
    }

    fn remove_pending_task(&self, task_id: i32) {
        let mut pending_tasks = self.pending_tasks.lock().unwrap();
        if let Some(index) = pending_tasks.iter().position(|&x| x == task_id) {
            pending_tasks.remove(index);
        }
    }

    fn remove_cancelling_task(&self, task_id: i32) {
        let mut cancelling_tasks = self.cancelling_tasks.lock().unwrap();
        if let Some(index) = cancelling_tasks.iter().position(|&x| x == task_id) {
            cancelling_tasks.remove(index);
        }
    }

    fn remove_running_task(&self, task_id: i32) {
        let mut running_tasks = self.running_tasks.lock().unwrap();
        if let Some(index) = running_tasks.iter().position(|&x| x == task_id) {
            running_tasks.remove(index);
        }
    }

    fn get_pending_tasks(&self) -> Vec<i32> {
        self.pending_tasks.lock().unwrap().clone()
    }

    fn get_cancelling_tasks(&self) -> Vec<i32> {
        self.cancelling_tasks.lock().unwrap().clone()
    }

    fn get_running_tasks(&self) -> Vec<i32> {
        self.running_tasks.lock().unwrap().clone()
    }

    fn display_tasks(&self) {
        println!("Pending Tasks: {:?}", self.get_pending_tasks());
        println!("Cancelling Tasks: {:?}", self.get_cancelling_tasks());
        println!("Running Tasks: {:?}", self.get_running_tasks());
    }
}

pub struct TaskManager {
    pool: Pool<Postgres>,
    tasks_processor: TasksProcessor,
    statuses: Arc<Mutex<HashMap<i32, ThreadStatus>>>,
    max_threads: usize,
}

impl TaskManager {
    pub fn new_with_pool(pool: Pool<Postgres>) -> Self {
        Self {
            pool,
            tasks_processor: TasksProcessor::new(),
            statuses: Arc::new(Mutex::new(HashMap::new())),
            max_threads: 2,
        }
    }

    pub async fn new() -> Self {
        Self {
            pool: sqlx::postgres::PgPoolOptions::new()
                .max_connections(5)
                .connect(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
                .await
                .unwrap(),
            tasks_processor: TasksProcessor::new(),
            statuses: Arc::new(Mutex::new(HashMap::new())),
            max_threads: 2,
        }
    }

    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Started listening for tasks...");

        // self.test();

        // Periodically check for new tasks (every 2 seconds)
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(2));
        loop {
            let start_time = Instant::now();
            interval.tick().await;
            self.check_for_tasks().await?;
            println!("Elapsed time: {:?}", start_time.elapsed());
        }
    }

    async fn check_completed_start_pending(&self) {
        // When a task is completed, it should be removed from the pending list
        let running_tasks = self.tasks_processor.get_running_tasks();
        for (task_id, status) in self.get_all_statuses() {
            println!("Task ID: {}, Status: {:?}", task_id, status);
            if running_tasks.contains(&task_id) && status.is_complete {
                // Remove the task from the running list
                let res = tasks::update_task_state(
                    &rocket::State::from(&self.pool),
                    task_id,
                    TaskState::Completed,
                )
                .await;
                self.tasks_processor.remove_running_task(task_id);
                println!("Task completed - ID: {}", task_id);
                // Remove the task from the statuses
                self.statuses.lock().unwrap().remove(&task_id);
            }
        }

        // If tasks are pending, start them
        let pending_tasks = self.tasks_processor.get_pending_tasks();
        for task_id in pending_tasks {
            let _ = self.handle_task_pending(task_id).await;
        }
    }

    async fn check_for_tasks(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Check for completed tasks
        self.check_completed_start_pending().await;

        // Get all tasks
        let tasks = tasks::get_tasks(&rocket::State::from(&self.pool), None)
            .await
            .unwrap();
        println!("---------------------------");
        for task in tasks.into_inner() {
            match task.state {
                TaskState::Pending => {
                    if self.tasks_processor.add_pending_task(task.id) {
                        println!("Task pending - ID: {}", task.id);
                        let _ = self.handle_task_pending(task.id).await;
                    }
                }
                TaskState::Cancelling => {
                    if self.tasks_processor.add_cancelling_task(task.id) {
                        println!("Task cancelling - ID: {}", task.id);
                        let _ = self.handle_task_cancelling(task.id).await;
                    }
                }
                TaskState::Running => {
                    if self.tasks_processor.add_running_task(task.id) {
                        println!("Task running - ID: {}", task.id);
                        let _ = self.handle_task_running(task.id).await;
                    }
                }
                _ => {}
            }
        }

        self.tasks_processor.display_tasks();
        Ok(())
    }

    async fn handle_task_pending(&self, task_id: i32) -> Result<(), Box<dyn std::error::Error>> {
        // When a task is pending, it should be started

        // If the maximum number of threads is reached, do not start the task
        if self.tasks_processor.get_running_tasks().len() >= self.max_threads {
            return Err("Maximum number of threads reached".into());
        }

        // Start the task
        let res = tasks::update_task_state(
            &rocket::State::from(&self.pool),
            task_id,
            TaskState::Running,
        )
        .await;

        if self.tasks_processor.add_running_task(task_id) && res.is_ok() {
            // Remove the task from the pending list
            self.tasks_processor.remove_pending_task(task_id);
            // Start the task in a new thread

            // Spawn a successful thread
            self.spawn_monitored_thread(task_id, || {
                // thread::sleep(Duration::from_secs(2));
                let mut i: i64 = 0;
                for _ in 0..2147483645 {
                    i += 1;
                }
                Ok("Task completed successfully".to_string())
            });

            return Ok(());
        }

        Err("Error starting task".into())
    }

    async fn handle_task_cancelling(&self, task_id: i32) -> Result<(), Box<dyn std::error::Error>> {
        // let task = Task::get_by_id(&self.pool, task_id).await?;
        // task.execute().await?;

        Ok(())
    }

    async fn handle_task_running(&self, task_id: i32) -> Result<(), Box<dyn std::error::Error>> {
        // Should not be called for a running task,
        // only if the task is in the running state but not actually running
        // So : task failed

        // Set the task state to failed
        let res =
            tasks::update_task_state(&rocket::State::from(&self.pool), task_id, TaskState::Failed)
                .await;

        if res.is_ok() {
            // Remove the task from the running list
            self.tasks_processor.remove_running_task(task_id);
        }

        println!("Task failed - ID: {}", task_id);

        res
    }

    fn spawn_monitored_thread<F>(&self, task_id: i32, work: F)
    where
        F: FnOnce() -> Result<String, String> + Send + 'static,
    {
        let status_map = Arc::clone(&self.statuses);

        thread::spawn(move || {
            let start_time = Instant::now();

            // Initialize thread status
            {
                let mut statuses = status_map.lock().unwrap();
                statuses.insert(
                    task_id,
                    ThreadStatus {
                        is_complete: false,
                        success: false,
                        start_time,
                        duration: Duration::from_secs(0),
                        result: String::new(),
                    },
                );
            }

            // Execute the work
            let work_result = work();

            // Update thread status
            let mut statuses = status_map.lock().unwrap();
            statuses.insert(
                task_id,
                ThreadStatus {
                    is_complete: true,
                    success: work_result.is_ok(),
                    start_time,
                    duration: start_time.elapsed(),
                    result: match work_result {
                        Ok(s) => s,
                        Err(e) => e,
                    },
                },
            );
        });
        println!("Spawned thread: {}", task_id);
    }

    fn get_status(&self, task_id: i32) -> Option<ThreadStatus> {
        self.statuses.lock().unwrap().get(&task_id).cloned()
    }

    fn get_all_statuses(&self) -> HashMap<i32, ThreadStatus> {
        self.statuses.lock().unwrap().clone()
    }
}
