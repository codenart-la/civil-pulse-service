use axum::{
    Router,
    routing::{
        get,
        post,
    }
};

pub mod routes;
pub use routes::base::handler;
pub use routes::scrape::wiki;
pub use routes::data::read_json;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // TODO: perhaps the main function should take command line arguments
    // QUESTION: perhaps the lib folder should be the one instantiating / running the router?
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

