pub mod command;
pub mod conf;
pub mod log;
pub mod ngx;

use axum::routing::get;
use axum::Router;

pub fn load_routers() -> Router {
    let root = Router::new().route("/", get(|| async { "hello, ngx_agent" }));

    Router::new()
        .nest("/", root)
        .nest("/command", command::command_routers())
}
