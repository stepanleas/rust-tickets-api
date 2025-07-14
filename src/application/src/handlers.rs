use crate::DeleteTicketCommand;
use crate::commands::{CreateTicketCommand, UpdateTicketCommand};
use crate::dtos::TicketDto;
use crate::queries::FindTicketQuery;
use crate::repositories::TicketRepository;
use domain::Ticket;
use std::sync::Arc;

pub struct FindTicketQueryHandler {
    repository: Arc<dyn TicketRepository>,
}

impl FindTicketQueryHandler {
    pub fn new(repository: Arc<dyn TicketRepository>) -> Self {
        Self {
            repository: repository.clone(),
        }
    }

    pub async fn execute(&self, query: FindTicketQuery) -> anyhow::Result<TicketDto> {
        self.repository
            .find_by_id(query.id.unwrap())
            .map(TicketDto::from)
    }
}

pub struct ListAllTicketQueryHandler {
    repository: Arc<dyn TicketRepository>,
}

impl ListAllTicketQueryHandler {
    pub fn new(repository: Arc<dyn TicketRepository>) -> Self {
        Self {
            repository: repository.clone(),
        }
    }

    pub async fn execute(&self) -> anyhow::Result<Vec<TicketDto>> {
        self.repository
            .list_all()
            .map(|items| items.into_iter().map(TicketDto::from).collect())
    }
}

pub struct CreateTicketCommandHandler {
    repository: Arc<dyn TicketRepository>,
}

impl CreateTicketCommandHandler {
    pub fn new(repository: Arc<dyn TicketRepository>) -> Self {
        Self {
            repository: repository.clone(),
        }
    }

    pub async fn execute(&self, command: CreateTicketCommand) -> anyhow::Result<TicketDto> {
        self.repository
            .save(Ticket::new(command.title, command.description))
            .map(TicketDto::from)
    }
}

pub struct UpdateTicketCommandHandler {
    repository: Arc<dyn TicketRepository>,
}

impl UpdateTicketCommandHandler {
    pub fn new(repository: Arc<dyn TicketRepository>) -> Self {
        Self {
            repository: repository.clone(),
        }
    }

    pub async fn execute(&self, command: UpdateTicketCommand) -> anyhow::Result<TicketDto> {
        self.repository
            .save(Ticket {
                id: command.id,
                title: command.title,
                description: command.description,
                status: command.status,
            })
            .map(TicketDto::from)
    }
}

pub struct DeleteTicketCommandHandler {
    repository: Arc<dyn TicketRepository>,
}

impl DeleteTicketCommandHandler {
    pub fn new(repository: Arc<dyn TicketRepository>) -> Self {
        Self {
            repository: repository.clone(),
        }
    }

    pub async fn execute(&self, command: DeleteTicketCommand) -> anyhow::Result<()> {
        self.repository.delete(command.id)
    }
}
