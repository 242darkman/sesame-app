use crate::models::user::{CreateUser, ModifyUser, NewUser, User, UserChangeset};
use crate::schema::users::dsl::{email, id, keycloak_uuid, users};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::PooledConnection;
use keycloak::{KeycloakAdmin, KeycloakAdminToken};
use uuid::Uuid;

pub fn find_user(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    user_id: Uuid,
) -> Result<Option<User>, diesel::result::Error> {
    match users.filter(id.eq(user_id)).first::<User>(conn) {
        Ok(user) => Ok(Some(user)),
        Err(_) => Ok(None),
    }
}

pub fn find_user_with_email(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    user_email: String,
) -> Result<Option<User>, diesel::result::Error> {
    match users.filter(email.eq(user_email)).first::<User>(conn) {
        Ok(user) => Ok(Some(user)),
        Err(_) => Ok(None),
    }
}

pub async fn find_user_with_keycloak_id(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    keycloak_id: Uuid,
) -> Result<Option<User>, diesel::result::Error> {
    match users
        .filter(keycloak_uuid.eq(keycloak_id))
        .first(conn)
        .optional()?
    {
        Some(user) => Ok(Some(user)),
        None => {
            let url =
                std::env::var("KEYCLOAK_ADDR").unwrap_or_else(|_| "http://keycloak:8080".into());
            println!("url:  {url}");
            let user = std::env::var("KEYCLOAK_USER").unwrap_or_else(|_| "admin".into());
            let password = std::env::var("KEYCLOAK_PASSWORD").unwrap_or_else(|_| "admin".into());

            // Ajoutez des logs pour vérifier les variables d'environnement
            println!("Keycloak user: {}", user);
            println!("Keycloak password: {}", password);

            let client = reqwest::Client::new();
            match KeycloakAdminToken::acquire(&url, &user, &password, &client).await {
                Ok(admin_token) => {
                    let keycloak = KeycloakAdmin::new(&url, admin_token, client);
                    let realm = std::env::var("KEYCLOAK_REALM").unwrap_or_else(|_| "sesame".into());

                    let user_info = keycloak
                        .realm_users_with_user_id_get(&realm, &keycloak_id.to_string(), None)
                        .await;

                    match user_info {
                        Ok(info) => {
                            if info.email.is_some()
                                && info.first_name.is_some()
                                && info.last_name.is_some()
                            {
                                let new_user = NewUser {
                                    email: info.email.unwrap(),
                                    firstname: info.first_name.unwrap(),
                                    lastname: info.last_name.unwrap(),
                                    keycloak_uuid: keycloak_id,
                                };
                                match diesel::insert_into(users)
                                    .values(&new_user)
                                    .get_result::<User>(conn)
                                {
                                    Ok(user) => Ok(Some(user)),
                                    Err(e) => {
                                        println!("Failed to retrieve user after insert: {:?}", e);
                                        Ok(None)
                                    }
                                }
                            } else {
                                println!("Not enough info in user representation : {:?}", info);
                                Ok(None)
                            }
                        }
                        Err(e) => {
                            println!("Failed to get user info: {:?}", e);
                            Ok(None)
                        }
                    }
                }
                Err(e) => {
                    println!("Failed to acquire admin token: {:?}", e);
                    Ok(None)
                }
            }
        }
    }
}

pub fn create_user(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    input: CreateUser,
) -> Result<User, diesel::result::Error> {
    let new_user = NewUser {
        email: input.email,
        firstname: input.firstname,
        lastname: input.lastname,
        keycloak_uuid: input.keycloak_uuid,
    };
    diesel::insert_into(users)
        .values(&new_user)
        .get_result::<User>(conn)
}

pub fn update_user(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    input: ModifyUser,
) -> Result<User, diesel::result::Error> {
    let target = users.filter(id.eq(input.id));
    diesel::update(target)
        .set(&UserChangeset {
            id: None,
            email: Some(input.email),
            firstname: Some(input.firstname),
            lastname: Some(input.lastname),
            keycloak_uuid: Some(input.keycloak_uuid),
        })
        .get_result::<User>(conn)
}
