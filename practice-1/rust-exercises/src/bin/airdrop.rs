use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    native_token::LAMPORTS_PER_SOL,
    signature::Keypair,
    signer::{SeedDerivable, Signer},
};

fn main() {
    dotenvy::dotenv().ok();
    let private_key = std::env::var("SECRET_KEY").expect("no private key provided");
    let private_key =
        serde_json::from_str::<[u8; 32]>(&private_key).expect("can't deserialize private key");

    let keypair = Keypair::from_seed(&private_key).expect("can't create keypair from bytes");

    let url = "https://api.devnet.solana.com";
    let client = RpcClient::new(url.to_string());

    let public_key = keypair.pubkey();

    client
        .request_airdrop(&public_key, LAMPORTS_PER_SOL)
        .expect("can't airdrop");
}
