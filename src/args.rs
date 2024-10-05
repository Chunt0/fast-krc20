#![allow(unused)]
use clap::{Arg, ArgAction, Command};

const VERSION: &str = "0.1.0";

#[derive(Debug, Clone)]
pub struct Args {
    pub network: String,
    pub resolver: bool,
    pub get_current_network: bool,
    pub get_sync_status: bool,
    pub get_info: bool,
    pub get_block_count: bool,
    pub get_peer_addresses: bool,
    pub get_sink: bool,
    pub get_connected_peer_info: bool,
    pub get_sink_blue_score: bool,
    pub get_coin_supply: bool,
    pub get_ping: bool,
    pub get_balance_address: String,
    pub get_balance_addresses: String,
    pub get_mempool_entry: String,
    pub get_mempool_entries: bool,
    pub transfer_krc20_tokens: String,
    pub build_wallet: bool,
    pub create_addrs: String,
    pub single_tx: String,
}

impl Args {
    pub fn parse() -> Result<Args, clap::Error> {
        let m = Command::new("fast-cli")
            .about(format!(
                "fast-cli: $FAST tools for KRC-20 transactions - v{}",
                VERSION
            ))
            .version(VERSION)
            .arg(
                Arg::new("network")
                    .long("network")
                    .required(false)
                    .help("Kaspad Network".to_string())
                    .value_name("FILE")
            )
            .arg(
                Arg::new("resolver")
                    .long("r")
                    .required(false)
                    .help("Use resolver instead of local node".to_string())
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("local-grpc")
                    .long("l")
                    .required(false)
                    .help("If gRPC connection should be local instead of remote".to_string())
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("get-current-network")
                    .long("cn")
                    .required(false)
                    .help("Gets current network (mainnet/devnet/testnet)".to_string())
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("get-sync-status")
                    .long("s")
                    .required(false)
                    .help("Get sync status of node".to_string())
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("get-info")
                    .long("i")
                    .required(false)
                    .help("Gets info on node".to_string())
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("get-block-count")
                    .long("bc")
                    .required(false)
                    .help("Gets block count".to_string())
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("get-peer-addresses")
                    .long("pa")
                    .required(false)
                    .help("Gets peer addresses conencted to node".to_string())
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("get-sink")
                    .long("gs")
                    .required(false)
                    .help("Gets sink".to_string())
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("get-connected-peer-info")
                    .long("pi")
                    .required(false)
                    .help("Gets info on connected peers".to_string())
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("get-sink-blue-score")
                    .long("sb")
                    .required(false)
                    .help("Gets blue score of sink".to_string())
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("get-coin-supply")
                    .long("c")
                    .required(false)
                    .help("If gRPC connection should be local instead of remote".to_string())
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("get-ping")
                    .long("p")
                    .required(false)
                    .help("Pings node".to_string())
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("get-balance-by-address")
                    .long("b")
                    .required(false)
                    .help("Get account balance from inputed address. Must provide path to a file that has the address on the first line".to_string())
                    .value_name("FILE"),
            )
            .arg(
                Arg::new("get-balance-by-addresses")
                    .long("bs")
                    .required(false)
                    .help("Get account balance from inputed addresses. Must provide a path to a file that has the addresses one per line".to_string())
                    .value_name("FILE"),
            )
            .arg(
                Arg::new("get-mempool-entry")
                    .long("me")
                    .required(false)
                    .help("Get mempool entry for a specific transaction id. Must provide a path to a file that has the transaction id on the first line".to_string())
                    .value_name("FILE")
            )
            .arg(
                Arg::new("get-mempool-entries")
                    .long("mes")
                    .required(false)
                    .help("Get mempool entries currently in the mempool".to_string())
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("transfer-krc20-tokens")
                    .long("txkrc20")
                    .required(false)
                    .help("Transfer KRC-20 tokens. Must provide a path to a file that has the transaction info".to_string())
                    .value_name("FILE")
            )
            .arg(
                Arg::new("build-wallet")
                    .long("bw")
                    .required(false)
                    .help("Build Kaspa Wallet".to_string())
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("single-tx")
                    .long("stx")
                    .required(false)
                    .help("Submit single transaction. Must provide a path to a file that contains wallets seed phrase.".to_string())
                    .value_name("FILE")
            )
            .arg(
                Arg::new("create-addrs")
                    .long("ca")
                    .required(false)
                    .help("Create N many child wallets from a master seed. Must provide a path to a file that contains seed phrase and number of addresses.".to_string())
                    .value_name("FILE")
            )
            .get_matches();

        let args = Args {
            network: m
                .get_one::<String>("network")
                .unwrap_or(&"testnet-10".to_string())
                .clone(),
            resolver: *m.get_one::<bool>("resolver").unwrap_or(&false),
            get_current_network: *m.get_one::<bool>("get-current-network").unwrap_or(&false),
            get_sync_status: *m.get_one::<bool>("get-sync-status").unwrap_or(&false),
            get_info: *m.get_one::<bool>("get-info").unwrap_or(&false),
            get_block_count: *m.get_one::<bool>("get-block-count").unwrap_or(&false),
            get_peer_addresses: *m.get_one::<bool>("get-peer-addresses").unwrap_or(&false),
            get_sink: *m.get_one::<bool>("get-sink").unwrap_or(&false),
            get_connected_peer_info: *m
                .get_one::<bool>("get-connected-peer-info")
                .unwrap_or(&false),
            get_sink_blue_score: *m.get_one::<bool>("get-sink-blue-score").unwrap_or(&false),
            get_coin_supply: *m.get_one::<bool>("get-coin-supply").unwrap_or(&false),
            get_ping: *m.get_one::<bool>("get-ping").unwrap_or(&false),
            get_balance_address: m
                .get_one::<String>("get-balance-by-address")
                .unwrap_or(&"".to_string())
                .clone(),
            get_balance_addresses: m
                .get_one::<String>("get-balance-by-addresses")
                .unwrap_or(&"".to_string())
                .clone(),
            get_mempool_entry: m
                .get_one::<String>("get-mempool-entry")
                .unwrap_or(&"".to_string())
                .clone(),
            get_mempool_entries: *m.get_one::<bool>("get-mempool-entries").unwrap_or(&false),
            transfer_krc20_tokens: m
                .get_one::<String>("transfer-krc20-tokens")
                .unwrap_or(&"".to_string())
                .clone(),
            build_wallet: *m.get_one::<bool>("build-wallet").unwrap_or(&false),
            single_tx: m
                .get_one::<String>("single-tx")
                .unwrap_or(&"".to_string())
                .clone(),
            create_addrs: m
                .get_one::<String>("create-addrs")
                .unwrap_or(&"".to_string())
                .clone(),
        };
        Ok(args)
    }
}

pub fn parse_args() -> Args {
    match Args::parse() {
        Ok(args) => args,
        Err(err) => {
            println!("{err}");
            std::process::exit(1);
        }
    }
}
