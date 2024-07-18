CREATE TABLE comment(
    id UUID PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
    dateComment TIMESTAMP NOT NULL,
    comments VARCHAR NOT NULL,
    idUser UUID REFERENCES users(id),
    idIntervention UUID REFERENCES intervention(id)
);
