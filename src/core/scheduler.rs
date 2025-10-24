use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

type TaskFn = Box<dyn Fn() + Send + 'static>;

pub struct Scheduler {
    tasks: Arc<Mutex<HashMap<String, TaskHandle>>>,
}

struct TaskHandle {
    #[allow(dead_code)]
    handle: glib::SourceId,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn schedule_repeating(&self, name: String, interval_ms: u64, task: TaskFn) {
        let task = Arc::new(Mutex::new(task));
        let task_clone = task.clone();

        let handle = glib::timeout_add_local(Duration::from_millis(interval_ms), move || {
            let task = task_clone.lock().unwrap();
            task();
            glib::Continue(true)
        });

        let task_handle = TaskHandle { handle };
        
        let mut tasks = self.tasks.lock().unwrap();
        tasks.insert(name, task_handle);
    }

    pub fn schedule_once(&self, name: String, delay_ms: u64, task: TaskFn) {
        let tasks = self.tasks.clone();
        let name_clone = name.clone();

        let handle = glib::timeout_add_local(Duration::from_millis(delay_ms), move || {
            task();
            
            // Remover la tarea después de ejecutarla
            let mut tasks = tasks.lock().unwrap();
            tasks.remove(&name_clone);
            
            glib::Continue(false)
        });

        let task_handle = TaskHandle { handle };
        
        let mut tasks = self.tasks.lock().unwrap();
        tasks.insert(name, task_handle);
    }

    pub fn cancel_task(&self, name: &str) {
        let mut tasks = self.tasks.lock().unwrap();
        if let Some(task_handle) = tasks.remove(name) {
            task_handle.handle.remove();
        }
    }

    pub fn cancel_all(&self) {
        let mut tasks = self.tasks.lock().unwrap();
        for (_, task_handle) in tasks.drain() {
            task_handle.handle.remove();
        }
    }
}

impl Drop for Scheduler {
    fn drop(&mut self) {
        self.cancel_all();
    }
}