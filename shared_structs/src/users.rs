use serde::{Deserialize, Serialize};

struct User {
    name: String,
    current_progress: Vec<AssignmentProgress>,
}

struct AssignmentProgress {
    assignment: u32,
    task_progress: Vec<TaskProgress>,
}

struct TaskProgress {
    task: u32,
    status: crate::tasks::Status,
}
