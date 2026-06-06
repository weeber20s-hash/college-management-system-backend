use dotenvy::dotenv;
use std::env;
use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db_pool: PgPool,
    pub jwt_secret: String,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub server_host: String,
    pub server_port: u16,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let server_host = env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let server_port = env::var("SERVER_PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .expect("SERVER_PORT must be a number");

        Config {
            database_url,
            jwt_secret,
            server_host,
            server_port,
        }
    }
}