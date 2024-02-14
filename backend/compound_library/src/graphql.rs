use crate::resolvers::compound_lib_res::{CompoundMutation, CompoundQuery};
use async_graphql::{EmptySubscription, MergedObject, Schema, SchemaBuilder};

#[derive(Debug, Clone, MergedObject, Default)]
pub struct Query(CompoundQuery);

#[derive(Debug, Clone, MergedObject, Default)]
pub struct Mutation(CompoundMutation);

pub type RootSchema = Schema<Query, Mutation, EmptySubscription>;

pub fn root_schema_builder() -> SchemaBuilder<Query, Mutation, EmptySubscription> {
    Schema::build(Query::default(), Mutation::default(), EmptySubscription).enable_federation()
}
