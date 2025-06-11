use crate::errors::AppError;
use crate::models::Category;
use async_trait::async_trait;
use mockall::mock;

mock! {
    pub CategoryService {}

    #[async_trait]
    impl super::CategoryServiceTrait for CategoryService {
        async fn get_categories(&self) -> Result<Vec<Category>, AppError>;
    }
}
