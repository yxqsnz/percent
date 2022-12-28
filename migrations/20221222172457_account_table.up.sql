-- Add up migration script here

CREATE TABLE "accounts" (
  "nick" varchar(16) PRIMARY KEY,
  "name" varchar(32),
  -- Hashed lol
  "password" varchar NOT NULL,
  "token" text not null,
  "created_at" timestamptz NOT NULL DEFAULT (now())
);
