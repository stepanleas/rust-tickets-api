use crate::app_state::AppState;
use crate::error::ApiError;
use crate::requests::{CreateTicketRequest, UpdateTicketRequest};
use crate::responses::TicketResponse;
use crate::validation::ValidatedJson;
use actix_web::web::Path;
use actix_web::{HttpRequest, HttpResponse, Responder, delete, get, post, put, web};
use anyhow::anyhow;
use application::{
    CreateTicketCommand, CreateTicketCommandHandler, DeleteTicketCommand,
    DeleteTicketCommandHandler, FindTicketQuery, FindTicketQueryHandler, ListAllTicketQueryHandler,
    UpdateTicketCommand, UpdateTicketCommandHandler,
};
use serde_json::json;
use uuid::Uuid;

const TICKETS: &str = "Tickets";

#[utoipa::path(
    context_path = "/api/tickets",
    tag = TICKETS,
    responses(
        (status = 200, description = "List current ticket items", body = [TicketResponse])
    )
)]
#[get("")]
pub async fn list_all(req: HttpRequest) -> Result<impl Responder, ApiError> {
    let state = req
        .app_data::<web::Data<AppState>>()
        .ok_or_else(|| ApiError::internal(anyhow!("Missing app state")))?;

    let handler = ListAllTicketQueryHandler::new(state.ticket_repository.clone());

    let tickets = handler.execute().await?;
    let data = tickets
        .into_iter()
        .map(TicketResponse::from)
        .collect::<Vec<TicketResponse>>();

    Ok(HttpResponse::Ok().json(json!({ "data": data })))
}

#[utoipa::path(
    context_path = "/api/tickets",
    tag = TICKETS,
    responses(
        (status = 200, description = "Get TICKET item by id", body = [TicketResponse])
    ),
    params(
        ("id" = Uuid, Path, description = "Id of the ticket item")
    ),
)]
#[get("/{id}")]
pub async fn find_one(req: HttpRequest, id: Path<Uuid>) -> Result<impl Responder, ApiError> {
    let state = req
        .app_data::<web::Data<AppState>>()
        .ok_or_else(|| ApiError::internal(anyhow!("Missing app state")))?;

    let handler = FindTicketQueryHandler::new(state.ticket_repository.clone());

    let ticket = handler
        .execute(FindTicketQuery::new(Some(id.into_inner())))
        .await?;

    Ok(HttpResponse::Ok().json(json!({ "data": TicketResponse::from(ticket) })))
}

#[utoipa::path(
    context_path = "/api/tickets",
    tag = TICKETS,
    responses(
        (status = 201, description = "Create a ticket item", body = [TicketResponse])
    ),
    request_body = CreateTicketRequest,
)]
#[post("")]
pub async fn create(
    req: HttpRequest,
    request: ValidatedJson<CreateTicketRequest>,
) -> Result<impl Responder, ApiError> {
    let payload = request.into_inner();

    let state = req
        .app_data::<web::Data<AppState>>()
        .ok_or_else(|| ApiError::internal(anyhow!("Missing app state")))?;

    let handler = CreateTicketCommandHandler::new(state.ticket_repository.clone());
    let command = CreateTicketCommand::new(payload.title.clone(), payload.description.clone());

    let ticket = handler.execute(command).await?;

    Ok(HttpResponse::Created().json(json!({ "data": TicketResponse::from(ticket) })))
}

#[utoipa::path(
    context_path = "/api/tickets",
    tag = TICKETS,
    responses(
        (status = 200, description = "Update a ticket item", body = [TicketResponse])
    ),
    params(
        ("id", description = "Id of the ticket item to update")
    ),
    request_body = UpdateTicketRequest,
)]
#[put("/{id}")]
pub async fn update(
    req: HttpRequest,
    id: Path<Uuid>,
    request: ValidatedJson<UpdateTicketRequest>,
) -> Result<impl Responder, ApiError> {
    let payload = request.into_inner();

    let state = req
        .app_data::<web::Data<AppState>>()
        .ok_or_else(|| ApiError::internal(anyhow!("Missing app state")))?;

    let handler = UpdateTicketCommandHandler::new(state.ticket_repository.clone());
    let command = UpdateTicketCommand::new(
        id.into_inner(),
        payload.title.clone(),
        payload.description.clone(),
        payload.status.clone(),
    );

    let ticket = handler.execute(command).await?;

    Ok(HttpResponse::Created().json(json!({ "data": TicketResponse::from(ticket) })))
}

#[utoipa::path(
    context_path = "/api/tickets",
    tag = TICKETS,
    responses(
        (status = 204, description = "Delete a ticket item", body = [TicketResponse])
    ),
    params(
        ("id", description = "Id of the ticket item to delete")
    )
)]
#[delete("/{id}")]
pub async fn delete(req: HttpRequest, id: Path<Uuid>) -> Result<impl Responder, ApiError> {
    let state = req
        .app_data::<web::Data<AppState>>()
        .ok_or_else(|| ApiError::internal(anyhow!("Missing app state")))?;

    let handler = DeleteTicketCommandHandler::new(state.ticket_repository.clone());
    let command = DeleteTicketCommand::new(id.into_inner());

    handler.execute(command).await?;

    Ok(HttpResponse::NoContent().finish())
}
