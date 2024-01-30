// entities/wells.rs

use async_graphql::SimpleObject;
use sea_orm::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, SimpleObject)]
#[graphql(name = "wells")]
#[sea_orm(table_name = "wells")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub plate: String,
    pub pos: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
