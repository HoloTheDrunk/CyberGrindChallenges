use sqlx::postgres::{PgPool, PgPoolOptions};

lazy_static::lazy_static! {
    pub static ref DATABASE_URL: String =
        dotenv::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    pub static ref POOL: PgPool = {
        PgPoolOptions::new()
            .connect_lazy(&DATABASE_URL)
            .expect("Unable to connect to Postgres")
    };
}
