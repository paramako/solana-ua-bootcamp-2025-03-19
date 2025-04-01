to run bins use
## 📦 Project Layout

```text
src/bin/
├── generate_keypair.rs
├── load_keypair.rs
└── check_balance.rs
```

▶️ How to Run Each Program
```bash
cp .env.example .env
cargo run --bin generate_keypair
cargo run --bin load_keypair
cargo run --bin check_balance
cargo run --bin airdrop
```