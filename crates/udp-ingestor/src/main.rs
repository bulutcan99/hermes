#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("{} service running...", env!("CARGO_PKG_NAME"));
    Ok(())
}
