use sea_orm::{DatabaseConnection, EntityTrait, QueryOrder, QuerySelect, Select, prelude::Expr};

use super::PaginationParams;
use crate::query::{OrderBy, SortOrder};

#[async_trait::async_trait]
pub trait SelectExt<E>
where
    E: EntityTrait,
{
    fn pagination(self, params: &PaginationParams) -> Self;

    fn apply_order(self, order: &OrderBy<E>) -> Self;

    async fn total_count(self, db: &DatabaseConnection) -> u64;
}

#[async_trait::async_trait]
impl<E> SelectExt<E> for Select<E>
where
    E: EntityTrait,
{
    fn pagination(self, params: &PaginationParams) -> Self {
        let params = params.clone().validate();
        self.limit(params.page_size).offset(params.offset())
    }

    fn apply_order(mut self, ob: &OrderBy<E>) -> Self {
        {
            match ob.order {
                SortOrder::Asc => {
                    self = self.order_by_asc(ob.column);
                }
                SortOrder::Desc => {
                    self = self.order_by_desc(ob.column);
                }
            }
        }
        self
    }

    async fn total_count(self, db: &DatabaseConnection) -> u64 {
        match self
            .select_only()
            .column_as(Expr::value(1).count(), "count")
            .into_tuple::<i64>()
            .one(db)
            .await
        {
            Ok(Some(v)) => v as u64,
            _ => 0,
        }
    }
}
