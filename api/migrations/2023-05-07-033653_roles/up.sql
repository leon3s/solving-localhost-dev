-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "roles" (
  "id" UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  "name" VARCHAR NOT NULL UNIQUE,
  "realm" VARCHAR NOT NULL,
  "description" VARCHAR
);

CREATE TABLE IF NOT EXISTS "user_roles" (
  "user_id" UUID NOT NULL REFERENCES "users" ("id") ON DELETE CASCADE,
  "role_id" UUID NOT NULL REFERENCES "roles" ("id") ON DELETE CASCADE,
  PRIMARY KEY ("user_id", "role_id")
);
