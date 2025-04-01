use serde::Serialize;
use solana_sdk::{signature::Keypair, signer::Signer};
use std::io::Write;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, BufWriter},
    path::Path,
};

fn main() {
    let keypair = Keypair::new();
    let public_key = keypair.pubkey();

    println!("Generated keypair! {keypair:#?}");
    println!("Public key: {public_key}");
    let secret_key = keypair.secret().as_bytes();

    update_env_var("PUBLIC_KEY", &public_key.to_string())
        .expect("can't update .env secret_key value");
    update_env_var("SECRET_KEY", &secret_key).expect("can't update .env secret_key value");
}

fn update_env_var<T>(key: &str, value: &T) -> std::io::Result<()>
where
    T: Serialize,
{
    let mut vars = HashMap::new();

    // Load existing .env vars
    if Path::new(".env").exists() {
        let file = File::open(".env")?;
        let reader = BufReader::new(file);
        for line in reader.lines().map_while(Result::ok) {
            if let Some((k, v)) = line.split_once('=') {
                vars.insert(k.trim().to_string(), v.trim().to_string());
            }
        }
    }

    // Serialize array as JSON and store it
    let json = serde_json::to_string(value)?;
    vars.insert(key.to_string(), json);

    // Write back to .env file
    let file = File::create(".env")?;
    let mut writer = BufWriter::new(file);
    for (k, v) in vars {
        writeln!(writer, "{}={}", k, v)?;
    }

    Ok(())
}
