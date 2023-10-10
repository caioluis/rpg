CREATE TABLE section (
    id SMALLSERIAL NOT NULL,
    parent_section_id SMALLINT,
    title VARCHAR(80) NOT NULL,
    description VARCHAR(255),
    PRIMARY KEY (id),
    FOREIGN KEY (parent_section_id) REFERENCES section(id)
);

CREATE TABLE topic (
    id SMALLSERIAL NOT NULL,
    character_id SERIAL,
    section_id SMALLINT NOT NULL,
    character_village_id SMALLINT,
    character_rank_id SMALLINT,
    character_age SMALLINT,
    title VARCHAR(80) NOT NULL,
    character_name VARCHAR(70),
    content TEXT NOT NULL,
    character_avatar TEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,

    PRIMARY KEY (id),
    FOREIGN KEY (section_id) REFERENCES section(id)
);

CREATE TABLE post (
    id SMALLSERIAL NOT NULL,
    character_id SERIAL,
    topic_id SMALLINT NOT NULL,
    character_village_id SMALLINT,
    character_rank_id SMALLINT,
    character_age SMALLINT,
    character_name VARCHAR(70),
    content TEXT NOT NULL,
    character_avatar TEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,

    PRIMARY KEY (id),
    FOREIGN KEY (topic_id) REFERENCES topic(id)
);

CREATE INDEX topic_content_idx ON topic USING GIN (to_tsvector('portuguese', content));