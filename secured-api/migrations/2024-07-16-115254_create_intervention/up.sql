CREATE TABLE intervention (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
    dateIntervention TIMESTAMP NOT NULL,
    interventionstatus VARCHAR NOT NULL,
    idToilet UUID REFERENCES toilet(id),
    idUser UUID REFERENCES users(id),
    idDefault UUID REFERENCES defaults(id),
    description VARCHAR NOT NULL
);