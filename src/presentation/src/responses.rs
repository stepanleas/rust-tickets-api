use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use application::TicketDto;
use crate::validation::ValidationFieldError;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub code: u16,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<ValidationFieldError>>,
}

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