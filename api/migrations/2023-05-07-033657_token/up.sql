-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "tokens" (
  "id" UUID NOT NULL PRIMARY KEY DEFAULT uuid_generate_v4(),
  "value" VARCHAR NOT NULL UNIQUE,
  "kind" VARCHAR NOT NULL,
  "user_id" UUID NOT NULL REFERENCES "users" ("id") ON DELETE CASCADE,
  "created_at" TIMESTAMP NOT NULL DEFAULT NOW(),
  "expires_at" TIMESTAMP,
  "session" VARCHAR
);
