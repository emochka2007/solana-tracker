## WakaTime Blockchain Tracker

This repository implements **WakaTime Blockchain Tracker**, a Solana-based dApp that on-chains your daily coding
activity (as measured by WakaTime), stakes SOL into a vault PDA, and **burns SOL** if you fail to code at least 3 hours
per day by sending a penalty amount to a designated burn address.

---

## Key Components

### On-chain Program (`programs/tracker`)

* Written in Rust using [Anchor](https://www.anchor-lang.com/).
* **Instructions**:

    1. `initialize`: Creates a vault account (PDA) seeded by `[`vault\_1`, user`].
    2. `withdraw`: Transfers lamports from the vault back to the user’s wallet.
    3. `verify_time`: If your “yesterday” coding time is below the 3‑hour threshold, **burns** a fixed amount of SOL by
       transferring lamports from the vault PDA to a designated burn address.
* Vault state holds only an owner `Pubkey`.

### Off-chain Backend (`app/`)

* Rust/Tokio application (`sol-tracker`):

    * Loads configuration via environment variables.
    * Fetches last-day WakaTime coding seconds from the WakaTime REST API.
    * Funds the vault PDA with lamports.
    * (Optional) Initializes the vault and sends the `verify_time` instruction based on metrics.
* Dependencies: `reqwest`, `anchor-client`, Solana async RPC, `dotenvy`, `tracing`.

### Scripts & Testing

* **Anchor tests**: TypeScript/Mocha tests under `tests/`, driving localnet and validating program logs.
* **Yarn**: Used for linting (`prettier`) and running tests (`ts-mocha`).

### Configuration & Tooling

* **Anchor.toml**: Configures Anchor toolchain (`yarn`), feature flags, localnet, registry, and wallet path.
* **Cargo.toml** (workspace): Groups `programs/*` and `app/`, with optimized release profiles (LTO, single codegen
  unit).
* **tsconfig.json**, **.prettierignore**, **package.json**: Ensure consistent formatting and typing.

### Containerization & Deployment

* **Dockerfile** (multi-stage):

    1. **builder**: Compiles program and app in release mode.
    2. **runtime**: Slim image with necessary runtime libraries and the `sol-tracker` binary.
* **docker-compose.yml**: Defines service for local development or staging.
* **GitHub Actions** (`.github/workflows/docker-image.yml`):

    * Builds and runs Docker image nightly (03:00 UTC) or on pushes to `main`.
    * Uses repository secrets for configuration.

### To Do

* Sync SOL/USD pricing and burn an equivalent of \$10 USD in SOL.
* Enhance Docker workflow for CI/CD improvements.

---

## Getting Started

1. **Setup**

    * Copy `.env.example` to `.env` and fill in your environment variables:

      ```
      RUST_LOG="debug"
      KEYPAIR_PATH="~/.config/solana/id.json"
      BURN_ID=""
      VAULT_ID=""
      CLIENT_ID=""
      RPC_URL="https://api.testnet.solana.com"
      CLUSTER="TESTNET"
      FEE_LAMPORTS_AMOUNT="1000000"
      WAKA_KEY=""
      ```

2. **Build**

    * Local: `cargo build --release`
    * Docker: `docker build -t solana-tracker:latest .`

3. **Run**

    * Localnet:

      ```bash
      anchor test
      yarn run test
      ```
    * Docker Compose:

      ```bash
      docker-compose up --build
      ```
    * GitHub Actions: Automatically on `main` push or daily at 03:00 UTC.

---

## Usage Flow

1. Deploy the on-chain program to your chosen Solana cluster.
2. Launch `sol-tracker`: fetch WakaTime metrics, fund vault, and (optionally) burn fees if under target.
3. Monitor vault balance and transaction logs for accountability.

---

Happy coding and tracking! 🚀
