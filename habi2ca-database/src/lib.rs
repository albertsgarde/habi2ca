//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

pub mod prelude;

pub mod migration;
pub mod player;
pub mod task;

#[macro_export]
macro_rules! implement_id {
    ($name:ident) => {
        #[derive(
            Debug,
            Clone,
            Copy,
            PartialEq,
            Eq,
            Hash,
            serde::Serialize,
            serde::Deserialize,
            DeriveValueType,
        )]
        pub struct $name(pub i64);

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl From<u64> for $name {
            fn from(id: u64) -> Self {
                Self(id as i64)
            }
        }
        impl sea_orm::TryFromU64 for $name {
            fn try_from_u64(n: u64) -> Result<Self, DbErr> {
                Ok(Self(n as i64))
            }
        }
    };
}