use anchor_client::anchor_lang::prelude::Pubkey;
use anchor_client::Cluster;
use anyhow::anyhow;
use std::env;
use std::str::FromStr;

pub struct AppConfig {
    pub(crate) vault_id: Pubkey,
    pub(crate) client_id: Pubkey,
    pub(crate) burn_id: Pubkey,
    pub(crate) keypair_path: String,
    pub(crate) rpc_url: String,
    pub(crate) cluster: Cluster,
    pub(crate) fee_amount: u64,
    pub(crate) waka_key: String,
}

impl AppConfig {
    pub fn new() -> anyhow::Result<Self> {
        let waka_key = env::var("WAKA_KEY")?;
        let vault_id = Pubkey::from_str(&env::var("VAULT_ID")?)?;
        let client_id = Pubkey::from_str(&env::var("CLIENT_ID")?)?;
        let burn_id = Pubkey::from_str(&env::var("BURN_ID")?)?;
        let rpc_url = env::var("RPC_URL")?;
        let keypair_path = env::var("KEYPAIR_PATH")?;
        let cluster = Self::get_cluster()?;
        let withdraw_amount = env::var("FEE_LAMPORTS_AMOUNT")?.parse::<u64>()?;
        Ok(Self {
            vault_id,
            client_id,
            keypair_path,
            rpc_url,
            cluster,
            fee_amount: withdraw_amount,
            burn_id,
            waka_key,
        })
    }

    pub fn get_cluster() -> anyhow::Result<Cluster> {
        let cluster = match env::var("CLUSTER")?.as_str() {
            "TESTNET" => Cluster::Testnet,
            "MAINNET" => Cluster::Mainnet,
            "DEVNET" => Cluster::Devnet,
            _ => return Err(anyhow!("INCORRECT CLUSTER")),
        };
        Ok(cluster)
    }
}
