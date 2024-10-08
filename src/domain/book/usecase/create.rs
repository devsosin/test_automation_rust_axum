use axum::async_trait;

use crate::{
    domain::book::{dto::request::NewBook, repository::save::SaveBookRepo},
    global::errors::CustomError,
};

pub struct CreateBookUsecaseImpl<T: SaveBookRepo> {
    repository: T,
}

#[async_trait]
pub trait CreateBookUsecase: Send + Sync {
    async fn create_book(&self, new_book: &NewBook, user_id: i32) -> Result<i32, Box<CustomError>>;
}

impl<T> CreateBookUsecaseImpl<T>
where
    T: SaveBookRepo,
{
    pub fn new(repository: T) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<T> CreateBookUsecase for CreateBookUsecaseImpl<T>
where
    T: SaveBookRepo,
{
    async fn create_book(&self, new_book: &NewBook, user_id: i32) -> Result<i32, Box<CustomError>> {
        create_book(&self.repository, new_book, user_id).await
    }
}

pub async fn create_book<T: SaveBookRepo>(
    repository: &T,
    new_book: &NewBook,
    user_id: i32,
) -> Result<i32, Box<CustomError>> {
    let book = new_book.to_entity();
    repository.save_book(book, user_id).await
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use axum::async_trait;
    use mockall::{mock, predicate};

    use crate::domain::book::{
        dto::request::NewBook,
        entity::{Book, BookType},
        repository::{get_book_type::GetBookTypeRepo, save::SaveBookRepo},
    };

    use crate::global::errors::CustomError;

    use super::{CreateBookUsecase, CreateBookUsecaseImpl};

    mock! {
        pub SaveBookRepoImpl {}

        #[async_trait]
        impl SaveBookRepo for SaveBookRepoImpl {
            async fn save_book(&self, book: Book, user_id: i32) -> Result<i32, Box<CustomError>>;
        }
    }
    mock! {
        pub GetBookTypeRepoImpl {}

        #[async_trait]
        impl GetBookTypeRepo for GetBookTypeRepoImpl {
            async fn get_book_types(&self) -> Result<Vec<BookType>, Arc<CustomError>>;
            async fn get_book_type_by_name(&self, name: &str) -> Result<BookType, Arc<CustomError>>;
        }
    }

    #[tokio::test]
    async fn check_create_book_success() {
        // Arrange
        let new_book = NewBook::new("새 가계부".to_string(), 1);
        let user_id = 1;

        let mut mock_repo = MockSaveBookRepoImpl::new();
        // 모킹 동작 설정
        mock_repo
            .expect_save_book()
            .with(predicate::eq(new_book.to_entity()), predicate::eq(user_id))
            .returning(|_, _| Ok(1)); // 성공 시 id 1반환

        let usecase = CreateBookUsecaseImpl::new(mock_repo);

        // Act
        let result = usecase.create_book(&new_book, user_id).await;
        assert!(result.as_ref().map_err(|e| println!("{:?}", e)).is_ok());

        // Assert
        assert_eq!(result.unwrap(), 1);
    }

    /*
     * "비즈니스 로직 처리 상 book_type이 잘못되었을 경우"
     */
    #[tokio::test]
    async fn check_create_book_failure() {
        // Arrnge
        let user_id = 1;
        let new_book = NewBook::new("새 가계부".to_string(), 1);

        let mut mock_repo = MockSaveBookRepoImpl::new();
        mock_repo
            .expect_save_book()
            .with(predicate::eq(new_book.to_entity()), predicate::eq(user_id))
            .returning(|_, _| {
                Err(Box::new(CustomError::Unexpected(anyhow::Error::msg(
                    "에러 발생",
                ))))
            }); // repo단위 에러 반환

        let usecase = CreateBookUsecaseImpl::new(mock_repo);

        // Act
        let result = usecase.create_book(&new_book, user_id).await;

        // Assert
        assert!(result.is_err())
    }
}
