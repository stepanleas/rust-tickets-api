use crate::validation::ValidationFieldError;
use application::{Settings, TicketDto};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub code: u16,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<ValidationFieldError>>,
}

#[readonly::make]
#[derive(Serialize, Deserialize, ToSchema)]
pub struct AppInfoResponse {
    environment: String,
}

impl AppInfoResponse {
    pub fn new(settings: Settings) -> Self {
        Self { environment: settings.environment }
    }
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