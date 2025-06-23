use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use anyhow::Result;
use application::Settings;
use infrastructure::PostgresTicketRepository;
use log::{debug, info};
use presentation::AppState;
use std::sync::Arc;
use actix_web::web::Data;
use utoipa::OpenApi;
use utoipa_actix_web::AppExt;
use utoipa_swagger_ui::SwaggerUi;

pub async fn run() -> Result<Server> {
    let settings = Settings::default().load()?;
    run_internal(&settings).await
}

async fn run_internal(settings: &Settings) -> Result<Server> {
    info!("Starting HTTP server at {}", &settings.http_url);
    debug!("with configuration: {:?}", &settings);

    let pool = infrastructure::configure(settings).await?;

    let app_state = AppState {
        ticket_repository: Arc::new(PostgresTicketRepository::new(&pool)),
    };

    let server = HttpServer::new(move || {
        App::new()
            .into_utoipa_app()
            .openapi(presentation::ApiDoc::openapi())
            .map(|app| app.wrap(Logger::default()))
            .map(|app| app.configure(presentation::configure))
            .openapi_service(|api| {
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", api)
            })
            .app_data(Data::new(app_state.clone()))
            .into_app()
    })
        .bind(&settings.http_url)?
        .run();

    Ok(server)
}
