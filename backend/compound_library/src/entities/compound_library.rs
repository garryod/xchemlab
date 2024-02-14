// src/entities/compound_library.rs

use async_graphql::{Enum, SimpleObject};
use sea_orm::prelude::*;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, DeriveActiveEnum, Enum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "compound_state")]
pub enum CompoundState {
    #[sea_orm(string_value = "Normal")]
    Normal,
    #[sea_orm(string_value = "Crystaline")]
    Crystaline,
    #[sea_orm(string_value = "Precipitated")]
    Precipitated,
    #[sea_orm(string_value = "Bad Dispense")]
    BadDispense,
    #[sea_orm(string_value = "Phase Separation")]
    PhaseSeparation,
}

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, SimpleObject)]
#[sea_orm(table_name = "compound_library")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(column_type = "Text", unique)]
    pub name: String,
    #[graphql(shareable)]
    pub compound_state: CompoundState,
}

#[derive(Clone, Copy, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
