#[derive(Clone)]
pub struct AppState {
    pub db: sea_orm::DbConn,
}

impl AppState {
    pub(crate) fn new(db: sea_orm::DbConn) -> Self {
        Self { db }
    }
}
