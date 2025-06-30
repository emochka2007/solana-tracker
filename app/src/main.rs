use crate::config::AppConfig;
use crate::sol::SolClient;
use crate::waka::WakaClient;
use clap::{Parser, Subcommand};
use tracing::info;

mod config;
mod helpers;
mod sol;
mod waka;
mod waka_types;

#[derive(Debug, Parser)]
#[command(
    name = "waka-sol-cli",
    about = "CLI to manage Solana vault and send Waka Time activity"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Initialize the on-chain vault
    Init,
    /// Fund the wallet with specified amount (in lamports)
    Fund {
        /// Amount in lamports to fund the vault
        amount: u64,
    },
    /// Fetch last day's Waka Time activity and send as token amount
    SendWakaTime,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;
    tracing_subscriber::fmt::init();
    let config = AppConfig::new()?;
    let sol_client = SolClient::new(&config)?;

    if config.is_cli {
        let cli = Cli::parse();
        match &cli.command {
            Commands::Init => {
                sol_client.initialize_vault(&config).await?;
                info!("Vault initialized successfully.");
            }
            Commands::Fund { amount } => {
                sol_client.fund_wallet(*amount, &config).await?;
                info!("Wallet funded with {} lamports.", amount);
            }
            Commands::SendWakaTime => {
                let time_in_secs = WakaClient::new(&config).get_activity_last_day().await?;
                sol_client
                    .send_waka_time_amount(time_in_secs, &config)
                    .await?;
                info!("Sent Waka Time activity: {} tokens.", time_in_secs);
            }
        }
    } else {
        let time_in_secs = WakaClient::new(&config).get_activity_last_day().await?;
        sol_client
            .send_waka_time_amount(time_in_secs, &config)
            .await?;
        info!("Sent Waka Time activity: {} tokens.", time_in_secs);
    }
    Ok(())
}
