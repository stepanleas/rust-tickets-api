use diesel::r2d2::ConnectionManager;
use diesel::{PgConnection, r2d2};

mod config;
mod entities;
mod postgres_repository;
mod schema;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub use config::configure;
pub use postgres_repository::PostgresTicketRepository;
