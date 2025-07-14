use chrono::NaiveDateTime;
use diesel::prelude::*;
use domain::Ticket;
use uuid::Uuid;

#[derive(Queryable, Insertable, AsChangeset, PartialEq, Debug)]
#[diesel(table_name = crate::schema::tickets)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub(crate) struct TicketEntity {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<Ticket> for TicketEntity {
    fn from(ticket: Ticket) -> Self {
        TicketEntity {
            id: ticket.id,
            title: ticket.title,
            description: ticket.description,
            status: ticket.status.to_string(),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}

impl Into<Ticket> for TicketEntity {
    fn into(self) -> Ticket {
        Ticket {
            id: self.id,
            title: self.title,
            description: self.description,
            status: self.status.to_string().into(),
        }
    }
}
