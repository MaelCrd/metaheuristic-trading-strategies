use std::sync::{Arc, Mutex};

pub struct TasksProcessor {
    pending_tasks: Arc<Mutex<Vec<i32>>>,
    cancelling_tasks: Arc<Mutex<Vec<i32>>>,
    running_tasks: Arc<Mutex<Vec<i32>>>,
}

impl TasksProcessor {
    pub fn new() -> Self {
        Self {
            pending_tasks: Arc::new(Mutex::new(Vec::<i32>::new())),
            cancelling_tasks: Arc::new(Mutex::new(Vec::<i32>::new())),
            running_tasks: Arc::new(Mutex::new(Vec::<i32>::new())),
        }
    }

    pub fn add_pending_task(&self, task_id: i32) -> bool {
        // Add if not already present
        if !self.pending_tasks.lock().unwrap().contains(&task_id) {
            self.pending_tasks.lock().unwrap().push(task_id);
            return true;
        }
        false
    }

    pub fn add_cancelling_task(&self, task_id: i32) -> bool {
        // Add if not already present
        if !self.cancelling_tasks.lock().unwrap().contains(&task_id) {
            self.cancelling_tasks.lock().unwrap().push(task_id);
            return true;
        }
        false
    }

    pub fn add_running_task(&self, task_id: i32) -> bool {
        // Add if not already present
        if !self.running_tasks.lock().unwrap().contains(&task_id) {
            self.running_tasks.lock().unwrap().push(task_id);
            return true;
        }
        false
    }

    pub fn remove_pending_task(&self, task_id: i32) {
        let mut pending_tasks = self.pending_tasks.lock().unwrap();
        if let Some(index) = pending_tasks.iter().position(|&x| x == task_id) {
            pending_tasks.remove(index);
        }
    }

    pub fn remove_cancelling_task(&self, task_id: i32) {
        let mut cancelling_tasks = self.cancelling_tasks.lock().unwrap();
        if let Some(index) = cancelling_tasks.iter().position(|&x| x == task_id) {
            cancelling_tasks.remove(index);
        }
    }

    pub fn remove_running_task(&self, task_id: i32) {
        let mut running_tasks = self.running_tasks.lock().unwrap();
        if let Some(index) = running_tasks.iter().position(|&x| x == task_id) {
            running_tasks.remove(index);
        }
    }

    pub fn get_pending_tasks(&self) -> Vec<i32> {
        self.pending_tasks.lock().unwrap().clone()
    }

    pub fn get_cancelling_tasks(&self) -> Vec<i32> {
        self.cancelling_tasks.lock().unwrap().clone()
    }

    pub fn get_running_tasks(&self) -> Vec<i32> {
        self.running_tasks.lock().unwrap().clone()
    }

    pub fn display_tasks(&self) {
        println!("Pending Tasks: {:?}", self.get_pending_tasks());
        println!("Cancelling Tasks: {:?}", self.get_cancelling_tasks());
        println!("Running Tasks: {:?}", self.get_running_tasks());
    }
}
