mod server;
mod tools;

use rmcp::{ServiceExt, transport::stdio};
use server::Calculator;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let service = Calculator::new()
        .serve(stdio())
        .await
        .inspect_err(|e| eprintln!("Server error: {e}"))?;

    service.waiting().await?;
    Ok(())
}
