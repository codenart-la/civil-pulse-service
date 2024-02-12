use axum::{
    extract::Path, 
    response::Html, 
    response::IntoResponse,
    routing::get, 
    routing::post, Json,
    Router,
};
use hello_world::fetch_wiki;
use serde::Serialize;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .route("/scrape/:ll", get(wiki))
        .route("/data", post(read_json));

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    println!("listening on {}", listener.local_addr()?);
    let _result = axum::serve(listener, app).await;

    Ok(())
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn wiki(Path(ll): Path<String>) -> String {
    fetch_wiki(&ll).await.unwrap_or_else(|e| {
        tracing::error!("error fetching wiki: {:?}", e);
        format!("error fetching wiki: {:?}", e)
    })
}

#[derive(Serialize)]
struct Payload {
    name: String,
}

async fn read_json() -> axum::response::Response {
    // get JSON from body of request
    // print JSON to stdout
    // just return the same JSON with status 200
    let payload = Payload { name: String::from("hello!") };
    Json(payload).into_response()
}
