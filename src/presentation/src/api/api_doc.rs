use crate::api::api_routes::__path_list_all;
use crate::api::api_routes::__path_find_one;
use crate::api::api_routes::__path_create;
use crate::api::api_routes::__path_update;
use crate::api::api_routes::__path_delete;

use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = "tickets", description = "Tickets management endpoints.")
    ),
    paths(
        list_all,
        find_one,
        create,
        update,
        delete,
    )
)]
pub struct ApiDoc;
