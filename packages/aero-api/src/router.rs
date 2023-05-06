use axum::routing::{get, post, delete};
use axum::Router;

use crate::controller::upload::upload;
use crate::controller::setup::setup;
use crate::controller::list_apps::list_apps;
use crate::controller::get_app::get_app;
use crate::controller::create_app::create_app;
use crate::controller::delete_app::delete_app;
use crate::controller::invoke_app::invoke_app;
use crate::repository::couch::CouchRepository;
use crate::repository::runwasm::RunwasmRepository;
use crate::usecase::appcase::Appcase;

pub fn app() -> Router {
    let appcase = Appcase::new(
        CouchRepository::new(),
        RunwasmRepository::new(),
    );

    Router::new()
        .route("/upload", post(upload))
        .route("/setup", post(setup))
        .route("/apps", get(list_apps))
        .route("/apps/:app_id", get(get_app))
        .route("/apps", post(create_app))
        .route("/apps/:app_id", delete(delete_app))
        .route("/apps/:app_id/invoke", post(invoke_app))
        .with_state(appcase)
}
