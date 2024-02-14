// src/resolvers/compound_types_res.rs

use crate::entities::{compound_types, compound_instances};
use async_graphql::{Context, Object, ComplexObject};
use opa_client::subject_authorization;
use sea_orm::{ActiveValue, DatabaseConnection, EntityTrait, ModelTrait};
use uuid::Uuid;
use the_paginator::graphql::{CursorInput, ModelConnection};

#[derive(Debug, Clone, Default)]
pub struct CompoundQuery;

#[derive(Debug, Clone, Default)]
pub struct CompoundMutation;

#[ComplexObject]
impl compound_types::Model {
    async fn compound_instances(
        &self,
        ctx: &Context<'_>,
    ) -> async_graphql::Result<Vec<compound_instances::Model>> {
        subject_authorization!("xchemlab.compound_library.read_compound", ctx).await?;
        let db = ctx.data::<DatabaseConnection>()?;
        Ok(
            self.find_related(compound_instances::Entity)
                .all(db)
                .await?
        )
    }
}

#[Object]
impl CompoundQuery {
    async fn compounds(
        &self,
        ctx: &Context<'_>,
        cursor: CursorInput,
    ) -> async_graphql::Result<ModelConnection<compound_types::Model>> {
        subject_authorization!("xchemlab.compound_library.read_compound", ctx).await?;
        let db = ctx.data::<DatabaseConnection>()?;
        Ok(
            cursor
                .try_into_query_cursor::<compound_types::Entity>()?
                .all(db)
                .await?
                .try_into_connection()?
        )
    }

    async fn compound(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> async_graphql::Result<Option<compound_types::Model>> {
        subject_authorization!("xchemlab.compound_library.read_compound", ctx).await?;
        let db = ctx.data::<DatabaseConnection>()?;
        Ok(compound_types::Entity::find_by_id(id).one(db).await?)
    }
}

#[Object]
impl CompoundMutation {
    async fn add_compound(
        &self,
        ctx: &Context<'_>,
        name: String,
        smiles: String,
    ) -> async_graphql::Result<compound_types::Model> {
        subject_authorization!("xchemlab.compound_library.write_compound", ctx).await?;
        let db = ctx.data::<DatabaseConnection>()?;
        let compound = compound_types::ActiveModel {
            id: ActiveValue::Set(Uuid::now_v7()),
            name: ActiveValue::set(name),
            smiles: ActiveValue::Set(smiles),
        };
        Ok(compound_types::Entity::insert(compound)
            .exec_with_returning(db)
            .await?
        )
    }
}
