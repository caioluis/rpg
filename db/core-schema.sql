CREATE TABLE sections
(
    id                UUID        NOT NULL DEFAULT gen_random_uuid(),
    parent_section_id UUID,
    updated_by        UUID,
    title             VARCHAR(80) NOT NULL,
    description       TEXT        NOT NULL,
    locked            BOOLEAN     NOT NULL DEFAULT FALSE,
    created_at        TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at        TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    FOREIGN KEY (parent_section_id) REFERENCES sections (id) ON DELETE CASCADE
);

CREATE TABLE topics
(
    id         UUID                     NOT NULL DEFAULT gen_random_uuid(),
    created_by UUID                     NOT NULL,
    section_id UUID                     NOT NULL,
    updated_by UUID,
    locked     BOOLEAN                  NOT NULL DEFAULT FALSE,
    title      VARCHAR(80)              NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE,
    PRIMARY KEY (id),
    FOREIGN KEY (section_id) REFERENCES sections (id) ON DELETE CASCADE
);

CREATE TABLE posts
(
    id         UUID                     NOT NULL DEFAULT gen_random_uuid(),
    topic_id   UUID                     NOT NULL,
    created_by UUID                     NOT NULL,
    updated_by UUID,
    content    TEXT                     NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE,
    PRIMARY KEY (id),
    FOREIGN KEY (topic_id) REFERENCES topics (id) ON DELETE CASCADE
);

-- Full-text search indexes
CREATE INDEX topics_content_idx ON topics USING GIN (to_tsvector('portuguese', title));
CREATE INDEX posts_content_idx ON posts USING GIN (to_tsvector('portuguese', content));

CREATE TABLE sections_audit
(
    id       UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    operation_type VARCHAR(6) NOT NULL,
    operation_at   TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    operation_by   UUID,
    section_id             UUID,
    parent_section_id UUID,
    title          VARCHAR(80),
    description    TEXT,
    locked         BOOLEAN
);

CREATE OR REPLACE FUNCTION audit_sections() RETURNS TRIGGER AS $$
BEGIN
    INSERT INTO sections_audit(operation_type, operation_by, section_id, parent_section_id, title, description, locked)
    VALUES (TG_OP, NEW.updated_by, OLD.id, OLD.parent_section_id, OLD.title, OLD.description, OLD.locked);
    RETURN OLD;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER sections_audit_trigger
    AFTER UPDATE OR DELETE ON sections
    FOR EACH ROW EXECUTE FUNCTION audit_sections();

CREATE TABLE topics_audit
(
    id       UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    operation_type VARCHAR(6) NOT NULL,
    operation_at   TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    operation_by   UUID,
    topic_id       UUID,
    section_id     UUID,
    locked         BOOLEAN,
    title          VARCHAR(80)
);

CREATE OR REPLACE FUNCTION audit_topics() RETURNS TRIGGER AS $$
BEGIN
    INSERT INTO topics_audit(operation_type, operation_by, topic_id, section_id, locked, title)
    VALUES (TG_OP, NEW.updated_by, OLD.id, OLD.section_id, OLD.locked, OLD.title);
    RETURN OLD;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER topics_audit_trigger
    AFTER UPDATE OR DELETE ON topics
    FOR EACH ROW EXECUTE FUNCTION audit_topics();

CREATE TABLE posts_audit
(
    id       UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    operation_type VARCHAR(6) NOT NULL,
    operation_at   TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    operation_by   UUID,
    post_id        UUID,
    topic_id       UUID,
    content        TEXT
);

CREATE OR REPLACE FUNCTION audit_posts() RETURNS TRIGGER AS $$
BEGIN
    INSERT INTO posts_audit(operation_type, operation_by, post_id, topic_id, content)
    VALUES (TG_OP, NEW.updated_by, OLD.id, OLD.topic_id, OLD.content);
    RETURN OLD;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER posts_audit_trigger
    AFTER UPDATE OR DELETE ON posts
    FOR EACH ROW EXECUTE FUNCTION audit_posts();
