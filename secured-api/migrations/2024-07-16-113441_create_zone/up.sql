CREATE TABLE zone (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
    zoneName VARCHAR NOT NULL,
    idLevel UUID REFERENCES level(id)
);

