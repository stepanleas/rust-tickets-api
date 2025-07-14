use serde::{Deserialize, Serialize};
use std::fmt::Display;
use strum_macros::EnumIter;
use utoipa::ToSchema;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumIter, ToSchema)]
pub enum TicketStatus {
    ToDo,
    InProgress,
    Done,
    Closed,
}

impl From<String> for TicketStatus {
    fn from(status: String) -> Self {
        match status.to_lowercase().as_str() {
            "to_do" => TicketStatus::ToDo,
            "in_progress" => TicketStatus::InProgress,
            "done" => TicketStatus::Done,
            "closed" => TicketStatus::Closed,
            _ => TicketStatus::ToDo,
        }
    }
}

impl Display for TicketStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TicketStatus::ToDo => write!(f, "to_do"),
            TicketStatus::InProgress => write!(f, "in_progress"),
            TicketStatus::Done => write!(f, "done"),
            TicketStatus::Closed => write!(f, "closed"),
        }
    }
}
