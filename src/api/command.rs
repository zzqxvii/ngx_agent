use crate::model::r::R;
use crate::service;
use crate::util::ngx_utils::NgxCmd;
use axum::extract::{Query, RawQuery};
use axum::http::{Request, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Form, Json, Router};
use serde::{Deserialize, Serialize};

pub fn command_routers() -> Router {
    Router::new()
        .route("/start", get(start))
        .route("/quit", get(quit))
        .route("/stop", get(stop))
        .route("/reload", get(reload))
        .route("/reopen", get(reopen))
        .route("/check", get(check))
        .route("/version", get(|| async {}))
        .route("/configure", get(|| async {}))
        .route("/dumpConf", get(|| async {}))
}

#[derive(Deserialize, Serialize)]
struct Param {
    bin: String,
    conf: String,
}

async fn start(Query(query): Query<Param>) -> impl IntoResponse {
    R::ok_data(NgxCmd::START(query.bin, query.conf).exec())
}

async fn stop(Query(query): Query<Param>) -> impl IntoResponse {
    R::ok_data(NgxCmd::STOP(query.bin).exec())
}

async fn quit(Query(query): Query<Param>) -> impl IntoResponse {
    R::ok_data(NgxCmd::QUIT(query.bin).exec())
}

async fn reload(Query(query): Query<Param>) -> impl IntoResponse {
    R::ok_data(NgxCmd::RELOAD(query.bin, query.conf).exec())
}

async fn reopen(Query(query): Query<Param>) -> impl IntoResponse {
    R::ok_data(NgxCmd::REOPEN(query.bin).exec())
}

async fn check(Query(query): Query<Param>) -> impl IntoResponse {
    R::ok_data(NgxCmd::CHECK(query.bin, query.conf).exec())
}

// --

async fn version(Query(query): Query<Param>) -> impl IntoResponse {
    R::ok_data(NgxCmd::VERSION(query.bin).exec())
}

async fn configure(Query(query): Query<Param>) -> impl IntoResponse {
    R::ok_data(NgxCmd::CONFIGURE(query.bin).exec())
}
