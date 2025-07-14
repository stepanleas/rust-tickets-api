use crate::DbPool;
use crate::entities::TicketEntity;
use crate::schema::tickets::dsl::tickets;
use crate::schema::tickets::id;
use application::TicketRepository;
use diesel::ExpressionMethods;
use diesel::{OptionalExtension, QueryDsl, RunQueryDsl};
use domain::{DomainError, Ticket};
use uuid::Uuid;

pub struct PostgresTicketRepository {
    pool: DbPool,
}

impl PostgresTicketRepository {
    pub fn new(pool: &DbPool) -> Self {
        Self { pool: pool.clone() }
    }
}

impl TicketRepository for PostgresTicketRepository {
    fn list_all(&self) -> anyhow::Result<Vec<Ticket>> {
        let mut connection = self.pool.get()?;
        let items: Vec<TicketEntity> = tickets.load(&mut connection)?;

        Ok(items.into_iter().map(TicketEntity::into).collect())
    }

    fn find_by_id(&self, entity_id: Uuid) -> anyhow::Result<Ticket> {
        let mut connection = self.pool.get()?;

        let ticket_entity = tickets
            .filter(id.eq(entity_id))
            .first::<TicketEntity>(&mut connection)
            .optional()?
            .ok_or(DomainError::NotFound { id: entity_id })?;

        Ok(ticket_entity.into())
    }

    fn save(&self, entity: Ticket) -> anyhow::Result<Ticket> {
        let mut connection = self.pool.get()?;

        let persistent_entity = TicketEntity::from(entity);
        diesel::insert_into(tickets)
            .values(&persistent_entity)
            .on_conflict(id)
            .do_update()
            .set(&persistent_entity)
            .execute(&mut connection)?;

        Ok(persistent_entity.into())
    }

    fn delete(&self, entity_id: Uuid) -> anyhow::Result<()> {
        let mut connection = self.pool.get()?;
        diesel::delete(tickets.filter(id.eq(entity_id))).execute(&mut connection)?;

        Ok(())
    }
}
