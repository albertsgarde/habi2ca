use crate::{implement_id, player::PlayerId};
use serde::{Deserialize, Serialize};

implement_id!(TaskId);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TaskData {
    pub player: PlayerId,
    pub name: String,
    pub description: String,
    pub completed: bool,
}

impl TaskData {
    pub fn new(player: PlayerId, name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            player,
            name: name.into(),
            description: description.into(),
            completed: false,
        }
    }

    pub fn player(&self) -> PlayerId {
        self.player
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn completed(&self) -> bool {
        self.completed
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: TaskId,
    pub data: TaskData,
}
