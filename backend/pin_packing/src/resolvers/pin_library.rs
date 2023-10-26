use crate::{
    resolvers::Cursor,
    tables::{
        pin_library::{self, PinStatus},
        pin_mount,
    },
};
use async_graphql::{
    connection::{query, Connection, Edge, EmptyFields},
    ComplexObject, Context, Object,
};
use opa_client::subject_authorization;
use sea_orm::{
    ActiveValue, DatabaseConnection, EntityTrait, IntoActiveModel, ModelTrait, Value, Values,
};
use the_paginator::QueryCursorPage;

#[ComplexObject]
impl pin_library::Model {
    async fn mounts(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<pin_mount::Model>> {
        subject_authorization!("xchemlab.pin_packing.read_pin_mount", ctx).await?;
        let database = ctx.data::<DatabaseConnection>()?;
        Ok(self.find_related(pin_mount::Entity).all(database).await?)
    }
}

#[derive(Debug, Clone, Default)]
pub struct PinLibraryQuery;

#[Object]
impl PinLibraryQuery {
    async fn library_pins(
        &self,
        ctx: &Context<'_>,
        cursor: Cursor,
    ) -> async_graphql::Result<Connection<String, pin_library::Model, EmptyFields, EmptyFields>>
    {
        subject_authorization!("xchemlab.pin_packing.read_pin_library", ctx).await?;
        let database = ctx.data::<DatabaseConnection>()?;
        query(
            cursor.after,
            cursor.before,
            cursor.first,
            cursor.last,
            |after, before, first, last| async move {
                let page = match (first, last) {
                    (Some(limit), None) => Ok(pin_library::Entity::page_after(
                        after.map(|after| Values(vec![Value::from(after)])),
                        limit as u64,
                        database,
                    )
                    .await?),
                    (None, Some(limit)) => Ok(pin_library::Entity::page_before(
                        before.map(|before| Values(vec![Value::from(before)])),
                        limit as u64,
                        database,
                    )
                    .await?),
                    (None, None) => Err(async_graphql::Error::new(
                        "Pagination limit must be specificed",
                    )),
                    (Some(_), Some(_)) => Err(async_graphql::Error::new(
                        "Pagination direction could not be determined",
                    )),
                }?;

                let mut connection = Connection::new(page.has_previous, page.has_next);
                connection.edges.extend(
                    page.items
                        .into_iter()
                        .map(|pin| Edge::new(pin.barcode.clone(), pin)),
                );
                Ok::<_, async_graphql::Error>(connection)
            },
        )
        .await
    }
}

#[derive(Debug, Clone, Default)]
pub struct PinLibraryMutation;

#[Object]
impl PinLibraryMutation {
    async fn register_library_pin(
        &self,
        ctx: &Context<'_>,
        barcode: String,
        #[graphql(desc = "Mounting loop size in micrometers")] loop_size: i16,
    ) -> async_graphql::Result<pin_library::Model> {
        subject_authorization!("xchemlab.pin_packing.write_pin_library", ctx).await?;
        let database = ctx.data::<DatabaseConnection>()?;
        let pin = pin_library::ActiveModel {
            barcode: ActiveValue::Set(barcode),
            loop_size: ActiveValue::Set(loop_size),
            status: ActiveValue::Set(PinStatus::Ready),
        };
        Ok(pin_library::Entity::insert(pin)
            .exec_with_returning(database)
            .await?)
    }

    async fn update_library_pin_status(
        &self,
        ctx: &Context<'_>,
        barcode: String,
        status: PinStatus,
    ) -> async_graphql::Result<pin_library::Model> {
        subject_authorization!("xchemlab.pin_packing.write_pin_library", ctx).await?;
        let database = ctx.data::<DatabaseConnection>()?;
        let mut pin = pin_library::Entity::find_by_id(&barcode)
            .one(database)
            .await?
            .ok_or(format!("Could not find pin with barcode '{barcode}'"))?
            .into_active_model();
        pin.status = ActiveValue::Set(status);
        Ok(pin_library::Entity::update(pin).exec(database).await?)
    }
}
