use std::env;

use rust_vue_skeleton::{app::App, database::PostgresDatabase, oauth::OAuth};

use tokio::net::TcpListener;

fn get_db_name() -> String {
    format!(
        "postgres://{}:{}@localhost:{}/{}",
        std::env::var("DB_USER").expect("DB_USER not set"),
        std::env::var("DB_PASS").expect("DB_PASS not set"),
        std::env::var("DB_PORT").expect("DB_PORT not set"),
        std::env::var("DB_NAME").expect("DB_NAME not set"),
    )
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).init();

    let port: u16 = env::var("APP_PORT")
        .expect("APP_PORT not set")
        .parse()
        .expect("APP_PORT malformed");

    let postgres_url = env::var("DATABASE_URL").unwrap_or(get_db_name());
    let db = PostgresDatabase::new(&postgres_url).await;

    let client_id = env::var("DISCORD_OAUTH_CLIENT_ID").expect("DISCORD_OAUTH_CLIENT_ID not set");
    let client_secret = env::var("DISCORD_OAUTH_SECRET").expect("DISCORD_OAUTH_SECRET not set");
    let client_redirect = env::var("DISCORD_OAUTH_REDIRECT").expect("DISCORD_OAUTH_REDIRECT not set");
    let oauth = OAuth::new(client_id, client_secret, client_redirect);

    let listener = TcpListener::bind(("0.0.0.0", port))
        .await
        .expect("Failed to bind to address");

    let app = App::new(db, oauth);
    app.serve(listener).await
}
