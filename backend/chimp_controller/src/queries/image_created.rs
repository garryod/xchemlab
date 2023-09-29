use cynic::QueryFragment;
use url::Url;
use uuid::Uuid;

#[derive(Debug, QueryFragment)]
#[cynic(schema = "targeting", schema_module = "crate::schemas::targeting")]
pub struct Image {
    pub plate: Uuid,
    pub well: i32,
    pub download_url: Url,
}

#[derive(Debug, QueryFragment)]
#[cynic(
    schema = "targeting",
    schema_module = "crate::schemas::targeting",
    graphql_type = "RootSubscription"
)]
pub struct ImageCreatedSubscription {
    pub image_created: Image,
}
