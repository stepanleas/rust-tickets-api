use uuid::Uuid;
use crate::enums::TicketStatus;

pub struct Ticket {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub status: TicketStatus,
}

impl Ticket {
    pub fn new(title: String, description: String) -> Self {
        Ticket {
            id: Uuid::new_v4(),
            title,
            description,
            status: TicketStatus::ToDo,
        }
    }
}