//! Run with
//!
//! ```not_rust
//! cargo run -p example-hello-world
//! ```

use axum::{response::Html, routing::get, Router};
use hello_world::fetch_wiki;

/*
enum Result<T, E> {
    Ok(T),
    Err(E),
}

enum Option<T> {
    Some(T),
    None,
}

interface I {

}

var k: dyn I + J + K
*/

macro_rules! say_hi {
    () => {
        println!("hi");
    };

    ($name:expr) => {
        println!("hi {}", $name);
    };
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
enum Weather {
    Sunny,
    CloudyWithAChanceOfMeatballs(String),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct BiggoStruct {
    weather: Weather,
    color: Color,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new().route("/", get(handler));

    let color = Color {
        red: 0,
        green: 1,
        blue: 2,
        alpha: 3,
    };

    dbg!(color.clone());
    tracing::info!("Hello, World! {:?}", color);
    tracing::info!(
        "but what if it was json?\n{}",
        serde_json::to_string(&color)?
    );
    tracing::info!(
        "but what if it was yaml?\n{}",
        serde_yaml::to_string(&color)?
    );

    let biggo_struct = BiggoStruct {
        weather: Weather::Sunny,
        color: color.clone(),
    };
    tracing::error!(
        "but what if it was yaml?\n{}",
        serde_yaml::to_string(&biggo_struct)?
    );

    say_hi!();
    say_hi!("Mario");

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    println!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await;

    Ok(())
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
