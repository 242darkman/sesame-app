CREATE TABLE defaults (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
    defaultType VARCHAR NOT NULL
);
