use crate::config::AppConfig;
use crate::sol::SolClient;
use crate::waka::WakaClient;

mod config;
mod sol;
mod waka;
mod waka_types;

/**
- Anchor build + deploy
- Initialize vault
- Fund
- Send waka time
*/
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;
    tracing_subscriber::fmt::init();
    let config = AppConfig::new()?;
    let sol_client = SolClient::new(&config)?;
    let time_in_secs = WakaClient::new(&config).get_activity_last_day().await?;
    sol_client.initialize_vault(&config).await?;
    sol_client.fund_wallet(1_000_000, &config).await?;
    sol_client
        .send_waka_time_amount(time_in_secs, &config)
        .await?;
    Ok(())
}
