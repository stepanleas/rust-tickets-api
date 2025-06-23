use domain::Ticket;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct TicketDto {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub status: String,
}

impl From<Ticket> for TicketDto {
    fn from(ticket: Ticket) -> Self {
        Self {
            id: ticket.id,
            title: ticket.title,
            description: ticket.description,
            status: ticket.status.to_string(),
        }
    }
}