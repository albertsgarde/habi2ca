use crate::implement_id;
use serde::{Deserialize, Serialize};

implement_id!(TaskId);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskData {
    pub name: String,
    pub description: String,
    pub completed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: TaskId,
    pub data: TaskData,
}
