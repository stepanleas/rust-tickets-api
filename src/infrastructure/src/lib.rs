use diesel::r2d2::ConnectionManager;
use diesel::{r2d2, PgConnection};

mod config;
mod postgres_repository;
mod entities;
mod schema;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub use config::configure;
pub use postgres_repository::PostgresTicketRepository;
