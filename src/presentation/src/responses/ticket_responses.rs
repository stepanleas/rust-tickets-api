use application::TicketDto;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[readonly::make]
#[derive(Serialize, Deserialize, ToSchema)]
pub struct TicketResponse {
    id: Uuid,
    title: String,
    description: String,
    status: String,
}

impl From<TicketDto> for TicketResponse {
    fn from(ticket: TicketDto) -> Self {
        Self {
            id: ticket.id,
            title: ticket.title,
            description: ticket.description,
            status: ticket.status,
        }
    }
}