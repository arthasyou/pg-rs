use async_trait::async_trait;
use sea_orm::{prelude::*, *};

use super::pagination::{PaginatedResponse, PaginationParams};
use crate::error::Result;

/// Generic repository trait for common CRUD operations
#[async_trait]
pub trait Repository<E, M>
where
    E: EntityTrait<Model = M>,
    M: ModelTrait<Entity = E> + Send + Sync,
{
    /// Get database connection
    fn db(&self) -> &DatabaseConnection;

    /// Find entity by primary key
    async fn find_by_id(
        &self,
        id: <E::PrimaryKey as PrimaryKeyTrait>::ValueType,
    ) -> Result<Option<M>>
    where
        E::Model: FromQueryResult,
    {
        Ok(E::find_by_id(id).one(self.db()).await?)
    }

    /// Find all entities
    async fn find_all(&self) -> Result<Vec<M>>
    where
        E::Model: FromQueryResult,
    {
        Ok(E::find().all(self.db()).await?)
    }

    /// Find entities with pagination
    async fn find_paginated(&self, params: &PaginationParams) -> Result<PaginatedResponse<M>>
    where
        E::Model: FromQueryResult,
    {
        let params = params.clone().validate();

        let items = E::find()
            .limit(params.page_size)
            .offset(params.offset())
            .all(self.db())
            .await?;

        let total = E::find()
            .select_only()
            .column_as(Expr::value(1).count(), "count")
            .into_tuple::<i64>()
            .one(self.db())
            .await?
            .unwrap_or(0) as u64;

        Ok(PaginatedResponse::new(items, &params, total))
    }

    /// Insert a new entity
    async fn insert(&self, model: E::ActiveModel) -> Result<M>
    where
        E::ActiveModel: ActiveModelBehavior + Send,
        M: IntoActiveModel<E::ActiveModel>,
    {
        Ok(model.insert(self.db()).await?)
    }

    /// Update an existing entity
    async fn update(&self, model: E::ActiveModel) -> Result<M>
    where
        E::ActiveModel: ActiveModelBehavior + Send,
        M: IntoActiveModel<E::ActiveModel>,
    {
        Ok(model.update(self.db()).await?)
    }

    /// Delete entity by primary key
    async fn delete_by_id(
        &self,
        id: <E::PrimaryKey as PrimaryKeyTrait>::ValueType,
    ) -> Result<DeleteResult> {
        Ok(E::delete_by_id(id).exec(self.db()).await?)
    }

    /// Check if entity exists by primary key
    async fn exists_by_id(&self, id: <E::PrimaryKey as PrimaryKeyTrait>::ValueType) -> Result<bool>
    where
        E::Model: FromQueryResult,
    {
        Ok(self.find_by_id(id).await?.is_some())
    }
}

/// Base repository implementation
pub struct BaseRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> BaseRepository<'a> {
    /// Create a new base repository
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    /// Get database connection
    pub fn db(&self) -> &DatabaseConnection {
        self.db
    }
}

/// Macro to implement Repository trait for a concrete entity
#[macro_export]
macro_rules! impl_repository {
    ($struct_name:ident, $entity:ty, $model:ty) => {
        pub struct $struct_name<'a> {
            base: $crate::core::repository::BaseRepository<'a>,
        }

        impl<'a> $struct_name<'a> {
            pub fn new(db: &'a sea_orm::DatabaseConnection) -> Self {
                Self {
                    base: $crate::core::repository::BaseRepository::new(db),
                }
            }
        }

        impl<'a> $crate::core::repository::Repository<$entity, $model> for $struct_name<'a> {
            fn db(&self) -> &sea_orm::DatabaseConnection {
                self.base.db()
            }
        }
    };
}
