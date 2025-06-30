# WakaTime Blockchain Tracker

A simple Solana dApp that on-chains your daily coding time from WakaTime and burn solana if you dont code enough.

## Workflow Setup

1. **Clone & Env**
    - Copy `.env.example` to `.env` and fill in your variables (RPC_URL, KEYPAIR, VAULT_ID, CLIENT_ID, BURN_ID,
      WAKA_KEY).
2. **Build & Deploy Anchor Contract**
    - Build programs: `anchor build`
    - Deploy to your cluster: `anchor deploy`
3. **CLI Commands**
    - The CLI is in `app/src/main.rs` and supports:
        - `sol-tracker init` — initialize the on-chain vault
        - `sol-tracker fund <amount>` — fund the vault with lamports
        - `sol-tracker send-waka-time` — fetch yesterday’s WakaTime seconds and verify time

## Usage Flow

1. **Deploy** the Anchor program to your chosen cluster.
2. **Initialize** the vault PDA:
   ```bash
   sol-tracker init
   ```
3. **Fund** the vault with lamports:
   ```bash
   sol-tracker fund 1000000
   ```
4. **Verify Time** (send WakaTime data and apply penalty if under threshold):
   ```bash
   sol-tracker send-waka-time
   ```

## GitHub Actions

A workflow file already exists at `.github/workflows/run.yml`. To enable it, add the following secrets in your
repository settings:

- `WAKA_KEY`
- `KEYPAIR`
- `VAULT_ID`
- `CLIENT_ID`
- `BURN_ID`
- `RPC_URL`
- `CLUSTER`
- `FEE_LAMPORTS_AMOUNT`
