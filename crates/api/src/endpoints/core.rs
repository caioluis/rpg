use sqlx::postgres::PgPool;
use axum::{extract::{self, State, Path}, Json, Router, routing::{get, post, delete, patch}};

use domain::core::{Topic, Post, Section, SectionPreview, TopicData, SectionData};
use uuid::Uuid;

pub struct CoreRouter;

impl CoreRouter {
    pub fn new_router(pool: PgPool) -> Router {
        Router::new()
            .route("/sections", get(get_root_sections))
            .route("/sections", post(create_section))
            // TODO: route naming convention is somewhat inconsistent, it would be great to further study the axum's nesting of routes
            //      as I'm doing this as a workaround avoid route conflicts.
            .route("/section/:section_id", get(get_section))
            .route("/sections/:section_id/topic", post(create_topic))
            .route("/sections/:parent_section_id", post(create_child_section))
            .route("/topic/:topic_id", get(get_topic_data))
            .route("/topic/:topic_id", post(create_post))
            .route("/posts", patch(edit_post))
            .route("/posts", delete(delete_post))
            .with_state(pool)
    }
}

// TODO: Probably is better to separate the endpoints, this is getting cluttered.

#[derive(serde::Deserialize)]
pub struct CreateSectionRequest {
    pub title: String,
    pub description: String,
}

pub async fn get_root_sections(State(pool): State<PgPool>) -> Json<Vec<SectionPreview>> {
    let sections = Section::get_root_sections(pool).await.unwrap();
    Json(sections)
}

pub async fn get_section(State(pool): State<PgPool>, Path(section_id): Path<Uuid>) -> Json<SectionData> {
    let section = Section::get_section_data(pool, section_id).await.unwrap();
    Json(section)
}

pub async fn create_section(State(pool): State<PgPool>, extract::Json(payload): extract::Json<CreateSectionRequest>) -> Json<Uuid> {
    let section_id = Section::create_section(pool, payload.title.as_ref(), payload.description.as_ref(), None).await.unwrap();
    Json(section_id)
}


pub async fn create_child_section(State(pool): State<PgPool>, Path(parent_section_id): Path<Uuid>, extract::Json(payload): extract::Json<CreateSectionRequest>) -> Json<Uuid> {
    let section_id = Section::create_section(pool, payload.title.as_ref(), payload.description.as_ref(), parent_section_id.into()).await.unwrap();
    Json(section_id)
}

#[derive(serde::Deserialize)]
pub struct GetTopicRequest {
    pub topic_id: Uuid,
}

#[derive(serde::Deserialize)]
pub struct CreateTopicRequest {
    pub title: String,
    pub content: String,
    pub created_by: Uuid,
}


pub async fn get_topic_data(State(pool): State<PgPool>, Path(topic_id): Path<Uuid>) -> Json<TopicData> {
    let topic = Topic::get_topic_data(pool, topic_id).await.unwrap();
    Json(topic)
}

pub async fn create_topic(State(pool): State<PgPool>, Path(section_id): Path<Uuid>, extract::Json(payload): extract::Json<CreateTopicRequest>) -> Json<Uuid> {
    let topic_id = Topic::create_topic(pool, payload.title.as_ref(), section_id, payload.content.as_ref(), payload.created_by).await.unwrap();
    Json(topic_id)
}

#[derive(serde::Deserialize)]
pub struct CreatePostRequest {
    pub content: String,
    pub created_by: Uuid,
}

#[derive(serde::Deserialize)]
pub struct EditPostRequest {
    pub post_id: Uuid,
    pub column: String,
    pub value: String,
    pub edited_by: Uuid,
}

#[derive(serde::Deserialize)]
pub struct DeletePostRequest {
    pub post_id: Uuid
}

pub async fn create_post(State(pool): State<PgPool>, Path(topic_id): Path<Uuid>, extract::Json(payload): extract::Json<CreatePostRequest>) -> Json<Uuid> {
    let post_id = Post::create_post(pool, topic_id, payload.content.as_ref(), payload.created_by).await.unwrap();
    Json(post_id)
}

pub async fn edit_post(State(pool): State<PgPool>, extract::Json(payload): extract::Json<EditPostRequest>) -> Json<Uuid> {
    let post_id = Post::edit_post(pool, payload.post_id, payload.column.as_ref(), payload.value.as_ref(), payload.edited_by).await.unwrap();
    Json(post_id)
}

pub async fn delete_post(State(pool): State<PgPool>, extract::Json(payload): extract::Json<DeletePostRequest>) -> Json<Uuid> {
    let post_id = Post::delete_post(pool, payload.post_id).await.unwrap();
    Json(post_id)
}
