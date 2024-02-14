//src/entities/compound_instances.rs

use async_graphql::SimpleObject;
use sea_orm::{
    ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation,
    EntityTrait, EnumIter, PrimaryKeyTrait, Related, RelationDef, RelationTrait
};
use uuid::Uuid;
use super::compound_types;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, SimpleObject)]
#[sea_orm(table_name="compound_instances")]
#[graphql(name = "compund_instances", complex)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment=false)]
    pub id: Uuid,
    pub plate_id: Uuid, 
    pub well_num: i16,
    pub compound_type: Uuid,
}

#[derive(Clone, Copy, Debug, EnumIter, DeriveRelation)]
pub enum Relation{
    #[sea_orm(
        belongs_to="compound_types::Entity",
        from="Column::CompoundType",
        to="compound_types::Column::Id"
    )]
    CompoundTypes,
}

impl Related<compound_types::Entity> for Entity{
    fn to() -> RelationDef {
        Relation::CompoundTypes.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}