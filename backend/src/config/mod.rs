use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub server_host: String,
    pub server_port: u16,
    pub database_url: String,
    pub database_max_connections: u32,
    pub redis_url: String,
    pub jwt_access_secret: String,
    pub jwt_refresh_secret: String,
    pub jwt_access_expires_in: i64,
    pub jwt_refresh_expires_in: i64,
    pub cors_origins: Vec<String>,
    pub app_env: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Config {
            server_host: env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".into()),
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8080".into())
                .parse()?,
            database_url: env::var("DATABASE_URL")
                .expect("DATABASE_URL harus diset"),
            database_max_connections: env::var("DATABASE_MAX_CONNECTIONS")
                .unwrap_or_else(|_| "10".into())
                .parse()?,
            redis_url: env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://localhost:6379".into()),
            jwt_access_secret: env::var("JWT_ACCESS_SECRET")
                .expect("JWT_ACCESS_SECRET harus diset"),
            jwt_refresh_secret: env::var("JWT_REFRESH_SECRET")
                .expect("JWT_REFRESH_SECRET harus diset"),
            jwt_access_expires_in: env::var("JWT_ACCESS_EXPIRES_IN")
                .unwrap_or_else(|_| "900".into())
                .parse()?,
            jwt_refresh_expires_in: env::var("JWT_REFRESH_EXPIRES_IN")
                .unwrap_or_else(|_| "604800".into())
                .parse()?,
            cors_origins: env::var("CORS_ORIGINS")
                .unwrap_or_else(|_| "http://localhost:3000".into())
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),
            app_env: env::var("APP_ENV").unwrap_or_else(|_| "development".into()),
        })
    }

    pub fn is_production(&self) -> bool {
        self.app_env == "production"
    }
}
