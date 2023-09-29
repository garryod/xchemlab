use chimp_protocol::{BBox, Point};
use cynic::{InputObject, QueryFragment, QueryVariables};
use uuid::Uuid;

#[derive(Debug, Clone, QueryFragment)]
#[cynic(schema = "targeting", schema_module = "crate::schemas::targeting")]
pub struct Prediction {
    pub id: Uuid,
}

#[derive(Debug, Clone, InputObject)]
#[cynic(schema = "targeting", schema_module = "crate::schemas::targeting")]
pub struct PointInput {
    pub x: i32,
    pub y: i32,
}

impl TryFrom<Point> for PointInput {
    type Error = <i32 as TryFrom<usize>>::Error;

    fn try_from(value: Point) -> Result<Self, Self::Error> {
        Ok(Self {
            x: value.x,
            y: value.y,
        })
    }
}

#[derive(Debug, Clone, InputObject)]
#[cynic(schema = "targeting", schema_module = "crate::schemas::targeting")]
pub struct WellInput {
    pub plate: Uuid,
    pub well: i32,
}

#[derive(Debug, Clone, InputObject)]
#[cynic(schema = "targeting", schema_module = "crate::schemas::targeting")]
pub struct CrystalInput {
    pub bounding_box: BoundingBoxInput,
}

#[derive(Debug, Clone, InputObject)]
#[cynic(schema = "targeting", schema_module = "crate::schemas::targeting")]
pub struct BoundingBoxInput {
    pub left: i32,
    pub right: i32,
    pub top: i32,
    pub bottom: i32,
}

impl From<BBox> for BoundingBoxInput {
    fn from(value: BBox) -> Self {
        Self {
            left: value.left,
            right: value.right,
            top: value.top,
            bottom: value.bottom,
        }
    }
}

#[derive(Debug, Clone, InputObject)]
#[cynic(schema = "targeting", schema_module = "crate::schemas::targeting")]
pub struct DropInput {
    pub crystals: Vec<CrystalInput>,
    pub insertion_point: PointInput,
    pub bounding_box: BoundingBoxInput,
}

#[derive(QueryVariables)]
#[cynic(schema_module = "crate::schemas::targeting")]
pub struct CreatePredictionVariables {
    pub plate: WellInput,
    pub well_centroid: PointInput,
    pub well_radius: i32,
    pub drops: Vec<DropInput>,
}

#[derive(Debug, Clone, QueryFragment)]
#[cynic(
    schema = "targeting",
    schema_module = "crate::schemas::targeting",
    graphql_type = "RootMutation",
    variables = "CreatePredictionVariables"
)]
pub struct CreatePredictionMutation {
    #[arguments(plate: $plate, wellCentroid: $well_centroid, wellRadius: $well_radius, drops: $drops)]
    pub create_prediction: Prediction,
}
