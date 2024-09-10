use std::sync::Arc;

use axum::async_trait;

use crate::domain::record::repository::delete::DeleteRecordRepo;

pub(crate) struct DeleteRecordUsecaseImpl<T>
where
    T: DeleteRecordRepo,
{
    repository: Arc<T>,
}

#[async_trait]
pub(crate) trait DeleteRecordUsecase: Send + Sync {
    async fn delete_record(&self, id: i64) -> Result<(), String>;
}

impl<T> DeleteRecordUsecaseImpl<T>
where
    T: DeleteRecordRepo,
{
    pub(crate) fn new(repository: Arc<T>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<T> DeleteRecordUsecase for DeleteRecordUsecaseImpl<T>
where
    T: DeleteRecordRepo,
{
    async fn delete_record(&self, id: i64) -> Result<(), String> {
        delete_record(&*self.repository, id).await
    }
}

async fn delete_record<T>(repository: &T, id: i64) -> Result<(), String>
where
    T: DeleteRecordRepo,
{
    repository.delete_record(id).await
}

#[cfg(test)]
mod tests {
    use axum::async_trait;
    use mockall::{mock, predicate};

    use crate::domain::record::repository::delete::DeleteRecordRepo;

    use super::delete_record;

    mock! {
        DeleteRecordRepoImpl {}

        #[async_trait]
        impl DeleteRecordRepo for DeleteRecordRepoImpl{
            async fn delete_record(&self, id: i64) -> Result<(), String>;
        }
    }

    #[tokio::test]
    async fn check_delete_record_success() {
        // Arrange
        let id = 1;

        let mut mock_repo = MockDeleteRecordRepoImpl::new();
        mock_repo
            .expect_delete_record()
            .with(predicate::eq(id))
            .returning(|_| Ok(()));

        // Act
        let result = delete_record(&mock_repo, id).await;

        // Assert
        assert!(result.is_ok())
    }

    #[tokio::test]
    async fn check_id_not_found() {
        // Arrange
        let no_id = -32;

        let mut mock_repo = MockDeleteRecordRepoImpl::new();
        mock_repo
            .expect_delete_record()
            .with(predicate::eq(no_id))
            .returning(|i| Err(format!("{} id not found", i)));

        // Act
        let result = delete_record(&mock_repo, no_id).await;

        // Assert
        assert!(result.is_err())
    }
}
