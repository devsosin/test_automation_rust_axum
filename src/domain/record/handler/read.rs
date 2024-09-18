use std::sync::Arc;

use axum::{extract::Path, response::IntoResponse, Extension, Json};
use hyper::StatusCode;
use serde_json::json;

use crate::domain::record::usecase::read::ReadRecordUsecase;

pub async fn read_records<T>(
    Extension(usecase): Extension<Arc<T>>,
    Extension(user_id): Extension<i32>,
) -> impl IntoResponse
where
    T: ReadRecordUsecase,
{
    match usecase.read_records(user_id).await {
        Ok(records) => (StatusCode::OK, Json(json!(records))).into_response(),
        Err(err) => err.into_response(),
    }
}

pub async fn read_record<T>(
    Extension(usecase): Extension<Arc<T>>,
    Extension(user_id): Extension<i32>,
    Path(record_id): Path<i64>,
) -> impl IntoResponse
where
    T: ReadRecordUsecase,
{
    match usecase.read_record(user_id, record_id).await {
        Ok(record) => (StatusCode::OK, Json(json!(record))).into_response(),
        Err(err) => err.into_response(),
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use axum::{async_trait, body::Body, extract::Request, routing::get, Extension, Router};
    use chrono::NaiveDateTime;
    use http_body_util::BodyExt;
    use mockall::{mock, predicate};
    use serde_json::Value;
    use tower::ServiceExt;

    use super::{read_record, read_records};
    use crate::{
        domain::record::{entity::Record, usecase::read::ReadRecordUsecase},
        global::errors::CustomError,
    };

    mock! {
        ReadRecordUsecaseImpl {}

        #[async_trait]
        impl ReadRecordUsecase for ReadRecordUsecaseImpl {
            async fn read_records(&self, user_id: i32) -> Result<Vec<Record>, Box<CustomError>>;
            async fn read_record(&self, user_id: i32, record_id: i64) -> Result<Record, Box<CustomError>>;
        }
    }
    fn test_records() -> Vec<Record> {
        vec![
            Record::new(
                1,
                18,
                15000,
                NaiveDateTime::parse_from_str("2024-09-08 15:30:27", "%Y-%m-%d %H:%M:%S").unwrap(),
                None,
            )
            .id(Some(1))
            .build(),
            Record::new(
                1,
                18,
                15000,
                NaiveDateTime::parse_from_str("2024-09-08 15:30:27", "%Y-%m-%d %H:%M:%S").unwrap(),
                None,
            )
            .id(Some(2))
            .build(),
            Record::new(
                1,
                18,
                15000,
                NaiveDateTime::parse_from_str("2024-09-08 15:30:27", "%Y-%m-%d %H:%M:%S").unwrap(),
                None,
            )
            .id(Some(3))
            .build(),
        ]
    }

    fn _create_list_app(user_id: i32, mock_usecase: MockReadRecordUsecaseImpl) -> Router {
        Router::new()
            .route(
                "/api/v1/record",
                get(read_records::<MockReadRecordUsecaseImpl>),
            )
            .layer(Extension(Arc::new(mock_usecase)))
            .layer(Extension(user_id))
    }
    fn _create_list_req() -> Request {
        Request::builder()
            .method("GET")
            .uri("/api/v1/record")
            .body(Body::empty())
            .unwrap()
    }

    fn _create_app(user_id: i32, mock_usecase: MockReadRecordUsecaseImpl) -> Router {
        Router::new()
            .route(
                "/api/v1/record/:record_id",
                get(read_record::<MockReadRecordUsecaseImpl>),
            )
            .layer(Extension(Arc::new(mock_usecase)))
            .layer(Extension(user_id))
    }
    fn _create_req(record_id: i64) -> Request {
        Request::builder()
            .method("GET")
            .uri(format!("/api/v1/record/{}", record_id))
            .body(Body::empty())
            .unwrap()
    }

    #[tokio::test]
    async fn check_read_records_status() {
        // Arrange
        let user_id = 1;

        let mut mock_usecase = MockReadRecordUsecaseImpl::new();
        mock_usecase
            .expect_read_records()
            .with(predicate::eq(user_id))
            .returning(|_| Ok(test_records()));

        let app = _create_list_app(user_id, mock_usecase);
        let req = _create_list_req();

        // Act
        let response = app.oneshot(req).await.unwrap();

        // Assert
        assert_eq!(response.status(), 200)
    }

    #[tokio::test]
    async fn check_read_records_body() {
        // Arrange
        let user_id = 1;

        let mut mock_usecase = MockReadRecordUsecaseImpl::new();
        mock_usecase
            .expect_read_records()
            .with(predicate::eq(user_id))
            .returning(|_| Ok(test_records()));

        let app = _create_list_app(user_id, mock_usecase);
        let req = _create_list_req();

        // Act
        let response = app.oneshot(req).await.unwrap();

        let body = response.into_body();

        let body_bytes = body
            .collect()
            .await
            .expect("failed to read body")
            .to_bytes();

        let body_str =
            String::from_utf8(body_bytes.to_vec()).expect("failed to convert body to string");

        let body_json: Value = serde_json::from_str(&body_str).expect("failed to parse JSON");

        // Assert
        assert_eq!(body_json[0].get("id").unwrap(), Some(1).unwrap());
    }

    #[tokio::test]
    async fn check_read_record_status() {
        // Arrange
        let user_id = 1;
        let record_id = 1;
        let mut mock_usecase = MockReadRecordUsecaseImpl::new();
        mock_usecase
            .expect_read_record()
            .with(predicate::eq(user_id), predicate::eq(record_id))
            .returning(|_, i| {
                Ok(Record::new(
                    1,
                    18,
                    15200,
                    NaiveDateTime::parse_from_str("2024-09-07 15:30:27", "%Y-%m-%d %H:%M:%S")
                        .unwrap(),
                    None,
                )
                .id(Some(i)))
            });

        let app = _create_app(user_id, mock_usecase);
        let req = _create_req(record_id);

        // Act
        let response = app.oneshot(req).await.unwrap();

        // Assert
        assert_eq!(response.status(), 200)
    }

    #[tokio::test]
    async fn check_read_record_body() {
        // Arrange
        let user_id = 1;
        let record_id = 1;
        let mut mock_usecase = MockReadRecordUsecaseImpl::new();
        mock_usecase
            .expect_read_record()
            .with(predicate::eq(user_id), predicate::eq(record_id))
            .returning(|_, i| {
                Ok(Record::new(
                    1,
                    18,
                    15200,
                    NaiveDateTime::parse_from_str("2024-09-07 15:30:27", "%Y-%m-%d %H:%M:%S")
                        .unwrap(),
                    None,
                )
                .id(Some(i))
                .build())
            });

        let app = _create_app(user_id, mock_usecase);
        let req = _create_req(record_id);

        // Act
        let response = app.oneshot(req).await.unwrap();

        let body = response.into_body();

        let body_bytes = body
            .collect()
            .await
            .expect("failed to read body")
            .to_bytes();

        let body_str =
            String::from_utf8(body_bytes.to_vec()).expect("failed to convert body to string");

        let body_json: Value = serde_json::from_str(&body_str).expect("failed to parse JSON");

        // Assert
        assert_eq!(body_json["id"], record_id);
    }

    #[tokio::test]
    async fn check_read_record_not_found() {
        // Arrange
        let user_id = 1;
        let no_id = -32;
        let mut mock_usecase = MockReadRecordUsecaseImpl::new();
        mock_usecase
            .expect_read_record()
            .with(predicate::eq(user_id), predicate::eq(no_id))
            .returning(|_, _| Err(Box::new(CustomError::NotFound("Record".to_string()))));

        let app = _create_app(user_id, mock_usecase);
        let req = _create_req(no_id);

        // Act
        let response = app.oneshot(req).await.unwrap();

        // Assert
        assert_eq!(response.status(), 404);
    }
}
