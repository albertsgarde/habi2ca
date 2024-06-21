use std::fmt::Display;

use rusqlite::{
    types::{FromSql, ToSqlOutput, Value, ValueRef},
    ToSql,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PlayerId(pub i64);

impl ToSql for PlayerId {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(ToSqlOutput::Owned(Value::Integer(self.0)))
    }
}

impl FromSql for PlayerId {
    fn column_result(value: ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        if let ValueRef::Integer(id) = value {
            Ok(PlayerId(id))
        } else {
            Err(rusqlite::types::FromSqlError::InvalidType)
        }
    }
}

impl Display for PlayerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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
