-- Add up migration script here
CREATE TABLE "auth"
(
    id             INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    pid            UUID UNIQUE NOT NULL DEFAULT uuid_generate_v4(),
    email         TEXT UNIQUE NOT NULL,
    password_hash TEXT        NOT NULL,
    refresh_token TEXT,
--     2fa_secret TEXT
--     last_login TIMESTAMP,
--     failed_attempts INT,
--     is_locked BOOLEAN,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at    TIMESTAMPTZ NOT NULL DEFAULT now()
);
