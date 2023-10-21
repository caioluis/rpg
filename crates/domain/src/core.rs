use sqlx::{PgPool, Row};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Section {
    pub id: Uuid,
    pub parent_section_id: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub title: String,
    pub description: String,
    pub locked: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Topic {
    pub id: Uuid,
    pub created_by: Uuid,
    pub updated_by: Option<Uuid>,
    pub section_id: Uuid,
    pub locked: bool,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Post {
    pub id: Uuid,
    pub topic_id: Uuid,
    pub created_by: Uuid,
    pub updated_by: Option<Uuid>,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
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
            parent_section_id,
        )
            .fetch_one(&pool)
            .await?;

        Ok(recap.id)
    }
// TODO: update it to handle better the last post and user info
    pub async fn get_section_data(pool: PgPool, section_id: Uuid) -> DBResult<SectionData> {
        let query = r#"
        WITH RECURSIVE section_tree AS (
            SELECT id, parent_section_id, updated_by, title, description, locked, created_at, updated_at
            FROM sections
            WHERE id = $1
            UNION ALL
            SELECT sections.id, sections.parent_section_id, sections.updated_by, sections.title, sections.description, sections.locked,  sections.created_at, sections.updated_at
            FROM sections
            INNER JOIN section_tree ON section_tree.id = sections.parent_section_id
        )
        SELECT section_tree.id, section_tree.parent_section_id, section_tree.updated_by, section_tree.title, section_tree.description, section_tree.locked, section_tree.created_at, section_tree.updated_at, topics.id as topic_id, topics.created_by as topic_created_by, topics.updated_by as topic_updated_by, topics.section_id, topics.locked as topic_locked, topics.title as topic_title, topics.created_at as topic_created_at, topics.updated_at  as topic_updated_at
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
            updated_by: rows[0].get("updated_by"),
            title: rows[0].get("title"),
            description: rows[0].get("description"),
            locked: rows[0].get("locked"),
            created_at: rows[0].get("created_at"),
            updated_at: rows[0].get("updated_at"),
        };

        let topics = rows.iter().filter_map(|row| {
            let topic_id: Option<Uuid> = row.get("topic_id");
            topic_id.map(|id| Topic {
                id,
                created_by: row.get("topic_created_by"),
                section_id: row.get("id"),
                updated_by: row.get("topic_updated_by"),
                locked: row.get("topic_locked"),
                title: row.get("topic_title"),
                created_at: row.get("topic_created_at"),
                updated_at: row.get("topic_updated_at"),
            })
        }).collect();

        let children_sections = rows.iter().filter_map(|row | {
            if row.get::<Option<Uuid>, _>("parent_section_id") == Some(section_id) {
                Some(Section {
                    id: row.get("id"),
                    parent_section_id: row.get("parent_section_id"),
                    updated_by: row.get("updated_by"),
                    title: row.get("title"),
                    description: row.get("description"),
                    locked: row.get("locked"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                })
            } else {
                None
            }
        }).collect();

        Ok(SectionData { section, topics, children_sections })
    }

    pub async fn get_root_sections(pool: PgPool) -> DBResult<Vec<SectionPreview>> {
        let query = r#"
        WITH
        TopicsOnSection AS (
            SELECT DISTINCT ON (section_id)
                id, section_id
            FROM topics
            ORDER BY section_id DESC
        ),
        RecentPosts AS (
            SELECT DISTINCT ON (topic_id)
                id, topic_id, created_by, updated_by, content, created_at, updated_at
            FROM posts
            ORDER BY topic_id, created_at DESC
        )
        SELECT
            s.id as section_id, s.title as section_title, s.updated_by as section_updated_by, s.description as section_description, s.locked as section_locked, s.created_at as section_created_at, s.updated_at as section_updated_at,
            tp.id as topic_id, tp.section_id as topic_section_id,
            rp.id as post_id, rp.topic_id as post_topic_id, rp.created_by as post_created_by, rp.updated_by as post_updated_by, rp.content as post_content, rp.created_at as post_created_at, rp.updated_at as post_updated_at
        FROM sections s
        LEFT JOIN TopicsOnSection tp ON s.id = tp.section_id
        LEFT JOIN RecentPosts rp ON tp.id = rp.topic_id
        WHERE s.parent_section_id IS NULL;
    "#;

        let rows = sqlx::query(query)
            .fetch_all(&pool)
            .await?;

        let section_previews = rows.into_iter().map(|row| SectionPreview {
            section: Section {
                id: row.get("section_id"),
                parent_section_id: None,
                updated_by: row.get("section_updated_by"),
                title: row.get("section_title"),
                description: row.get("section_description"),
                locked: row.get("section_locked"),
                created_at: row.get("section_created_at"),
                updated_at: row.get("section_updated_at"),
            },
            most_recent_post: if row.get::<Option<Uuid>, _>("post_id").is_some() {
                Some(Post {
                    id: row.get("post_id"),
                    topic_id: row.get("topic_id"),
                    created_by: row.get("post_created_by"),
                    updated_by: row.get("post_updated_by"),
                    content: row.get("post_content"),
                    created_at: row.get("post_created_at"),
                    updated_at: row.get("post_updated_at"),
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
            ),
            InsertTopic AS (
                INSERT INTO topics (created_by, section_id, title)
                SELECT $2, $1, $3
                FROM SectionStatus
                WHERE NOT locked
                RETURNING id
            )
            INSERT INTO posts (topic_id, created_by, content)
            SELECT InsertTopic.id, $2, $4
            FROM InsertTopic
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
                topics.id as topic_id, topics.created_by as topic_created_by, topics.updated_by as topic_updated_by, topics.section_id as topic_section_id, topics.locked as topic_locked, topics.title as topic_title, topics.created_at as topic_created_at, topics.updated_at as topic_updated_at,
                posts.id as post_id, posts.topic_id as post_topic_id, posts.created_by as post_created_by, posts.updated_by as post_updated_by, posts.content as post_content, posts.created_at as post_created_at, posts.updated_at as post_updated_at
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
            updated_by: rows[0].get("topic_updated_by"),
            section_id: rows[0].get("topic_section_id"),
            locked: rows[0].get("topic_locked"),
            title: rows[0].get("topic_title"),
            created_at: rows[0].get("topic_created_at"),
            updated_at: rows[0].get("topic_updated_at"),
        };

        for row in rows {
            if let Some(post_id) = row.get::<Option<Uuid>, _>("post_id") {
                posts.push(Post {
                    id: post_id,
                    topic_id: row.get("post_topic_id"),
                    created_by: row.get("post_created_by"),
                    updated_by: row.get("post_updated_by"),
                    content: row.get("post_content"),
                    created_at: row.get("post_created_at"),
                    updated_at: row.get("post_updated_at"),
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
            InsertPost AS (
                INSERT INTO posts (topic_id, created_by, content)
                SELECT $1, $2, $3
                FROM TopicStatus
                WHERE NOT locked
                RETURNING id
            )
            UPDATE topics
            SET updated_at = now()
            WHERE id = $1
            RETURNING id;
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

    pub async fn edit_post(pool: PgPool, post_id: Uuid, column: &str, value: &str, edited_by: Uuid) -> DBResult<Uuid> {
        let query_string = format!(r#"UPDATE posts SET {} = $3, updated_by = $4, updated_at = now() WHERE id = $1 RETURNING id"#, column);

        let query = sqlx::query(&query_string);

        let row = query
            .bind(post_id)
            .bind(column)
            .bind(value)
            .bind(edited_by)
            .fetch_optional(&pool)
            .await?;
        match row {
            Some(row) => Ok(row.get("id")),
            None => Err("The post does not exist.".into()),
        }
    }

    pub async fn delete_post(pool: PgPool , post_id: Uuid) -> DBResult<Uuid> {
        let query = r#"
                DELETE FROM posts
                WHERE id = $1
                RETURNING id
        "#;

        let row = sqlx::query(query)
            .bind(post_id)
            .fetch_optional(&pool)
            .await?;

        match row {
            Some(row) => Ok(row.get("id")),
            None => Err("The post does not exist.".into()),
        }
    }
}
