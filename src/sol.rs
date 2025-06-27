use crate::consts::BURN_ADDRESS;
use anchor_client::{Client as AnchorClient, Cluster};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use solana_program::system_instruction;
use solana_sdk::signature::{Keypair, Signer, read_keypair_file};
use solana_sdk::transaction::Transaction;
use std::env;
use std::rc::Rc;
use std::str::FromStr;
use tracing::info;

pub struct SolClient {
    rpc_client: RpcClient,
    anchor_client: AnchorClient<Rc<Keypair>>,
    keypair: Keypair,
    vault_id: Pubkey,
    client_id: Pubkey,
}
impl SolClient {
    pub fn new() -> anyhow::Result<Self> {
        let client = RpcClient::new("https://api.testnet.solana.com".to_string());
        let path = env::var("KEYPAIR_PATH")?;
        let keypair = read_keypair_file(path).unwrap();
        let anchor_client = AnchorClient::new(Cluster::Testnet, Rc::new(keypair.insecure_clone()));

        let vault_id = Pubkey::from_str("rS9PUxa2aE2XmEMVXSkx6owFpGyiVjpBfYCVMD5Yy9v")?;
        let client_id = Pubkey::from_str("H2G4hT7PrzCRY7gZvxeZtoB1DN5yrTxoroQHsMNJbhxy")?;
        Ok(Self {
            rpc_client: client,
            anchor_client,
            keypair,
            vault_id,
            client_id,
        })
    }

    pub async fn initialize_vault(&self) {
        let vault_pda = derive_vault_pda(&self.vault_id, &self.client_id);

        let vault_program = &self.anchor_client.program(self.vault_id).unwrap();

        let sig = vault_program
            .request()
            .accounts(tracker::accounts::Initialize {
                vault: vault_pda,
                user: self.client_id,
                system_program: solana_sdk::system_program::ID,
            })
            .args(tracker::instruction::Initialize {})
            .send()
            .await
            .unwrap();
        info!("Tx signature: {}", sig);
    }

    pub async fn fund_wallet(&self) {
        // use non-blocking rpc client
        let vault_pda_derived = derive_vault_pda(&self.vault_id, &self.client_id);
        info!("{:?}", vault_pda_derived);
        let lamports = 10_000_000;
        let ix = system_instruction::transfer(&self.keypair.pubkey(), &vault_pda_derived, lamports);
        let recent_blockhash = &self.rpc_client.get_latest_blockhash().await.unwrap();
        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&self.keypair.pubkey()),
            &[&self.keypair],
            recent_blockhash.clone(),
        );

        let sig = &self
            .rpc_client
            .send_and_confirm_transaction(&tx)
            .await
            .unwrap();

        info!(
            "Sent {} lamports to vault PDA: {}",
            lamports, vault_pda_derived
        );
        info!("Tx signature: {}", sig);
    }

    pub async fn withdraw(&self) {
        let vault_pda = derive_vault_pda(&self.vault_id, &self.client_id);
        // 4. Withdraw amount
        let amount: u64 = 1_000_000; // 0.001 SOL
        let vault_program = &self.anchor_client.program(self.vault_id).unwrap();

        // 5. Send withdraw instruction
        let sig = vault_program
            .request()
            .accounts(solana_sdk::instruction::AccountMeta {
                pubkey: vault_pda,
                is_signer: false,
                is_writable: true,
            })
            .accounts(solana_sdk::instruction::AccountMeta {
                pubkey: self.client_id,
                is_signer: true,
                is_writable: true,
            })
            .args(tracker::instruction::Withdraw { amount })
            .send()
            .await
            .unwrap();
        info!("Sig {:?}", sig);
    }

    pub async fn send_waka_time_amount(&self, time_in_secs: u64) {
        let vault_program = &self.anchor_client.program(self.vault_id).unwrap();

        // 0.001 SOL
        let amount = 1_000_000;

        let sig = vault_program
            .request()
            .accounts(tracker::accounts::TransferLamports {
                from: self.client_id,
                to: Pubkey::from_str_const(BURN_ADDRESS),
                system_program: solana_sdk::system_program::ID,
            })
            .args(tracker::instruction::VerifyTime {
                amount,
                time_in_secs,
            })
            .send()
            .await
            .unwrap();
        info!("Sig {:?}", sig);
    }
}

pub fn derive_vault_pda(program_id: &Pubkey, user_pubkey: &Pubkey) -> Pubkey {
    let (vault_pda, _bump) =
        Pubkey::find_program_address(&[b"vault_1", user_pubkey.as_ref()], &program_id);
    vault_pda
}
