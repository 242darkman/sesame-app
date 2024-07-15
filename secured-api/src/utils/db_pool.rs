use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use std::env;

pub type PostgresPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> PostgresPool {
    dotenv::dotenv().ok();
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("could not build connection pool")
}
