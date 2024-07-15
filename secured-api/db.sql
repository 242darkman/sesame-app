-- Création de l'utilisateur de réplication
CREATE ROLE replica_user WITH REPLICATION PASSWORD 'replica_password' LOGIN;

-- Création de la base de données
CREATE DATABASE api_database;

-- Création de l'utilisateur maître
CREATE ROLE master_user WITH LOGIN PASSWORD 'master_password';

-- Attribution des privilèges nécessaires à l'utilisateur maître
GRANT ALL PRIVILEGES ON DATABASE api_database TO master_user;
