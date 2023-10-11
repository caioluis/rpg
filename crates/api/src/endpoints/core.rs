use sqlx::postgres::PgPool;
use axum::{extract::{self, State}, Json, Router, routing::{get, post}};

use domain::core::{Topic, Post, Section, SectionPreview};
use uuid::Uuid;

pub struct CoreRouter;

impl CoreRouter {
    pub fn new_router(pool: PgPool) -> Router {
        Router::new()
            .route("/create-section", post(create_section))
            .route("/create-topic", post(create_topic))
            .route("/create-post", post(create_post))
            .route("/get-root-sections", get(get_root_sections))
            .with_state(pool)
    }
}

pub async fn get_root_sections(State(pool): State<PgPool>) -> Json<Vec<SectionPreview>> {
    let sections = Section::get_root_sections(pool).await.unwrap();
    Json(sections)
}

pub async fn create_section(State(pool): State<PgPool>, extract::Json(payload): extract::Json<CreateSectionRequest>) -> Json<Uuid> {
    let section_id = Section::create_section(pool, payload.title.as_ref(), payload.description.as_ref(), payload.parent_section_id).await.unwrap();
    Json(section_id)
}

#[derive(serde::Deserialize)]
pub struct CreateSectionRequest {
    pub title: String,
    pub description: String,
    pub parent_section_id: Option<Uuid>,
}

pub async fn create_topic(State(pool): State<PgPool>, extract::Json(payload): extract::Json<CreateTopicRequest>) -> Json<Uuid> {
    let topic_id = Topic::create_topic(pool, payload.title.as_ref(), payload.section_id, payload.content.as_ref(), payload.created_by).await.unwrap();
    Json(topic_id)
}

#[derive(serde::Deserialize)]
pub struct CreateTopicRequest {
    pub title: String,
    pub section_id: Uuid,
    pub content: String,
    pub created_by: Uuid,
}

pub async fn create_post(State(pool): State<PgPool>, extract::Json(payload): extract::Json<CreatePostRequest>) -> Json<Uuid> {
    let post_id = Post::create_post(pool, payload.topic_id, payload.content.as_ref(), payload.created_by).await.unwrap();
    Json(post_id)
}

#[derive(serde::Deserialize)]
pub struct CreatePostRequest {
    pub topic_id: Uuid,
    pub content: String,
    pub created_by: Uuid,
}
