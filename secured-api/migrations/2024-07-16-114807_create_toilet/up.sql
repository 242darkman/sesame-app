CREATE TABLE toilet (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
    toiletStatus VARCHAR NOT NULL,
    idZone UUID REFERENCES zone(id),
    idLevel UUID REFERENCES level(id)
);