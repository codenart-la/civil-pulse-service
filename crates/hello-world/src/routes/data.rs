use serde::Serialize;
use axum::{
    response::IntoResponse,
    Json,
};


#[derive(Serialize)]
struct Payload {
    name: String,
}

pub async fn read_json() -> axum::response::Response {
    // get JSON from body of request
    // print JSON to stdout
    // just return the same JSON with status 200
    let payload = Payload { name: String::from("hello!") };
    Json(payload).into_response()
}

