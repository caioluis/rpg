use sqlx::{PgPool, Row};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Section {
    pub id: Uuid,
    pub parent_section_id: Option<Uuid>,
    pub title: String,
    pub description: String,
    pub locked: bool,
}

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Topic {
    pub id: Uuid,
    pub created_by: Uuid,
    pub section_id: Uuid,
    pub locked: bool,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Post {
    pub id: Uuid,
    pub topic_id: Uuid,
    pub created_by: Uuid,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

#[derive(sqlx::FromRow, Serialize)]
pub struct SectionData {
    pub section: Section,
    pub topics: Vec<Topic>,
    pub children_sections: Vec<Section>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct SectionPreview {
    pub section: Section,
    pub most_recent_topic: Option<Topic>,
    pub most_recent_post: Option<Post>,
}

#[derive(sqlx::FromRow, Serialize)]
pub struct TopicData {
    pub topic: Topic,
    pub posts: Vec<Post>,
}

type DBResult<T> = Result<T, Box<dyn std::error::Error>>;

impl Section {
    pub async fn create_section(pool: PgPool, title: &str, description: &str, parent_section_id: Option<Uuid>) -> DBResult<Uuid> {
        let recap = sqlx::query!(
            r#"
            INSERT INTO sections (title, description, parent_section_id)
            VALUES ($1, $2, $3)
            RETURNING id
            "#,
            title,
            description,
            parent_section_id
        )
            .fetch_one(&pool)
            .await?;

        Ok(recap.id)
    }

    pub async fn get_section_data(pool: PgPool, section_id: Uuid) -> DBResult<SectionData> {
        let query = r#"
        WITH RECURSIVE section_tree AS (
            SELECT id, parent_section_id, title, description, locked
            FROM sections
            WHERE id = $1
            UNION ALL
            SELECT sections.id, sections.parent_section_id, sections.title, sections.description, sections.locked
            FROM sections
            INNER JOIN section_tree ON section_tree.id = sections.parent_section_id
        )
        SELECT section_tree.id, section_tree.parent_section_id, section_tree.title, section_tree.description, section_tree.locked, topics.id, topics.created_by, topics.section_id, topics.locked, topics.title, topics.content, topics.created_at, topics.updated_at
        FROM section_tree
        LEFT JOIN topics ON section_tree.id = topics.section_id;
    "#;

        let rows = sqlx::query(query)
            .bind(section_id)
            .fetch_all(&pool)
            .await?;

        if rows.is_empty() {
            return Err("Section not found.".into());
        }

        // Assuming SectionData and Topic are defined and can be deserialized from a row
        let section = Section {
            id: rows[0].get("id"),
            parent_section_id: rows[0].get("parent_section_id"),
            title: rows[0].get("title"),
            description: rows[0].get("description"),
            locked: rows[0].get("locked"),
        };

        let topics = rows.iter().filter_map(|row| {
            let topic_id: Option<Uuid> = row.get("topics.id");
            topic_id.map(|id| Topic {
                id,
                created_by: row.get("topics.created_by"),
                section_id: row.get("topics.section_id"),
                locked: row.get("topics.locked"),
                title: row.get("topics.title"),
                content: row.get("topics.content"),
                created_at: row.get("topics.created_at"),
                updated_at: row.get("topics.updated_at"),
            })
        }).collect();

        let children_sections = rows.iter().filter_map(|row| {
            let section_id: Option<Uuid> = row.get("id");
            section_id.map(|id| Section {
                id,
                parent_section_id: row.get("parent_section_id"),
                title: row.get("title"),
                description: row.get("description"),
                locked: row.get("locked"),
            })
        }).collect();

        Ok(SectionData { section, topics, children_sections })
    }

    pub async fn get_root_sections(pool: PgPool) -> DBResult<Vec<SectionPreview>> {
        let query = r#"
        WITH
        RecentTopics AS (
            SELECT DISTINCT ON (section_id)
                id, created_by, section_id, locked, title, content, created_at, updated_at
            FROM topics
            ORDER BY section_id, updated_at DESC
        ),
        RecentPosts AS (
            SELECT DISTINCT ON (topic_id)
                id, topic_id, created_by, content, created_at
            FROM posts
            ORDER BY topic_id, created_at DESC
        )
        SELECT
            s.id as section_id, s.title as section_title, s.description as section_description, s.locked as section_locked,
            rt.id as topic_id, rt.created_by as topic_created_by, rt.locked as topic_locked, rt.title as topic_title, rt.content as topic_content, rt.created_at as topic_created_at, rt.updated_at as topic_updated_at,
            rp.id as post_id, rp.created_by as post_created_by, rp.content as post_content, rp.created_at as post_created_at
        FROM sections s
        LEFT JOIN RecentTopics rt ON s.id = rt.section_id
        LEFT JOIN RecentPosts rp ON rt.id = rp.topic_id
        WHERE s.parent_section_id IS NULL;
    "#;

        let rows = sqlx::query(query)
            .fetch_all(&pool)
            .await?;

        let section_previews = rows.into_iter().map(|row| SectionPreview {
            section: Section {
                id: row.get("section_id"),
                parent_section_id: None,  // Since we're only fetching root sections
                title: row.get("section_title"),
                description: row.get("section_description"),
                locked: row.get("section_locked"),
            },
            most_recent_topic: if row.get::<Option<Uuid>, _>("topic_id").is_some() {
                Some(Topic {
                    id: row.get("topic_id"),
                    created_by: row.get("topic_created_by"),
                    section_id: row.get("section_id"),
                    locked: row.get("topic_locked"),
                    title: row.get("topic_title"),
                    content: row.get("topic_content"),
                    created_at: row.get("topic_created_at"),
                    updated_at: row.get("topic_updated_at"),
                })
            } else {
                None
            },
            most_recent_post: if row.get::<Option<Uuid>, _>("post_id").is_some() {
                Some(Post {
                    id: row.get("post_id"),
                    topic_id: row.get("topic_id"),
                    created_by: row.get("post_created_by"),
                    content: row.get("post_content"),
                    created_at: row.get("post_created_at"),
                })
            } else {
                None
            },
        }).collect();

        Ok(section_previews)
    }
}

impl Topic {
    pub async fn create_topic(pool: PgPool, title: &str, section_id: Uuid, content: &str, created_by: Uuid) -> DBResult<Uuid> {
        let query = r#"
            WITH SectionStatus AS (
                SELECT locked
                FROM sections
                WHERE id = $1
            )
            INSERT INTO topics (created_by, section_id, title, content)
            SELECT $2, $1, $3, $4
            FROM SectionStatus
            WHERE NOT locked
            RETURNING id;
        "#;

        let row = sqlx::query(query)
            .bind(section_id)
            .bind(created_by)
            .bind(title)
            .bind(content)
            .fetch_optional(&pool)
            .await?;

        match row {
            Some(row) => Ok(row.get("id")),
            None => Err("The section is locked or does not exist.".into()),
        }
    }

    pub async fn get_topic_data(pool: PgPool, topic_id: Uuid) -> DBResult<TopicData> {
        let query = r#"
            SELECT
                topics.id as topic_id, topics.created_by as topic_created_by, topics.section_id as topic_section_id,
                topics.locked as topic_locked, topics.title as topic_title, topics.content as topic_content,
                topics.created_at as topic_created_at, topics.updated_at as topic_updated_at,
                posts.id as post_id, posts.topic_id as post_topic_id, posts.created_by as post_created_by,
                posts.content as post_content, posts.created_at as post_created_at
            FROM
                topics
            LEFT JOIN
                posts ON topics.id = posts.topic_id
            WHERE
                topics.id = $1
            ORDER BY
                posts.created_at ASC;
        "#;

        let rows = sqlx::query(query)
            .bind(topic_id)
            .fetch_all(&pool)
            .await?;

        if rows.is_empty() {
            return Err("Topic not found.".into());
        }

        let mut posts = Vec::new();
        let topic = Topic {
            id: rows[0].get("topic_id"),
            created_by: rows[0].get("topic_created_by"),
            section_id: rows[0].get("topic_section_id"),
            locked: rows[0].get("topic_locked"),
            title: rows[0].get("topic_title"),
            content: rows[0].get("topic_content"),
            created_at: rows[0].get("topic_created_at"),
            updated_at: rows[0].get("topic_updated_at"),
        };

        for row in rows {
            if let Some(post_id) = row.get::<Option<Uuid>, _>("post_id") {
                posts.push(Post {
                    id: post_id,
                    topic_id: row.get("post_topic_id"),
                    created_by: row.get("post_created_by"),
                    content: row.get("post_content"),
                    created_at: row.get("post_created_at"),
                });
            }
        }

        Ok(TopicData { topic, posts })
    }
}

impl Post {
    pub async fn create_post(pool: PgPool, topic_id: Uuid, content: &str, created_by: Uuid) -> DBResult<Uuid> {
        let query = r#"
            WITH TopicStatus AS (
                SELECT locked
                FROM topics
                WHERE id = $1
            ),
            InsertPost (
                INSERT INTO posts (topic_id, created_by, content)
                SELECT $1, $2, $3
                FROM TopicStatus
                WHERE NOT locked
                RETURNING id;
            )
            UPDATE topics
            SET updated_at = now()
            WHERE id = $1
        "#;

        let row = sqlx::query(query)
            .bind(topic_id)
            .bind(created_by)
            .bind(content)
            .fetch_optional(&pool)
            .await?;

        match row {
            Some(row) => Ok(row.get("id")),
            None => Err("The topic is locked or does not exist.".into()),
        }
    }
}
