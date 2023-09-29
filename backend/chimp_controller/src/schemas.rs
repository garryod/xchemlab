use cynic::impl_scalar;
use url::Url;
use uuid::Uuid;

#[cynic::schema("targeting")]
pub mod targeting {}

impl_scalar!(Uuid, targeting::UUID);
impl_scalar!(Url, targeting::Url);
