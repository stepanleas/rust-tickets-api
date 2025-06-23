use uuid::Uuid;
use domain::TicketStatus;

pub struct CreateTicketCommand {
    pub title: String,
    pub description: String,
}

impl CreateTicketCommand {
    pub fn new(title: String, description: String) -> Self {
        CreateTicketCommand { title, description }
    }
}

pub struct UpdateTicketCommand {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub status: TicketStatus,
}

impl UpdateTicketCommand {
    pub fn new(id: Uuid, title: String, description: String, status: TicketStatus) -> Self {
        UpdateTicketCommand {
            id,
            title,
            description,
            status,
        }
    }
}

pub struct DeleteTicketCommand {
    pub id: Uuid,
}

impl DeleteTicketCommand {
    pub fn new(id: Uuid) -> Self {
        DeleteTicketCommand { id }
    }
}