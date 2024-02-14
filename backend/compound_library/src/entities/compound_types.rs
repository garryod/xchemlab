//src/entities/compound_types.rs

use async_graphql::SimpleObject;
use sea_orm::{
    ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EntityTrait,
    EnumIter, PrimaryKeyTrait, Related, RelationDef, RelationTrait,
};
use uuid::Uuid;
use super::compound_instances;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, SimpleObject)]
#[sea_orm(table_name = "compound_types")]
#[graphql(name = "compound_types", complex)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(column_type = "Text", unique)]
    pub name: String,
    pub smiles: String,
}

#[derive(Clone, Copy, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "compound_instances::Entity")]
    CompoundInstances,
}

impl Related<compound_instances::Entity> for Entity{
    fn to() -> RelationDef {
        Relation::CompoundInstances.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
