CREATE TABLE sections
(
    id                UUID        NOT NULL DEFAULT gen_random_uuid(),
    parent_section_id UUID,
    title             VARCHAR(80) NOT NULL,
    description       TEXT        NOT NULL,
    locked            BOOLEAN     NOT NULL DEFAULT FALSE,
    PRIMARY KEY (id),
    FOREIGN KEY (parent_section_id) REFERENCES sections (id) ON DELETE CASCADE
);

CREATE TABLE topics
(
    id         UUID                     NOT NULL DEFAULT gen_random_uuid(),
    created_by UUID                     NOT NULL,
    section_id UUID                     NOT NULL,
    locked     BOOLEAN                  NOT NULL DEFAULT FALSE,
    title      VARCHAR(80)              NOT NULL,
    content    TEXT                     NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE          DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    FOREIGN KEY (section_id) REFERENCES sections (id) ON DELETE CASCADE
);

CREATE TABLE posts
(
    id         UUID                     NOT NULL DEFAULT gen_random_uuid(),
    topic_id   UUID                     NOT NULL,
    created_by UUID                     NOT NULL,
    content    TEXT                     NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    FOREIGN KEY (topic_id) REFERENCES topics (id) ON DELETE CASCADE
);

-- Full-text search indexes
CREATE INDEX topics_content_idx ON topics USING GIN (to_tsvector('portuguese', content));
CREATE INDEX posts_content_idx ON posts USING GIN (to_tsvector('portuguese', content));
