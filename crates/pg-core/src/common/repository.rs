use std::sync::Arc;

use sea_orm::{prelude::*, *};

use super::pagination::{PaginatedResponse, PaginationParams};
use crate::{common::select_ext::SelectExt, error::Result};

/// Generic repository trait for common CRUD operations
#[async_trait::async_trait]
pub trait Repository<E, M>
where
    E: EntityTrait<Model = M>,
    M: ModelTrait<Entity = E> + FromQueryResult + Send + Sync,
{
    /// Get database connection
    fn db(&self) -> &DatabaseConnection;

    // =================================================
    //  Query
    // =================================================

    fn query(&self) -> Select<E> {
        E::find()
    }

    fn query_by_id(&self, id: <E::PrimaryKey as PrimaryKeyTrait>::ValueType) -> Select<E> {
        E::find_by_id(id)
    }

    fn query_filtered(&self, filter: Condition) -> Select<E> {
        self.query().filter(filter)
    }

    // =================================================
    //  Executor
    // =================================================

    /// Execute a query and return the first result
    async fn select_one(&self, query: Select<E>) -> Result<Option<M>> {
        Ok(query.one(self.db()).await?)
    }

    /// Execute a query and return all results
    async fn select_all(&self, query: Select<E>) -> Result<Vec<M>> {
        Ok(query.all(self.db()).await?)
    }

    /// Find entity by primary key
    async fn find_by_id(
        &self,
        id: <E::PrimaryKey as PrimaryKeyTrait>::ValueType,
    ) -> Result<Option<M>> {
        let query = self.query_by_id(id);
        Ok(self.select_one(query).await?)
    }

    /// Find all entities
    async fn find_all(&self) -> Result<Vec<M>> {
        let query = self.query();
        Ok(self.select_all(query).await?)
    }

    /// Find entities with filter
    async fn find_with_filter(&self, filter: Condition) -> Result<Vec<M>> {
        let query = self.query_filtered(filter);
        Ok(self.select_all(query).await?)
    }

    /// Find entities with optional filter + pagination
    async fn find_paginated(
        &self,
        params: &PaginationParams,
        filter: Option<Condition>,
    ) -> Result<PaginatedResponse<M>> {
        // 1) base query
        let mut base = self.query();

        // 2) apply filter only if provided
        if let Some(cond) = filter {
            base = base.filter(cond);
        }

        // 3) build paginated query
        let list_query = base.clone().pagination(params);

        // 4) get items
        let items = self.select_all(list_query).await?;

        // 5) count without pagination (only filter applied)
        let total = base.total_count(self.db()).await;

        // 6) return paginated response
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

    /// Delete by ActiveModel (must contain primary key)
    async fn delete(&self, model: E::ActiveModel) -> Result<DeleteResult>
    where
        E::ActiveModel: ActiveModelTrait + Send,
    {
        let res = model.delete(self.db()).await?;
        Ok(res)
    }

    /// Delete entity by primary key
    async fn delete_by_id(
        &self,
        id: <E::PrimaryKey as PrimaryKeyTrait>::ValueType,
    ) -> Result<DeleteResult> {
        Ok(E::delete_by_id(id).exec(self.db()).await?)
    }

    /// Delete by condition
    async fn delete_many(&self, cond: Condition) -> Result<DeleteResult> {
        let res = E::delete_many().filter(cond).exec(self.db()).await?;
        Ok(res)
    }

    /// Check if entity exists by primary key
    async fn exists_by_id(
        &self,
        id: <E::PrimaryKey as PrimaryKeyTrait>::ValueType,
    ) -> Result<bool> {
        Ok(self.find_by_id(id).await?.is_some())
    }
}

/// Base repository implementation
pub struct BaseRepository {
    db: Arc<DatabaseConnection>,
}

impl BaseRepository {
    /// Create a new base repository
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    pub fn db(&self) -> &DatabaseConnection {
        &self.db
    }
}

/// Macro to implement Repository trait for a concrete entity
#[macro_export]
macro_rules! impl_repository {
    ($struct_name:ident, $entity:ty, $model:ty) => {
        pub struct $struct_name {
            base: $crate::common::repository::BaseRepository,
        }

        impl $struct_name {
            pub fn new(db: std::sync::Arc<sea_orm::DatabaseConnection>) -> Self {
                Self {
                    base: $crate::common::repository::BaseRepository::new(db),
                }
            }
        }

        impl $crate::common::repository::Repository<$entity, $model> for $struct_name {
            fn db(&self) -> &sea_orm::DatabaseConnection {
                self.base.db()
            }
        }
    };
}
