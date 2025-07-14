use uuid::Uuid;

pub struct FindTicketQuery {
    pub id: Option<Uuid>,
}

impl FindTicketQuery {
    pub fn new(id: Option<Uuid>) -> Self {
        Self { id }
    }
}
