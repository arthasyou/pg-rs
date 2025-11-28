use sea_orm::EntityTrait;

#[derive(Debug, Clone)]
pub enum SortOrder {
    Asc,
    Desc,
}

#[derive(Debug, Clone)]
pub struct OrderBy<E>
where
    E: EntityTrait,
{
    pub column: E::Column,
    pub order: SortOrder,
}

impl<E> OrderBy<E>
where
    E: EntityTrait,
{
    pub fn asc(column: E::Column) -> Self {
        Self {
            column,
            order: SortOrder::Asc,
        }
    }
    pub fn desc(column: E::Column) -> Self {
        Self {
            column,
            order: SortOrder::Desc,
        }
    }
}
