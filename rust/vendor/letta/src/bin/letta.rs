//! Main binary entry point for the Letta CLI.

#[tokio::main]
async fn main() -> miette::Result<()> {
    letta::cli::run().await
}
