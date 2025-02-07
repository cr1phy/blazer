mod entity;
mod errors;
mod routes;
mod state;
mod types;
mod utils;

use std::{env, io};

use crate::state::AppState;
use actix_web::{middleware, App, HttpServer};
use listenfd::ListenFd;
use migration::{Migrator, MigratorTrait};
use sea_orm::Database;

#[tokio::main]
async fn main() -> io::Result<()> {
    tracing_subscriber::fmt().init();
    dotenvy::dotenv().ok();

    let host = env::var("HOST").expect("HOST isn't in .env");
    let port = env::var("PORT").expect("PORT isn't in .env");
    let addr = format!("{}:{}", host, port);

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL isn't in .env");
    let db = Database::connect(database_url).await.unwrap();
    Migrator::up(&db, None).await.unwrap();

    let state = AppState::new(db);

    let mut server = HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .wrap(middleware::Logger::default())
            .configure(routes::init)
    });

    server = match ListenFd::from_env().take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => server.bind_auto_h2c(&addr)?,
    };

    log::info!("Starting blazer-{} at {}.", env!("CARGO_PKG_VERSION"), addr);
    server.run().await?;

    Ok(())
}
