use crate::errors::AppError;
use crate::models::{Category, CreateCategoryDto, UpdateCategoryDto};
use async_trait::async_trait;
use mockall::mock;

mock! {
    pub CategoryService {}

    #[async_trait]
    impl super::CategoryServiceTrait for CategoryService {
        async fn get_categories(&self) -> Result<Vec<Category>, AppError>;
        async fn create_category(&self, dto: CreateCategoryDto) -> Result<Category, AppError>;
        async fn update_category(&self, name: String, profit: bool, dto: UpdateCategoryDto) -> Result<Category, AppError>;
        async fn delete_category(&self, name: String, profit: bool) -> Result<(), AppError>;
    }
}
