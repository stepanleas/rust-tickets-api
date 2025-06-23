use application::TicketRepository;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub ticket_repository: Arc<dyn TicketRepository + Send + Sync>,
}