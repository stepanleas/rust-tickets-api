use domain::Ticket;
use uuid::Uuid;

pub trait TicketRepository {
    fn list_all(&self) -> anyhow::Result<Vec<Ticket>>;
    fn find_by_id(&self, entity_id: Uuid) -> anyhow::Result<Ticket>;
    fn save(&self, entity: Ticket) -> anyhow::Result<Ticket>;
    fn delete(&self, entity_id: Uuid) -> anyhow::Result<()>;
}