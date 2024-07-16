CREATE TABLE level (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
    idEmplacement UUID REFERENCES emplacement(id)
);
