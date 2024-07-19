use serde::{Deserialize, Serialize};

use crate::implement_id;

implement_id!(PlayerId);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlayerData {
    pub name: String,
    pub xp: f32,
}

impl PlayerData {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            xp: 0.0,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn xp(&self) -> f32 {
        self.xp
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Player {
    pub id: PlayerId,
    pub data: PlayerData,
}

impl Player {
    pub fn new(id: PlayerId, data: PlayerData) -> Self {
        Self { id, data }
    }

    pub fn id(&self) -> PlayerId {
        self.id
    }

    pub fn name(&self) -> &str {
        self.data.name()
    }

    pub fn xp(&self) -> f32 {
        self.data.xp()
    }
}
