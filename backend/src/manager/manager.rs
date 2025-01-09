use sqlx::Pool;
use sqlx::Postgres;
use std::collections::HashMap;
use std::env;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use std::thread;
use std::time::{Duration, Instant};

use super::objects::{TaskLists, ThreadStatus};
use crate::{interface::handlers::tasks, objects::objects::TaskState};

pub struct TaskManager {
    pub pool: Pool<Postgres>,
    tasks_processor: TaskLists,
    statuses: Arc<Mutex<HashMap<i32, ThreadStatus>>>,
    cancel_flags: Arc<Mutex<HashMap<i32, Arc<AtomicBool>>>>,
    max_threads: usize,
}

impl TaskManager {
    pub fn new_with_pool(pool: Pool<Postgres>) -> Self {
        Self {
            pool,
            tasks_processor: TaskLists::new(),
            statuses: Arc::new(Mutex::new(HashMap::new())),
            cancel_flags: Arc::new(Mutex::new(HashMap::new())),
            max_threads: 8,
        }
    }

    pub async fn new() -> Self {
        Self {
            pool: sqlx::postgres::PgPoolOptions::new()
                .max_connections(5)
                .connect(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
                .await
                .unwrap(),
            tasks_processor: TaskLists::new(),
            statuses: Arc::new(Mutex::new(HashMap::new())),
            cancel_flags: Arc::new(Mutex::new(HashMap::new())),
            max_threads: 8,
        }
    }

    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Started listening for tasks...");

        // Periodically check for new tasks (every 2 seconds)
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(2));
        loop {
            interval.tick().await;
            self.check_for_tasks().await?;
        }
    }

    async fn check_completed_start_pending(&self) {
        // When a task is completed, it should be removed from the pending list
        let running_tasks = self.tasks_processor.get_running_tasks();
        for (task_id, status) in self.get_all_statuses() {
            println!("Task ID: {}, Status: {:?}", task_id, status);
            if running_tasks.contains(&task_id) && status.is_complete {
                // Remove the task from the running list
                let new_state = match status.success {
                    true => {
                        // Set the task state to completed
                        TaskState::Completed
                    }
                    false => {
                        // Set the task state to failed
                        TaskState::Failed
                    }
                };
                let res =
                    tasks::update_task_state(&rocket::State::from(&self.pool), task_id, new_state)
                        .await;

                if res.is_ok() {
                    self.tasks_processor.remove_running_task(task_id);
                    match status.success {
                        true => {
                            println!("Task completed - ID: {}", task_id);
                        }
                        false => {
                            println!("Task failed - ID: {} ({})", task_id, status.result);
                        }
                    }
                    // Remove the task from the statuses
                    self.statuses.lock().unwrap().remove(&task_id);
                } else {
                    println!("Error removing task - ID: {}", task_id);
                }
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
        // println!("---------------------------");
        self.check_completed_start_pending().await;

        // Get all tasks
        let tasks = tasks::get_tasks(&rocket::State::from(&self.pool), None)
            .await
            .unwrap();
        // println!("---------------------------");
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

        // self.tasks_processor.display_tasks();
        Ok(())
    }

    async fn start_task(&self, task_id: i32) {
        // //Spawn a successful thread
        // self.spawn_monitored_thread(task_id, |should_cancel| {
        //     // thread::sleep(Duration::from_secs(2));
        //     let mut i: i64 = 0;
        //     for _ in 0..21474836 {
        //         for __ in 0..100 {
        //             i += 1;
        //         }
        //         // Check if we should cancel
        //         if should_cancel.load(Ordering::SeqCst) {
        //             return Err("Task was cancelled".to_string());
        //         }
        //     }
        //     Ok("Task completed successfully".to_string())
        // });

        let pool_state = rocket::State::from(&self.pool);
        let task_list = tasks::get_tasks(&pool_state, Some(task_id.to_string()))
            .await
            .unwrap();
        let tasks = task_list.get(0).cloned();
        if tasks.is_none() {
            return;
        }

        let task = tasks.unwrap();

        self.spawn_monitored_thread(task_id, move |should_cancel| {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async { task.execute(should_cancel).await })
        });
    }

    async fn handle_task_pending(
        &self,
        task_id: i32,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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

            // Start the task
            self.start_task(task_id).await;

            return Ok(());
        }

        Err("Error starting task".into())
    }

    async fn handle_task_cancelling(&self, task_id: i32) -> Result<(), Box<dyn std::error::Error>> {
        // If task is pending, it should be cancelled
        if self.tasks_processor.get_pending_tasks().contains(&task_id) {
            // Remove the task from the pending list
            self.tasks_processor.remove_pending_task(task_id);
        }
        // If task is running, it should be cancelled
        else if self.tasks_processor.get_running_tasks().contains(&task_id) {
            // Remove the task from the running list
            self.tasks_processor.remove_running_task(task_id);
            // Set the task's cancel atomic flag to true
            if let Some(flag) = self.cancel_flags.lock().unwrap().get(&task_id) {
                flag.store(true, Ordering::SeqCst);
            }
        }

        // Set the task state to cancelled
        let res = tasks::update_task_state(
            &rocket::State::from(&self.pool),
            task_id,
            TaskState::Cancelled,
        )
        .await;
        if res.is_ok() {
            // Remove the task from the cancelling list
            self.tasks_processor.remove_cancelling_task(task_id);
            println!("Task cancelled - ID: {}", task_id);
        }

        Ok(())
    }

    async fn handle_task_running(
        &self,
        task_id: i32,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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
        F: FnOnce(Arc<AtomicBool>) -> Result<String, String> + Send + 'static,
    {
        let status_map = Arc::clone(&self.statuses);
        let cancel_flags = Arc::clone(&self.cancel_flags);

        // Create a new atomic flag for this task
        let should_cancel = Arc::new(AtomicBool::new(false));
        cancel_flags
            .lock()
            .unwrap()
            .insert(task_id, Arc::clone(&should_cancel));

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
            let work_result = work(should_cancel);

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

    // fn get_status(&self, task_id: i32) -> Option<ThreadStatus> {
    //     self.statuses.lock().unwrap().get(&task_id).cloned()
    // }

    fn get_all_statuses(&self) -> HashMap<i32, ThreadStatus> {
        self.statuses.lock().unwrap().clone()
    }
}
