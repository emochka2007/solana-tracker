use crate::config::AppConfig;
use anchor_client::Client as AnchorClient;
use anyhow::anyhow;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use solana_program::system_instruction::transfer;
use solana_sdk::signature::{read_keypair_file, Keypair, Signer};
use solana_sdk::transaction::Transaction;
use std::rc::Rc;
use tracing::info;

pub struct SolClient {
    rpc_client: RpcClient,
    anchor_client: AnchorClient<Rc<Keypair>>,
    vault_pda: Pubkey,
}
impl SolClient {
    pub fn new(config: &AppConfig) -> anyhow::Result<Self> {
        let client = RpcClient::new(config.rpc_url.clone());
        let anchor_client = AnchorClient::new(
            config.cluster.clone(),
            Rc::new(config.keypair.insecure_clone()),
        );
        let vault_pda = derive_vault_pda(&config.vault_id, &config.client_id);

        Ok(Self {
            rpc_client: client,
            anchor_client,
            vault_pda,
        })
    }

    pub async fn initialize_vault(&self, config: &AppConfig) -> anyhow::Result<()> {
        let vault_program = &self.anchor_client.program(config.vault_id)?;

        let sig = vault_program
            .request()
            .accounts(tracker::accounts::Initialize {
                vault: self.vault_pda,
                user: config.client_id,
                system_program: solana_system_interface::program::ID,
            })
            .args(tracker::instruction::Initialize {})
            .send()
            .await?;
        info!("Tx signature: {}", sig);
        Ok(())
    }

    pub async fn fund_wallet(&self, amount: u64, config: &AppConfig) -> anyhow::Result<()> {
        let vault_pda_derived = derive_vault_pda(&config.vault_id, &config.client_id);
        info!("{:?}", vault_pda_derived);
        let ix = transfer(&config.keypair.pubkey(), &vault_pda_derived, amount);
        let recent_blockhash = &self.rpc_client.get_latest_blockhash().await?;
        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&config.keypair.pubkey()),
            &[&config.keypair],
            recent_blockhash.clone(),
        );

        let sig = &self.rpc_client.send_and_confirm_transaction(&tx).await?;

        info!(
            "Sent {} lamports to vault PDA: {}",
            amount, vault_pda_derived
        );
        info!("Tx signature: {}", sig);
        Ok(())
    }

    pub async fn withdraw(&self, amount: u64, config: &AppConfig) {
        let vault_pda = derive_vault_pda(&config.vault_id, &config.client_id);
        let vault_program = &self.anchor_client.program(config.vault_id).unwrap();

        // 5. Send withdraw instruction
        let sig = vault_program
            .request()
            .accounts(solana_sdk::instruction::AccountMeta {
                pubkey: vault_pda,
                is_signer: false,
                is_writable: true,
            })
            .accounts(solana_sdk::instruction::AccountMeta {
                pubkey: config.client_id,
                is_signer: true,
                is_writable: true,
            })
            .args(tracker::instruction::Withdraw { amount })
            .send()
            .await
            .unwrap();
        info!("Sig {:?}", sig);
    }

    pub async fn send_waka_time_amount(
        &self,
        time_in_secs: u64,
        config: &AppConfig,
    ) -> anyhow::Result<()> {
        info!("Wakatime in secs {}", time_in_secs);
        let vault_program = &self.anchor_client.program(config.vault_id)?;

        info!("{:?}", config.vault_id.to_string());
        let sig = vault_program
            .request()
            .accounts(tracker::accounts::VerifyTime {
                vault: self.vault_pda,
                owner: config.client_id,
                burn: config.burn_id,
            })
            .args(tracker::instruction::VerifyTime {
                amount: config.fee_amount,
                time_in_secs,
            })
            .send()
            .await?;
        info!("Sig {:?}", sig);
        Ok(())
    }
}

pub fn derive_vault_pda(program_id: &Pubkey, user_pubkey: &Pubkey) -> Pubkey {
    let (vault_pda, _bump) =
        Pubkey::find_program_address(&[b"vault_1", user_pubkey.as_ref()], &program_id);
    vault_pda
}
