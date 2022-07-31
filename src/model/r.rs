use axum::extract::rejection::JsonRejection::JsonDataError;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct R<T> {
    code: i32,
    msg: String,
    data: T,
}

const OK: i32 = 1;
const FAIL: i32 = 0;

impl<T> R<T>
where
    (StatusCode, Json<R<T>>): IntoResponse,
{
    pub fn ok_data(data: T) -> impl IntoResponse {
        (
            StatusCode::OK,
            Json(R {
                code: OK,
                msg: String::from("success"),
                data,
            }),
        )
    }
}

impl R<String>
where
    (StatusCode, Json<R<String>>): IntoResponse,
{
    pub fn ok() -> impl IntoResponse {
        (
            StatusCode::OK,
            Json(R {
                code: OK,
                msg: String::from("success"),
                data: "",
            }),
        )
    }

    pub fn fail() -> impl IntoResponse {
        (
            StatusCode::OK,
            Json(R {
                code: FAIL,
                msg: String::from("error"),
                data: "",
            }),
        )
    }

    pub fn fail_msg(msg: String) -> impl IntoResponse {
        (
            StatusCode::OK,
            Json(R {
                code: FAIL,
                msg,
                data: "",
            }),
        )
    }
}
