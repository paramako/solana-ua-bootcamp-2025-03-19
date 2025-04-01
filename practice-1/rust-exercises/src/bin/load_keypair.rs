use solana_sdk::{
    signature::Keypair,
    signer::{SeedDerivable, Signer},
};

fn main() {
    dotenvy::dotenv().ok();

    let private_key = std::env::var("SECRET_KEY").expect("no private key provided");
    let private_key =
        serde_json::from_str::<[u8; 32]>(&private_key).expect("can't deserialize private key");

    let keypair = Keypair::from_seed(&private_key).expect("can't create keypair from bytes");
    let public_key = keypair.pubkey();

    println!("Public key: {public_key}");
}
