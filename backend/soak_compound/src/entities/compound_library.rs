// src/entities/compounds.rs

use async_graphql::{Enum, SimpleObject};
use sea_orm::prelude::*;

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
#[graphql(name = "compound_library", shareable)]
#[sea_orm(table_name = "compound_library")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[graphql(shareable)]
    pub id: i32,
    pub name: String,
    #[graphql(external)]
    pub compound_state: CompoundState,
}

#[derive(Clone, Copy, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
