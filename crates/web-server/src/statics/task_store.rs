use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::dto::medical::UploadMarkdownResponse;

#[derive(Clone, Debug)]
pub enum TaskStatus {
    Pending,
    Running,
    Succeeded,
    Failed,
}

impl TaskStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            TaskStatus::Pending => "pending",
            TaskStatus::Running => "running",
            TaskStatus::Succeeded => "succeeded",
            TaskStatus::Failed => "failed",
        }
    }
}

#[derive(Clone, Debug)]
pub struct TaskState {
    pub status: TaskStatus,
    pub result: Option<UploadMarkdownResponse>,
    pub error: Option<String>,
}

static TASKS: OnceLock<Arc<Mutex<HashMap<String, TaskState>>>> = OnceLock::new();
static NEXT_ID: AtomicU64 = AtomicU64::new(1);

fn tasks() -> &'static Arc<Mutex<HashMap<String, TaskState>>> {
    TASKS.get_or_init(|| Arc::new(Mutex::new(HashMap::new())))
}

pub fn create_task() -> String {
    let counter = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|v| v.as_secs())
        .unwrap_or_default();
    let id = format!("task-{}-{}", ts, counter);

    let state = TaskState {
        status: TaskStatus::Pending,
        result: None,
        error: None,
    };

    if let Ok(mut map) = tasks().lock() {
        map.insert(id.clone(), state);
    }

    id
}

pub fn set_running(task_id: &str) {
    if let Ok(mut map) = tasks().lock() {
        if let Some(state) = map.get_mut(task_id) {
            state.status = TaskStatus::Running;
        }
    }
}

pub fn set_result(task_id: &str, result: UploadMarkdownResponse) {
    if let Ok(mut map) = tasks().lock() {
        if let Some(state) = map.get_mut(task_id) {
            state.status = TaskStatus::Succeeded;
            state.result = Some(result);
            state.error = None;
        }
    }
}

pub fn set_error(task_id: &str, error: String) {
    if let Ok(mut map) = tasks().lock() {
        if let Some(state) = map.get_mut(task_id) {
            state.status = TaskStatus::Failed;
            state.error = Some(error);
        }
    }
}

pub fn get_task(task_id: &str) -> Option<TaskState> {
    tasks().lock().ok().and_then(|map| map.get(task_id).cloned())
}
