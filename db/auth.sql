CREATE TABLE users (
   id uuid PRIMARY KEY,
   username TEXT NOT NULL UNIQUE,
   created_at timestamp without time zone DEFAULT NOW() NOT NULL,
   updated_at timestamp without time zone DEFAULT NOW()
);

CREATE TABLE permissions (
  id int NOT NULL GENERATED ALWAYS AS IDENTITY,
  name text NOT NULL,
  description text NOT NULL,
  PRIMARY KEY (id)
);

CREATE TABLE roles (
  id int NOT NULL GENERATED ALWAYS AS IDENTITY,
  name text NOT NULL,
  description text NOT NULL,
  PRIMARY KEY (id)
);

CREATE TABLE roles_permissions (
  role_id int NOT NULL,
  permission_id int NOT NULL,
  PRIMARY KEY (role_id, permission_id),
  FOREIGN KEY (role_id) REFERENCES roles (id),
  FOREIGN KEY (permission_id) REFERENCES permissions (id)
);

CREATE TABLE user_roles (
  user_id uuid NOT NULL,
  role_id int NOT NULL,
  PRIMARY KEY (user_id, role_id),
  FOREIGN KEY (user_id) REFERENCES users (id),
  FOREIGN KEY (role_id) REFERENCES roles (id)
);

