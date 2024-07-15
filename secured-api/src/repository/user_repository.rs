use crate::models::user::{NewUser, UpdateUser, User};
use crate::schema::users::dsl::*;
use diesel::dsl::{count_star, now};
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use r2d2::PooledConnection;
use uuid::Uuid;

/// Gère les opérations de base de données pour les utilisateurs.
///
/// Ce répertoire fournit des méthodes asynchrones pour interagir avec la table `users`
/// dans la base de données. Il permet de créer, récupérer, mettre à jour et supprimer
/// des utilisateurs.
pub struct UserRepository;

impl UserRepository {
    /// Crée un nouvel utilisateur dans la base de données.
    ///
    /// # Arguments
    ///
    /// * `new_user` - Les données du nouvel utilisateur à créer.
    /// * `db_conn` - Une connexion à la base de données.
    ///
    /// # Retourne
    ///
    /// Le nouvel utilisateur créé ou une erreur si l'opération échoue.
    pub async fn create(
        new_user: NewUser,
        db_conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    ) -> QueryResult<User> {
        diesel::insert_into(users)
            .values(&new_user)
            .get_result(db_conn)
    }

    /// Récupère tous les utilisateurs de la base de données avec pagination.
    ///
    /// # Arguments
    ///
    /// * `db_conn` - Une connexion à la base de données.
    /// * `limit` - Le nombre maximum d'utilisateurs à retourner.
    /// * `offset` - Le nombre d'utilisateurs à sauter avant de commencer à retourner les résultats.
    ///
    /// # Retourne
    ///
    /// Une liste paginée de tous les utilisateurs et le nombre total d'utilisateurs,
    /// ou une erreur si l'opération échoue.
    pub async fn get_all(
        db_conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
        limit: i64,
        offset: i64,
    ) -> QueryResult<(Vec<User>, i64)> {
        let total_count = users.select(count_star()).first::<i64>(db_conn)?;

        let results = users.limit(limit).offset(offset).load::<User>(db_conn)?;

        Ok((results, total_count))
    }

    /// Récupère un utilisateur par son identifiant.
    ///
    /// # Arguments
    ///
    /// * `id_user` - L'identifiant de l'utilisateur à récupérer.
    /// * `db_conn` - Une connexion à la base de données.
    ///
    /// # Retourne
    ///
    /// L'utilisateur correspondant ou une erreur si l'opération échoue.
    pub async fn get_by_id(
        id_user: Uuid,
        db_conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    ) -> QueryResult<User> {
        users.find(id_user).first(db_conn)
    }

    /// Récupère un utilisateur par son nom d'utilisateur.
    ///
    /// # Arguments
    ///
    /// * `username` - Le nom d'utilisateur de l'utilisateur à récupérer.
    /// * `conn` - Une connexion à la base de données.
    ///
    /// # Retourne
    ///
    /// Un `Option<User>` contenant l'utilisateur trouvé ou `None` si aucun utilisateur n'a été trouvé,
    /// ou une erreur `diesel::result::Error` si l'opération échoue.
    pub async fn get_by_username(
        user_name: String,
        db_conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    ) -> Result<Option<User>, diesel::result::Error> {
        Ok(users
            .filter(username.eq(&user_name))
            .first::<User>(db_conn)
            .optional()?)
    }

    /// Récupère un utilisateur par son adresse email.
    ///
    /// # Arguments
    ///
    /// * `email` - L'adresse email de l'utilisateur à récupérer.
    /// * `conn` - Une connexion à la base de données.
    ///
    /// # Retourne
    ///
    /// Un `Option<User>` contenant l'utilisateur trouvé ou `None` si aucun utilisateur n'a été trouvé,
    /// ou une erreur `diesel::result::Error` si l'opération échoue.
    pub async fn get_by_email(
        mail: String,
        db_conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    ) -> Result<Option<User>, diesel::result::Error> {
        Ok(users
            .filter(email.eq(&mail))
            .first::<User>(db_conn)
            .optional()?)
    }

    /// Met à jour un utilisateur dans la base de données.
    ///
    /// # Arguments
    ///
    /// * `id_user` - L'identifiant de l'utilisateur à mettre à jour.
    /// * `updated_user` - Les nouvelles données de l'utilisateur.
    /// * `db_conn` - Une connexion à la base de données.
    ///
    /// # Retourne
    ///
    /// Le nombre de lignes affectées ou une erreur si l'opération échoue.
    pub async fn update(
        id_user: Uuid,
        updated_user: UpdateUser,
        db_conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    ) -> QueryResult<usize> {
        diesel::update(users.filter(user_id.eq(id_user)))
            .set((
                updated_user,
                updated_at.eq(now), // Utilisez `now` pour mettre à jour `updated_at`
            ))
            .execute(db_conn)
    }

    /// Supprime un utilisateur de la base de données.
    ///
    /// # Arguments
    ///
    /// * `id_user` - L'identifiant de l'utilisateur à supprimer.
    /// * `db_conn` - Une connexion à la base de données.
    ///
    /// # Retourne
    ///
    /// Le nombre de lignes affectées ou une erreur si l'opération échoue.
    pub async fn delete(
        id_user: Uuid,
        db_conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    ) -> QueryResult<usize> {
        diesel::delete(users.find(id_user)).execute(db_conn)
    }
}
