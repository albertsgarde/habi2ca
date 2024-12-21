//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::implement_id;

implement_id!(LevelId);

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "level")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: LevelId,
    pub xp_requirement: f64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl LevelId {
    pub fn next_level(self) -> Self {
        LevelId(self.0 + 1)
    }
}
