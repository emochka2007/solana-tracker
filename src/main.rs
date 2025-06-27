use crate::sol::SolClient;
use crate::waka::WakaClient;

mod consts;
mod sol;
mod waka;
mod waka_types;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;
    tracing_subscriber::fmt::init();
    let sol_client = SolClient::new()?;
    let time_in_secs = WakaClient::new().get_activity_last_day().await?;
    sol_client.send_waka_time_amount(time_in_secs).await;
    Ok(())
}
