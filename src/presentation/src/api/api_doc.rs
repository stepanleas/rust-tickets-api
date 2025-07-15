use crate::api::api_health_check::__path_live;
use crate::api::api_health_check::__path_ready;
use crate::api::api_health_check::__path_startup;
use crate::api::api_tickets::__path_create;
use crate::api::api_tickets::__path_delete;
use crate::api::api_tickets::__path_find_one;
use crate::api::api_tickets::__path_list_all;
use crate::api::api_tickets::__path_update;

use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = "Tickets", description = "Tickets management endpoints.")
    ),
    paths(
        startup,
        live,
        ready,
        list_all,
        find_one,
        create,
        update,
        delete,
    )
)]
pub struct ApiDoc;
