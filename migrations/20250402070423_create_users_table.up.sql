-- Add up migration script here
CREATE TABLE "user"
(
    id             INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    pid            UUID UNIQUE NOT NULL DEFAULT uuid_generate_v4(),
    name           TEXT NOT NULL,
    surname        TEXT NOT NULL,
    email          TEXT UNIQUE NOT NULL,
    role           TEXT NOT NULL CHECK (role IN ('ADMIN', 'MODERATOR', 'USER')),
    password_hash  TEXT NOT NULL,
    created_at     TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at     TIMESTAMPTZ NOT NULL DEFAULT now()
);
