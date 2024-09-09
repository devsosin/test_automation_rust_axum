use std::sync::Arc;

use axum::{
    routing::{delete, get, patch, post},
    Extension, Router,
};

use sqlx::PgPool;

mod create;
mod delete;
mod read;
mod read_type;
mod update;

use create::create_book;
use delete::delete_book;
use read::{read_book, read_books};
use read_type::read_book_types;
use update::update_book;

use super::{
    repository::{
        delete::DeleteBookRepoImpl, get_book::GetBookRepoImpl, get_book_type::GetBookTypeRepoImpl,
        save::SaveBookRepoImpl, update::UpdateBookRepoImpl,
    },
    usecase::{
        create::CreateBookUsecaseImpl, delete::DeleteBookUsecaseImpl, read::ReadBookUsecaseImpl,
        read_type::ReadBookTypeUsecaseImpl, update::UpdateBookUsecaseImpl,
    },
};

pub(crate) fn create_router(pool: Arc<PgPool>) -> Router {
    let repository = SaveBookRepoImpl::new(pool.clone());

    let usecase = CreateBookUsecaseImpl::new(Arc::new(repository));

    Router::new().route(
        "/",
        post(create_book::<CreateBookUsecaseImpl<SaveBookRepoImpl>>)
            .layer(Extension(Arc::new(usecase))),
    )
}

pub(crate) fn read_router(pool: Arc<PgPool>) -> Router {
    let repository = GetBookRepoImpl::new(pool);

    let usecase = ReadBookUsecaseImpl::new(Arc::new(repository));

    Router::new()
        .route("/", get(read_books::<ReadBookUsecaseImpl<GetBookRepoImpl>>))
        .route(
            "/:book_id",
            get(read_book::<ReadBookUsecaseImpl<GetBookRepoImpl>>),
        )
        .layer(Extension(Arc::new(usecase)))
}

pub(crate) fn read_type_router(pool: Arc<PgPool>) -> Router {
    let repository = GetBookTypeRepoImpl::new(pool);

    let usecase = ReadBookTypeUsecaseImpl::new(Arc::new(repository));

    Router::new()
        .route(
            "/",
            get(read_book_types::<ReadBookTypeUsecaseImpl<GetBookTypeRepoImpl>>),
        )
        .layer(Extension(Arc::new(usecase)))
}

pub(crate) fn update_router(pool: Arc<PgPool>) -> Router {
    let repository = UpdateBookRepoImpl::new(pool);
    let usecase = UpdateBookUsecaseImpl::new(Arc::new(repository));

    Router::new()
        .route(
            "/:book_id",
            patch(update_book::<UpdateBookUsecaseImpl<UpdateBookRepoImpl>>),
        )
        .layer(Extension(Arc::new(usecase)))
}

pub(crate) fn delete_router(pool: Arc<PgPool>) -> Router {
    let repository = DeleteBookRepoImpl::new(pool);
    let usecase = DeleteBookUsecaseImpl::new(Arc::new(repository));

    Router::new()
        .route(
            "/:book_id",
            delete(delete_book::<DeleteBookUsecaseImpl<DeleteBookRepoImpl>>),
        )
        .layer(Extension(Arc::new(usecase)))
}
