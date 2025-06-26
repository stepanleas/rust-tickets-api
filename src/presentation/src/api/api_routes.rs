use crate::error::ApiError;
use crate::responses::AppInfoResponse;
use crate::AppState;
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use anyhow::anyhow;
use serde_json::json;

const APP: &str = "App";

#[utoipa::path(
    context_path = "/api/info",
    tag = APP,
    responses(
        (status = 200, description = "Display application info", body = [AppInfoResponse])
    )
)]
#[get("")]
pub async fn info(req: HttpRequest) -> Result<impl Responder, ApiError> {
    let state = req.app_data::<web::Data<AppState>>()
        .ok_or_else(|| ApiError::internal(anyhow!("Missing app state")))?;

    Ok(HttpResponse::Ok().json(json!({ "data": AppInfoResponse::new(state.settings.clone()) })))
}