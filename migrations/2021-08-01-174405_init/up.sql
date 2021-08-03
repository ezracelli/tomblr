CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE "users" (
    "_rowid" SERIAL,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    "deleted_at" TIMESTAMPTZ,
    "id" UUID NOT NULL DEFAULT uuid_generate_v4(),
    "role" TEXT NOT NULL,
    "updated_at" TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,

    PRIMARY KEY ("id")
);

SELECT diesel_manage_updated_at('users');

CREATE TABLE "email_accounts" (
    "_rowid" SERIAL,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    "deleted_at" TIMESTAMPTZ,
    "id" UUID NOT NULL DEFAULT uuid_generate_v4(),
    "provider_account_id" TEXT NOT NULL,
    "updated_at" TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    "user_id" UUID NOT NULL,

    PRIMARY KEY ("id"),
    FOREIGN KEY ("user_id") REFERENCES "users" ("id"),
    UNIQUE ("provider_account_id", "user_id")
);

SELECT diesel_manage_updated_at('email_accounts');

CREATE TABLE "oauth_accounts" (
    "_rowid" SERIAL,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    "deleted_at" TIMESTAMPTZ,
    "id" UUID NOT NULL DEFAULT uuid_generate_v4(),
    "provider" TEXT NOT NULL,
    "provider_access_token" TEXT NOT NULL,
    "provider_access_token_expires_at" TIMESTAMPTZ NOT NULL,
    "provider_account_id" TEXT NOT NULL,
    "provider_refresh_token" TEXT NOT NULL,
    "updated_at" TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    "user_id" UUID NOT NULL,

    PRIMARY KEY ("id"),
    FOREIGN KEY ("user_id") REFERENCES "users" ("id"),
    UNIQUE ("provider", "user_id")
);

SELECT diesel_manage_updated_at('oauth_accounts');

CREATE TABLE "blogs" (
    "_rowid" SERIAL,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    "deleted_at" TIMESTAMPTZ,
    "id" UUID NOT NULL DEFAULT uuid_generate_v4(),
    "slug" TEXT NOT NULL,
    "title" TEXT NOT NULL,
    "updated_at" TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    "user_id" UUID NOT NULL,

    PRIMARY KEY ("id"),
    FOREIGN KEY ("user_id") REFERENCES "users" ("id"),
    UNIQUE ("slug")
);

SELECT diesel_manage_updated_at('blogs');

CREATE TABLE "posts" (
    "_rowid" SERIAL,
    "blog_id" UUID NOT NULL,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    "deleted_at" TIMESTAMPTZ,
    "id" UUID NOT NULL DEFAULT uuid_generate_v4(),
    "slug" TEXT NOT NULL,
    "updated_at" TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,

    PRIMARY KEY ("id"),
    FOREIGN KEY ("blog_id") REFERENCES "blogs" ("id"),
    UNIQUE ("blog_id", "slug")
);

SELECT diesel_manage_updated_at('posts');
