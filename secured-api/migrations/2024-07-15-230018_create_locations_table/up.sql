-- Your SQL goes here
CREATE TABLE locations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
    name VARCHAR NOT NULL
);

