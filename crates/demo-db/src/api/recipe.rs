use pg_tables::{
    pg_core::DbContext,
    table::recipe::{dto::QueryRecipe, service::RecipeService},
};

use crate::{Result, dto::recipe::{CreateRecipeRequest, RecipeResponse}};

pub struct RecipeApi {
    recipe: RecipeService,
}

impl RecipeApi {
    pub fn new(db: DbContext) -> Self {
        Self {
            recipe: RecipeService::new(db),
        }
    }

    pub async fn create(&self, req: CreateRecipeRequest) -> Result<RecipeResponse> {
        self.recipe.create(req).await
    }

    pub async fn get(&self, id: i64) -> Result<RecipeResponse> {
        self.recipe.get(id).await
    }

    pub async fn list(&self, req: QueryRecipe) -> Result<Vec<RecipeResponse>> {
        self.recipe.list(req).await
    }
}
