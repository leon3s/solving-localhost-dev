-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "users" (
  "id" UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  "name" VARCHAR NOT NULL UNIQUE,
  "email" VARCHAR NOT NULL UNIQUE,
  "password" VARCHAR NOT NULL,
  "created_at" TIMESTAMP NOT NULL DEFAULT NOW(),
  "updated_at" TIMESTAMP NOT NULL DEFAULT NOW(),
  "is_email_verified" BOOLEAN NOT NULL DEFAULT FALSE
);
