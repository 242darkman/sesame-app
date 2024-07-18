CREATE TABLE zone (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
    name VARCHAR NOT NULL,
    numLevel int NOT NULL,
    idLocation UUID REFERENCES locations(id)
);

