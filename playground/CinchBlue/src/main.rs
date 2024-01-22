use std::future::Future;

#[derive(Debug)]
pub enum MyResult<T, E> {
    /// Contains the success value
    MyOk(T),

    /// Contains the error value
    MyErr(E),
}

async fn async_add_one(x: i32) -> i32 {
    x + 1
}

async fn run_async_add_one(x: i32) -> MyResult<i32, String> {
    if x == 0 {
        return MyResult::MyErr("x is zero".to_string());
    }
    let x = async_add_one(1).await;
    MyResult::MyOk(x)
}

/// This #[tokio::main] attribute is a macro that expands to a regular non-async main function that
/// will add all the async runtime init code + run this main function as an inner function.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Everytime you run await, you essentially are interacting with the async runtime,
    // and putting this unfinished task onto the overall "global" async task queue.
    let body = reqwest::get("https://www.rust-lang.org")
        .await?
        .text()
        .await?;

    let async_result = run_async_add_one(0).await;
    dbg!(async_result);

    println!("body = {:?}", body);
    Ok(())
}
