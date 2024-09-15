use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use serde_json::json;

use crate::domain::book::{dto::request::NewBook, usecase::create::CreateBookUsecase};

pub async fn create_book<T>(
    Extension(usecase): Extension<Arc<T>>,
    Extension(user_id): Extension<i32>,
    Json(new_book): Json<NewBook>,
) -> impl IntoResponse
where
    T: CreateBookUsecase,
{
    tracing::debug!("CALL: Create Book");
    tracing::info!("Create Book : {}", new_book.get_name());

    // type_id check 1 ~ 3

    match usecase.create_book(&new_book, user_id).await {
        Ok(id) => {
            tracing::info!("Created: {}", id);
            (
                StatusCode::CREATED,
                Json(json!({"message": "신규 가계부 생성 완료", "book_id": id})),
            )
                .into_response()
        }
        Err(err) => {
            tracing::error!("Error: {:?}", err);
            err.into_response()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use axum::{
        async_trait,
        body::Body,
        http::{Method, Request},
        routing::post,
        Extension, Router,
    };

    use mockall::{mock, predicate};
    use serde_json::{to_string, Value};

    use http_body_util::BodyExt;
    use tower::ServiceExt;

    use crate::{
        domain::book::{
            dto::request::NewBook, handler::create::create_book, usecase::create::CreateBookUsecase,
        },
        global::errors::CustomError,
    };

    mock! {
        CreateBookUsecaseImpl {}

        #[async_trait]
        impl CreateBookUsecase for CreateBookUsecaseImpl {
            async fn create_book(&self, new_book: &NewBook, user_id: i32) -> Result<i32, Box<CustomError>>;
        }
    }

    fn create_req(body: String) -> Request<Body> {
        let req = Request::builder()
            .method(Method::POST)
            .uri("/api/v1/book")
            .header("content-type", "application/json;charset=utf-8")
            .body(Body::from(body))
            .unwrap();
        req
    }

    fn create_app(new_book: &NewBook, ret: Result<i32, Arc<CustomError>>) -> Router {
        let user_id = 1;

        let mut usecase = MockCreateBookUsecaseImpl::new();
        usecase
            .expect_create_book()
            .with(predicate::eq(new_book.clone()), predicate::eq(user_id))
            .returning(move |_, _| {
                if ret.is_err() {
                    Err(Box::new(
                        Arc::try_unwrap(ret.clone().err().unwrap()).unwrap(),
                    ))
                } else {
                    Ok(ret.clone().unwrap())
                }
            });

        let app = Router::new()
            .route(
                "/api/v1/book",
                post(create_book::<MockCreateBookUsecaseImpl>),
            )
            .layer(Extension(Arc::new(usecase)))
            .layer(Extension(user_id));

        app
    }

    #[tokio::test]
    async fn check_create_book_status() {
        // Arrange
        let new_book = NewBook::new("테스트 가계부".to_string(), 1);
        let json_body = to_string(&new_book).unwrap();

        let app = create_app(&new_book, Ok(1));
        let req = create_req(json_body);

        // Act
        let response = app.oneshot(req).await.unwrap();

        // Assert
        assert_eq!(response.status(), 201);
    }

    #[tokio::test]
    async fn check_create_book_body() {
        // Arrange
        let new_book = NewBook::new("테스트 가계부".to_string(), 1);
        let json_body = to_string(&new_book).unwrap();

        let app = create_app(&new_book, Ok(1));
        let req = create_req(json_body);

        // Act
        let response = app.oneshot(req).await.unwrap();

        // Assert
        let body = response.into_body();

        let body_bytes = body
            .collect()
            .await
            .expect("failed to read body")
            .to_bytes();

        let body_str =
            String::from_utf8(body_bytes.to_vec()).expect("failed to convert body to string");

        println!("{}", &body_str);

        let body_json: Value = serde_json::from_str(&body_str).expect("failed to parse JSON");

        assert_eq!(body_json["book_id"], 1);
    }

    #[tokio::test]
    async fn check_create_book_failure_no_book_type() {
        let new_book = NewBook::new("테스트 가계부".to_string(), -3);
        let json_body = to_string(&new_book).unwrap();

        let app = create_app(
            &new_book,
            Err(Arc::new(CustomError::NotFound("Category".to_string()))),
        );
        let req = create_req(json_body);

        // Act
        let response = app.oneshot(req).await.unwrap();

        // Assert
        assert_eq!(response.status(), 404)
    }

    #[tokio::test]
    async fn check_create_book_failure_duplicate() {
        let new_book = NewBook::new("중복 가계부".to_string(), 1);
        let json_body = to_string(&new_book).unwrap();

        let app = create_app(
            &new_book,
            Err(Arc::new(CustomError::Duplicated("Book".to_string()))),
        );
        let req = create_req(json_body);

        // Act
        let response = app.oneshot(req).await.unwrap();

        // Assert
        assert_eq!(response.status(), 400)
    }
}
