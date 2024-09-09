use std::sync::Arc;

use axum::{response::IntoResponse, Extension, Json};
use hyper::StatusCode;
use serde_json::json;

use crate::domain::record::{dto::request::NewRecord, usecase::create::CreateRecordUsecase};

pub(crate) async fn create_record<T>(
    Extension(usecase): Extension<Arc<T>>,
    Json(new_record): Json<NewRecord>,
) -> impl IntoResponse
where
    T: CreateRecordUsecase,
{
    match usecase.create_record(&new_record).await {
        Ok(id) => (
            StatusCode::CREATED,
            Json(json!({"message": "성공", "record_id": id})),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"message": e})),
        )
            .into_response(),
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use axum::{async_trait, extract::Request, routing::post, Extension, Router};
    use chrono::NaiveDateTime;
    use http_body_util::BodyExt;
    use mockall::{mock, predicate};
    use serde_json::{to_string, Value};
    use tower::ServiceExt;

    use super::create_record;
    use crate::domain::record::{dto::request::NewRecord, usecase::create::CreateRecordUsecase};

    mock! {
        CreateRecordUsecaseImpl {}

        #[async_trait]
        impl CreateRecordUsecase for CreateRecordUsecaseImpl {
            async fn create_record(&self, new_record: &NewRecord) -> Result<i64, String>;
        }
    }

    #[tokio::test]
    async fn check_create_record_status() {
        // Arrange
        let new_record = NewRecord::new(
            1,
            18,
            15200,
            Some("감자탕".to_string()),
            NaiveDateTime::parse_from_str("2024-09-08 18:39:27", "%Y-%m-%d %H:%M:%S").unwrap(),
            None,
            None,
        );

        let mut mock_usecase = MockCreateRecordUsecaseImpl::new();
        mock_usecase
            .expect_create_record()
            .with(predicate::eq(new_record.clone()))
            .returning(|_| Ok(1));

        let app = Router::new()
            .route(
                "/api/v1/record",
                post(create_record::<MockCreateRecordUsecaseImpl>),
            )
            .layer(Extension(Arc::new(mock_usecase)));
        let req = Request::builder()
            .method("POST")
            .uri("/api/v1/record")
            .header("content-type", "application/json")
            .body(to_string(&new_record).unwrap())
            .unwrap();

        // Act
        let response = app.oneshot(req).await.unwrap();

        // Assert
        assert_eq!(response.status(), 201);
    }

    #[tokio::test]
    async fn check_create_record_body() {
        // Arrange
        let new_record = NewRecord::new(
            1,
            18,
            15200,
            Some("순대국".to_string()),
            NaiveDateTime::parse_from_str("2024-09-08 15:37:48", "%Y-%m-%d %H:%M:%S").unwrap(),
            None,
            Some(vec![1]),
        );

        let mut mock_usecase = MockCreateRecordUsecaseImpl::new();
        mock_usecase
            .expect_create_record()
            .with(predicate::eq(new_record.clone()))
            .returning(|_| Ok(1));

        let app = Router::new()
            .route(
                "/api/v1/record",
                post(create_record::<MockCreateRecordUsecaseImpl>),
            )
            .layer(Extension(Arc::new(mock_usecase)));

        let req = Request::builder()
            .method("POST")
            .uri("/api/v1/record")
            .header("content-type", "application/json")
            .body(to_string(&new_record).unwrap())
            .unwrap();

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

        let body_json: Value = serde_json::from_str(&body_str).expect("failed to parse JSON");

        assert_eq!(body_json["record_id"], 1);
    }

    #[tokio::test]
    async fn check_create_record_not_found() {
        // Arrange
        let new_record = NewRecord::new(
            1,
            -32,
            15200,
            None,
            NaiveDateTime::parse_from_str("2024-09-07 15:30:28", "%Y-%m-%d %H:%M:%S").unwrap(),
            None,
            None,
        );

        let mut mock_usecase = MockCreateRecordUsecaseImpl::new();
        mock_usecase
            .expect_create_record()
            .with(predicate::eq(new_record.clone()))
            .returning(|_| Err("sub category id not found".to_string()));

        let app = Router::new()
            .route(
                "/api/v1/record",
                post(create_record::<MockCreateRecordUsecaseImpl>),
            )
            .layer(Extension(Arc::new(mock_usecase)));

        let req = Request::builder()
            .method("POST")
            .uri("/api/v1/record")
            .header("content-type", "application/json")
            .body(to_string(&new_record).unwrap())
            .unwrap();

        // Act
        let response = app.oneshot(req).await.unwrap();

        // Assert
        assert_eq!(response.status(), 404)
    }
}
