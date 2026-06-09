mod server;
mod tools;

use rmcp::{transport::stdio, ServiceExt};
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
