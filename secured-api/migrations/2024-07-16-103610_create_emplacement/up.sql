CREATE TABLE emplacement (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
    emplacementName VARCHAR NOT NULL
);
SELECT diesel_manage_updated_at('emplacement');
