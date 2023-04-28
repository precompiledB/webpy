use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct User {
    pub name: String,
    pub current_progress: Vec<AssignmentProgress>,
}

#[derive(Deserialize, Serialize)]
pub struct AssignmentProgress {
    pub assignment: u32,
    pub task_progress: Vec<TaskProgress>,
}

#[derive(Deserialize, Serialize)]
pub struct TaskProgress {
    pub task: u32,
    pub status: crate::tasks::Status,
}
