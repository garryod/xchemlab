// src/resolvers/compound_instances_res.rs

use crate::entities::{compound_instances, compound_types};
use async_graphql::{Context, Object, ComplexObject};
use opa_client::subject_authorization;
use sea_orm::{ActiveValue, DatabaseConnection, EntityTrait, ModelTrait};
use uuid::Uuid;
use the_paginator::graphql::{CursorInput, ModelConnection};

#[derive(Debug, Clone, Default)]
pub struct CompoundInstanceQuery;

#[derive(Debug, Clone, Default)]
pub struct CompoundInstanceMutation;

#[ComplexObject]
impl compound_instances::Model{
    async fn compound_types(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<compound_types::Model>> {
        subject_authorization!("xchemlab.compound_library.read_compound", ctx).await?;
        let db = ctx.data::<DatabaseConnection>()?;
        Ok(
            self.find_related(compound_types::Entity)
                .all(db)
                .await?
        )
    }
}

#[Object]
impl CompoundInstanceQuery {
    async fn compound_instances(
        &self,
        ctx: &Context<'_>,
        cursor: CursorInput,
    ) -> async_graphql::Result<ModelConnection<compound_instances::Model>> {
        subject_authorization!("xchemlab.compound_library.read_compound", ctx).await?;
        let db = ctx.data::<DatabaseConnection>()?;
        Ok(
            cursor 
                .try_into_query_cursor::<compound_instances::Entity>()?
                .all(db)
                .await?
                .try_into_connection()?
        )
    }

    async fn compound_instance(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> async_graphql::Result<Option<compound_instances::Model>> {
        subject_authorization!("xchemlab.compound_library.read_compound", ctx).await?;
        let db = ctx.data::<DatabaseConnection>()?;
        Ok(compound_instances::Entity::find_by_id(id).one(db).await?)
    }
}

#[Object]
impl CompoundInstanceMutation {
    async fn add_compound_instance(
        &self,
        ctx: &Context<'_>,
        plate_id: Uuid,
        well_num: i16,
        compound_type: Uuid,
    ) -> async_graphql::Result<compound_instances::Model> {
        subject_authorization!("xchemlab.compound_library.write_compound", ctx).await?;
        let db = ctx.data::<DatabaseConnection>()?;
        let compound_instance = compound_instances::ActiveModel {
            id: ActiveValue::Set(Uuid::now_v7()),
            plate_id: ActiveValue::Set(plate_id),
            well_num: ActiveValue::Set(well_num),
            compound_type: ActiveValue::Set(compound_type),
        };
        Ok(compound_instances::Entity::insert(compound_instance)
            .exec_with_returning(db)
            .await?
        )
    }
}
