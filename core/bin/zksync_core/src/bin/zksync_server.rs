use clap::Parser;

use std::{env, str::FromStr, time::Duration};
use zksync_config::configs::chain::NetworkConfig;

use zksync_config::ETHSenderConfig;
use zksync_core::{
    genesis_init, initialize_components, is_genesis_needed, setup_sigint_handler, Component,
    Components,
};
use zksync_storage::RocksDB;
use zksync_utils::wait_for_tasks::wait_for_tasks;

use std::fs::File;
use std::io::{self, BufRead, Read};

#[derive(Debug, Parser)]
#[structopt(author = "Matter Labs", version, about = "zkSync operator node", long_about = None)]
struct Cli {
    /// Generate genesis block for the first contract deployment using temporary DB.
    #[arg(long)]
    genesis: bool,
    /// Rebuild tree.
    #[arg(long)]
    rebuild_tree: bool,
    /// Comma-separated list of components to launch.
    #[arg(
        long,
        default_value = "api,tree,eth,data_fetcher,state_keeper,witness_generator,housekeeper"
    )]
    components: ComponentsToRun,
}

#[derive(Debug, Clone)]
struct ComponentsToRun(Vec<Component>);

impl FromStr for ComponentsToRun {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let components = s.split(',').try_fold(vec![], |mut acc, component_str| {
            let components = Components::from_str(component_str.trim())?;
            acc.extend(components.0);
            Ok::<_, String>(acc)
        })?;
        Ok(Self(components))
    }
}

fn split_key_value(line: &str) -> Option<(&str, &str)> {
    if let Some(pos) = line.find('=') {
        let (key, value) = line.split_at(pos);
        Some((key.trim(), value.trim_start_matches('=').trim()))
    } else {
        None
    }
}

fn set_env_detail(path: &str) {
    println!("read env:{}", path);

    if let Ok(f) = File::open(path) {
        let reader = io::BufReader::new(f);
        for line in reader.lines() {
            if let Ok(line_content) = line {
                println!("line content:{}", line_content);
                // 使用 split_once 方法分隔 key 和 value
                if let Some((key, value)) = split_key_value(&line_content) {
                    println!("Key: {}, Value: {}", key, value);
                    env::set_var(key, value);
                } else {
                    eprintln!("Invalid line format: {}", line_content);
                }
            } else {
                eprintln!("Error reading line.");
            }
        }
    } else {
        eprintln!("Error opening the file.");
    }
}
fn set_env() {
    let env_path = "/Users/lc/projects/go/src/github.com/matter-labs/zksync-era/etc/env/";

    let dev_env = env_path.to_owned() + "dev.env";
    set_env_detail(&dev_env);

    let init_env = env_path.to_owned() + ".init.env";
    set_env_detail(&init_env);
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Enter zk server main");

    // set_env();

    let opt = Cli::parse();
    vlog::init();
    let sentry_guard = vlog::init_sentry();

    if opt.genesis || is_genesis_needed().await {
        let network = NetworkConfig::from_env();
        let eth_sender = ETHSenderConfig::from_env();
        genesis_init(&eth_sender, &network).await;
        if opt.genesis {
            return Ok(());
        }
    }

    if sentry_guard.is_some() {
        vlog::info!(
            "Starting Sentry url: {}, l1_network: {}, l2_network {}",
            env::var("MISC_SENTRY_URL").unwrap(),
            env::var("CHAIN_ETH_NETWORK").unwrap(),
            env::var("CHAIN_ETH_ZKSYNC_NETWORK").unwrap(),
        );
    } else {
        vlog::info!("No sentry url configured");
    }

    let components = if opt.rebuild_tree {
        vec![Component::Tree]
    } else {
        opt.components.0
    };

    // OneShotWitnessGenerator is the only component that is not expected to run indefinitely
    // if this value is `false`, we expect all components to run indefinitely: we panic if any component returns.
    let is_only_oneshot_witness_generator_task = matches!(
        components.as_slice(),
        [Component::WitnessGenerator(Some(_), _)]
    );

    // Run core actors.
    let (core_task_handles, stop_sender, cb_receiver, health_check_handle) =
        initialize_components(components, is_only_oneshot_witness_generator_task)
            .await
            .expect("Unable to start Core actors");

    vlog::info!("Running {} core task handlers", core_task_handles.len());
    let sigint_receiver = setup_sigint_handler();

    let particular_crypto_alerts = None::<Vec<String>>;
    let graceful_shutdown = None::<futures::future::Ready<()>>;
    let tasks_allowed_to_finish = is_only_oneshot_witness_generator_task;
    tokio::select! {
        _ = wait_for_tasks(core_task_handles, particular_crypto_alerts, graceful_shutdown, tasks_allowed_to_finish) => {},
        _ = sigint_receiver => {
            vlog::info!("Stop signal received, shutting down");
        },
        error = cb_receiver => {
            if let Ok(error_msg) = error {
                vlog::warn!("Circuit breaker received, shutting down. Reason: {}", error_msg);
            }
        },
    };
    stop_sender.send(true).ok();
    RocksDB::await_rocksdb_termination();
    // Sleep for some time to let some components gracefully stop.
    tokio::time::sleep(Duration::from_secs(5)).await;
    health_check_handle.stop().await;
    vlog::info!("Stopped");
    Ok(())
}
