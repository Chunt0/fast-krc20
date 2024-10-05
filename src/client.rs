#![allow(unused)]
use kaspa_notify::subscription::context::SubscriptionContext;
use kaspa_rpc_core::{api::rpc::RpcApi, GetServerInfoResponse};
use kaspa_txscript::{opcodes::codes::*, script_builder::ScriptBuilder};
use kaspa_wallet_keys::privatekey::PrivateKey;
use kaspa_wrpc_client::{
    client::{ConnectOptions, ConnectStrategy},
    prelude::NetworkId,
    prelude::NetworkType,
    result::Result,
    KaspaRpcClient, Resolver, WrpcEncoding,
};

use std::fs;
use std::time::Duration;

async fn transfer_krc20_tokens(network: NetworkType) -> Result<()> {
    let encoding = WrpcEncoding::Borsh;

    let contents = fs::read_to_string("transfer_args.txt");
    let transfer_args: Vec<String> = contents
        .expect("Failed to read file")
        .lines()
        .map(|line| line.to_string())
        .collect();

    // Parse transfer arguements
    let private_key_arg = transfer_args[0].clone();
    let ticker = transfer_args[2].clone();
    let priority_fee_value = transfer_args[3].clone();
    let timeout = transfer_args[4].clone().parse().unwrap_or(120_000);
    let log_level = transfer_args[5].clone();
    let dest = transfer_args[6].clone();
    let amount = transfer_args[7].clone();

    let url: Option<&str> = None;
    let resolver = Some(Resolver::default());
    let selected_network = match network {
        NetworkType::Mainnet => Some(NetworkId::new(network)),
        NetworkType::Testnet => Some(NetworkId::with_suffix(network, 10)),
        NetworkType::Devnet => Some(NetworkId::new(network)),
        NetworkType::Simnet => Some(NetworkId::new(network)),
    };
    let subscription_context: Option<SubscriptionContext> = None;

    println!("wRPC - connecting to {}...", network);
    let client = KaspaRpcClient::new(
        encoding,
        url,
        resolver,
        selected_network,
        subscription_context,
    )?;

    let options = ConnectOptions {
        block_async_connect: true,
        connect_timeout: Some(Duration::from_millis(timeout)),
        strategy: ConnectStrategy::Fallback,
        ..Default::default()
    };

    client.connect(Some(options)).await?;

    let GetServerInfoResponse {
        is_synced,
        server_version,
        network_id,
        has_utxo_index,
        ..
    } = client.get_server_info().await?;

    println!("Node version: {server_version}");
    println!("Network: {network_id}");

    println!("Node is synced: {is_synced}");
    println!("Node is indexing: {has_utxo_index}");

    let private_key = PrivateKey::try_new(&private_key_arg);
    println!("Suibmitting private key: {:#?}", private_key);

    println!("Determining public key");
    let public_key = private_key
        .expect("Failed to create private key")
        .to_public_key();
    let address = public_key.ok().unwrap().to_address(network);
    println!("Determining address: {:#?}", address);

    let gas_fee = 0.3;

    /*
    let mut script = ScriptBuilder::new()
        .add_data(public_key.to_x_only_public_key())?
        .add_op(OpCheckSig)?
        .add_op(OpFalse)?
        .add_op(OpIf)?;
    */

    client.disconnect().await?;

    Ok(())
}
