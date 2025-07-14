use application::{Settings, TicketRepository};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub settings: Settings,
    pub ticket_repository: Arc<dyn TicketRepository + Send + Sync>,
}
