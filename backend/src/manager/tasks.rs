use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};

use super::TaskManager;

impl TaskManager {
    pub async fn start_task(&self, task_id: i32) {
        //
        println!("[TASK {}] : Starting", task_id);

        //Spawn a successful thread
        self.spawn_monitored_thread(task_id, |should_cancel| {
            // thread::sleep(Duration::from_secs(2));
            let mut i: i64 = 0;
            for _ in 0..21474836 {
                for __ in 0..100 {
                    i += 1;
                }
                // Check if we should cancel
                if should_cancel.load(Ordering::SeqCst) {
                    return Err("Task was cancelled".to_string());
                }
            }
            Ok("Task completed successfully".to_string())
        });
    }
}
