use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "Pucks")]
pub struct Model {
    #[sea_orm(column_name = "rowid", primary_key, auto_increment = true)]
    pub id: i32,
    #[sea_orm(column_name = "Puck")]
    pub puck: Option<String>,
    #[sea_orm(column_name = "PuckRobot")]
    pub puck_robot: Option<String>,
    #[sea_orm(column_name = "PuckInstructions")]
    pub puck_instructions: Option<String>,
    #[sea_orm(column_name = "PuckStorageShelf")]
    pub puck_storage_shelf: Option<String>,
    #[sea_orm(column_name = "PuckDone")]
    pub puck_done: Option<String>,
    #[sea_orm(column_name = "PuckData")]
    pub puck_data: Option<String>,
    #[sea_orm(column_name = "PuckShipment")]
    pub puck_shipment: Option<String>,
    #[sea_orm(column_name = "PuckWho")]
    pub puck_who: Option<String>,
    #[sea_orm(column_name = "PuckPriority")]
    pub puck_priority: Option<String>,
    #[sea_orm(column_name = "PuckDate")]
    pub puck_date: Option<String>,
    #[sea_orm(column_name = "PuckPins")]
    pub puck_pins: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
