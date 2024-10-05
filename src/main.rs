#![allow(unused)]
mod args;
mod client;
mod wallet;

use args::{parse_args, Args};
use kaspa_addresses::{Address, Prefix, Version};
use kaspa_bip32::{DerivationPath, ExtendedPrivateKey, Language, Mnemonic, SecretKey, WordCount};
use kaspa_consensus_client::{
    Transaction, TransactionInput, TransactionOutput, UtxoEntryReference,
};
use kaspa_consensus_core::sign::verify;
use kaspa_consensus_core::subnets::SUBNETWORK_ID_NATIVE;
use kaspa_consensus_core::tx::PopulatedTransaction;
use kaspa_notify::subscription::context::SubscriptionContext;
use kaspa_rpc_core::api::rpc::RpcApi;
use kaspa_rpc_core::RpcAddress;
use kaspa_rpc_core::*;
use kaspa_wallet_core::tx::{PaymentOutput, PaymentOutputs};
use kaspa_wallet_core::wasm::sign;
use kaspa_wallet_keys::derivation_path;
use kaspa_wallet_keys::keypair;
use kaspa_wallet_keys::prelude::PrivateKey;
use kaspa_wrpc_client::{
    client::{ConnectOptions, ConnectStrategy},
    error::Error,
    prelude::{NetworkId, NetworkType},
    result::Result,
    KaspaRpcClient, Resolver, WrpcEncoding,
};
use num_bigint::BigInt;
use secp256k1::Secp256k1;
use std::fs::read;
use std::{fs, str::FromStr};
use wallet::*;

use std::time::Duration;

const TIMEOUT: u64 = 120_000; // 2 minutes

fn read_file_to_vec(file_path: &str) -> Result<Vec<String>> {
    // Read the entire file into a single String
    let contents = match fs::read_to_string(file_path) {
        Ok(contents) => contents,
        Err(e) => return Err(Error::Custom(format!("Failed to read file: {}", e))),
    };

    // Split the contents into lines and collect them into a Vec<String>
    let lines: Vec<String> = contents
        .lines() // This returns an iterator over &str
        .map(|line| line.to_string()) // Convert each &str to String
        .collect();

    Ok(lines)
}

#[tokio::main]
async fn main() -> Result<()> {
    let args: Args = parse_args();
    println!("{:#?}", args);

    let network_type: NetworkType = if args.network == "mainnet" {
        NetworkType::Mainnet
    } else {
        NetworkType::Testnet
    };

    let prefix: Prefix = if args.network == "mainnet" {
        Prefix::Mainnet
    } else {
        Prefix::Testnet
    };
    let selected_network = match network_type {
        NetworkType::Mainnet => Some(NetworkId::new(network_type)),
        NetworkType::Testnet => Some(NetworkId::with_suffix(network_type, 10)),
        NetworkType::Devnet => Some(NetworkId::new(network_type)),
        NetworkType::Simnet => Some(NetworkId::new(network_type)),
    };

    let encoding = WrpcEncoding::Borsh;
    let url: Option<&str> = if args.resolver {
        None
    } else {
        match args.network.as_str() {
            "mainnet" => Some("ws://127.0.0.1:17110"),
            "testnet-10" => Some("ws://127.0.0.1:17210"),
            _ => None,
        }
    };
    // Currently we ar enot using the resolver, must run a local node!!!!!
    // let resolver: Option<Resolver> = Some(Resolver::default());
    let resolver: Option<Resolver> = if args.resolver {
        println!(
            "Using Defualt Kaspa Resolver to connect to {}",
            args.network
        );
        Some(Resolver::default())
    } else {
        println!(
            "Using Local Node to connect to {}: {}",
            args.network,
            url.unwrap()
        );
        None
    };
    let subscription_context: Option<SubscriptionContext> = None;

    println!("wRPC - connecting to {}... ", args.network);
    let client = KaspaRpcClient::new(
        encoding,
        url,
        resolver,
        selected_network,
        subscription_context,
    )?;

    let options = ConnectOptions {
        block_async_connect: true,
        connect_timeout: Some(Duration::from_millis(TIMEOUT)),
        strategy: ConnectStrategy::Fallback,
        ..Default::default()
    };

    client.connect(Some(options)).await?;

    if args.get_sync_status {
        println!("Getting Sync Status...");
        let is_synced: bool = client.get_sync_status().await?;
        println!("Node is synced: {is_synced}");
    }

    if args.get_current_network {
        println!("Getting Current Network...");
        let current_network = client.get_current_network().await?;
        println!("Current Network: {current_network}");
    }

    if args.get_info {
        println!("Getting Info...");
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
    }

    if args.get_block_count {
        println!("Getting Block Count...");
        let block_count = client.get_block_count().await?;
        println!("Block count: {:#?}", block_count);
    }

    if args.get_peer_addresses {
        println!("Getting Peer Addresses...");
        let peer_addresses = client.get_peer_addresses().await?;
        println!("Peer Addresses: {:#?}", peer_addresses)
    }

    if args.get_sink {
        println!("Getting Sink...");
        let sink = client.get_sink().await?;
        println!("Sink: {:#?}", sink);
    }

    if args.get_connected_peer_info {
        println!("Getting Connected Peer Info...");
        let connected_peer_info = client.get_connected_peer_info().await?;
        println!("Connected Peer Info: {:#?}", connected_peer_info);
    }

    if args.get_sink_blue_score {
        println!("Getting Sink Blue Score...");
        let sink_blue_score = client.get_sink_blue_score().await?;
        println!("Sink Blue Score: {:#?}", sink_blue_score);
    }

    if args.get_coin_supply {
        println!("Getting Coin Supply...");
        let coin_supply = client.get_coin_supply().await?;
        println!("Coin Supply: {:#?}", coin_supply);
    }

    if args.get_ping {
        println!("Getting Ping...");
        client.ping().await?;
        println!("Ping: {:#?}", ());
    }

    if !args.get_balance_address.is_empty() {
        let file_path: String = args.get_balance_address;

        match read_file_to_vec(&file_path) {
            Ok(lines) => {
                for line in lines {
                    let address = RpcAddress::constructor(&line);
                    println!("Getting Balance By Address...{:#?}", address);
                    let balance = client.get_balance_by_address(address).await?;
                    println!("Balance for {line}:\n{:#?}", balance);
                }
            }
            Err(e) => eprintln!("Error: {e}"),
        }
    }

    if !args.get_balance_addresses.is_empty() {
        let file_path: String = args.get_balance_addresses;

        match read_file_to_vec(&file_path) {
            Ok(lines) => {
                let mut addresses: Vec<RpcAddress> = Vec::new();
                for line in lines {
                    addresses.push(RpcAddress::constructor(&line));
                }
                println!("Getting Balance By Address...");
                let balances = client.get_balances_by_addresses(addresses).await?;
                println!("Balances:\n{:#?}", balances);
            }
            Err(e) => eprintln!("Error: {e}"),
        }
    }

    if !args.get_mempool_entry.is_empty() {
        let file_path: String = args.get_mempool_entry;

        match read_file_to_vec(&file_path) {
            Ok(lines) => {
                if let Some(first_line) = lines.first() {
                    let tx_id = first_line.to_string();
                    let include_orphan_pool: bool = true;
                    let filter_tx_pool: bool = false;
                    println!("Getting Mempool Entry...");
                } else {
                    println!("The file is empty");
                }
            }
            Err(e) => eprintln!("Error: {e}"),
        }
    }

    if args.get_mempool_entries {
        let include_orphan_pool: bool = true;
        let filter_tx_pool: bool = false;
        println!("Getting Mempool Entry...");
    }

    if !args.transfer_krc20_tokens.is_empty() {
        let file_path: String = args.transfer_krc20_tokens;

        match read_file_to_vec(&file_path) {
            Ok(lines) => {
                let transfer_args = lines;
                println!("Transfering KRC20 Tokens...");
            }
            Err(e) => eprintln!("Error: {e}"),
        }
    }

    if args.build_wallet {
        let word_count = 12;
        match build_from_new_mnemonic(word_count) {
            Ok(phrase) => println!("Phrase: {:#?}", phrase),
            Err(e) => eprintln!("Error: {e}"),
        };
    }

    if !args.single_tx.is_empty() {
        let file_path: String = args.single_tx;

        let lines = match read_file_to_vec(&file_path) {
            Ok(lines) => lines,
            Err(e) => return Err(Error::Custom(format!("Failed to read file: {}", e))),
        };
        let mnemonic_string = lines[0].clone();
        let dest_address_string = lines[1].clone();
        let amount: u64 = match lines[2].clone().parse() {
            Ok(amount) => amount,
            Err(e) => {
                return Err(Error::Custom(format!(
                    "Failed to convert amount to u64: {}",
                    e
                )))
            }
        }; // This value is in sompi
        println!("{amount}");
        let private_key = build_from_imported_mnemonic(mnemonic_string, None).unwrap();
        let address = Address::constructor(&dest_address_string);
        let source_address = address_from_private_key(&private_key, &prefix);

        // Create UTXO entries list
        let rpc_utxo_entries = client
            .get_utxos_by_addresses(vec![source_address.clone()])
            .await?;
        let total: u64 = rpc_utxo_entries
            .clone()
            .iter()
            .map(|entry| &entry.utxo_entry.amount)
            .sum();

        if rpc_utxo_entries.is_empty() {
            eprintln!("No UTXOs available for address: {source_address}");
        }
        let change: u64 = if total >= amount {
            total - amount
        } else {
            return Err(Error::Custom("Amount exceeds total balance.".to_string()));
        };

        // Convert RPC call response of UTXOs into struct that can be used in
        // kaspa source code
        let utxo_entries: Vec<UtxoEntryReference> = match rpc_utxo_entries
            .iter()
            .map(UtxoEntryReference::try_from)
            .collect()
        {
            Ok(entries) => entries,
            Err(e) => {
                return Err(Error::Custom(format!(
                    "Failed to convert RPC entries to Vec<UtxoEntryReference>: {}",
                    e
                )))
            }
        };

        // Create outputs
        let dest_output: PaymentOutput = PaymentOutput { address, amount };
        let change_output: PaymentOutput = PaymentOutput {
            address: source_address,
            amount: change,
        };
        let outputs: Vec<PaymentOutput> = vec![dest_output, change_output];

        let sig_op_count: Option<u8> = None;
        let priority_fee: u64 = 1
            .try_into()
            .map_err(|err| Error::custom(format!("invalid fee value: {err}")))?;
        if priority_fee > total {
            return Err(Error::Custom(
                "Priority fee is larger than total amount associated with sending address."
                    .to_string(),
            ));
        };
        let payload: Vec<u8> = vec![];
        let outputs = PaymentOutputs { outputs };
        let sig_op_count = sig_op_count.unwrap_or(1);

        let mut total_input_amount = 0;
        let mut entries = vec![];

        let inputs = utxo_entries
            .into_iter()
            .enumerate()
            .map(|(sequence, reference)| {
                let UtxoEntryReference { utxo } = &reference;
                total_input_amount += utxo.amount();
                entries.push(reference.clone());
                TransactionInput::new(
                    utxo.outpoint.clone(),
                    None,
                    sequence as u64,
                    sig_op_count,
                    Some(reference),
                )
            })
            .collect::<Vec<TransactionInput>>();

        if priority_fee > total_input_amount {
            return Err(
                format!("priority fee({priority_fee}) > amount({total_input_amount})").into(),
            );
        }

        let outputs: Vec<TransactionOutput> = outputs.into();
        let tx = match Transaction::new(
            None,
            0,
            inputs,
            outputs,
            0,
            SUBNETWORK_ID_NATIVE,
            0,
            payload,
            0,
        ) {
            Ok(transaction) => transaction,
            Err(e) => {
                return Err(Error::Custom(format!(
                    "Failed to create transaction: {}",
                    e
                )))
            }
        };

        // Sign transactions
        let private_keys: Vec<[u8; 32]> = vec![private_key.secret_bytes()];
        let verify_sig: bool = true;
        let signed_tx: &Transaction = match sign(&tx, &private_keys) {
            Ok(transaction) => transaction,
            Err(e) => return Err(Error::Custom(format!("Failed to sign transaction: {}", e))),
        };

        if verify_sig {
            let (cctx, utxos) = match signed_tx.tx_and_utxos() {
                Ok(cctx_and_utxos) => cctx_and_utxos,
                Err(e) => return Err(Error::Custom(format!("Failed to sign transaction: {}", e))),
            };
            let populated_transaction = PopulatedTransaction::new(&cctx, utxos);
            let verification_result = verify(&populated_transaction);
            verification_result.expect_err("Transaction failed signature verification");
        }

        println!("Tx after signing: {:#?}", signed_tx);
        let rpc_tx = RpcTransaction::from(signed_tx);
        client.submit_transaction(rpc_tx, false).await?;
    }

    if !args.create_addrs.is_empty() {
        let file_path: String = args.create_addrs;
        match read_file_to_vec(&file_path) {
            Ok(lines) => {
                let master_phrase = lines[0].clone();
                let number_of_children = lines[1].clone().parse().unwrap_or(0);
                write_and_build_child_keys(master_phrase, number_of_children);
            }
            Err(e) => eprintln!("Error: {e}"),
        }
    }

    client.disconnect().await?;

    Ok(())
}
