use diesel::{r2d2::ConnectionManager, PgConnection};
use r2d2::Pool;

#[derive(Clone)]
pub struct AppState {
    pub conn: Pool<ConnectionManager<PgConnection>>,
}
