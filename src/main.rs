pub mod modules;

use crate::modules::{conn::connection, https::run_server};
use dotenv::dotenv;
use ethers::prelude::*;
use std::env;
use tokio::task::JoinHandle;

#[derive(Debug)]
pub struct CustomErr {
    pub msg: String,
}

#[tokio::main]
async fn main() -> Result<(), CustomErr> {
    dotenv().ok();
    let node_ws_eth = env::var("WSS_NODE_ETH").expect("WSS Node endpoint is missing");
    let certificate_path = env::var("CERTIFICATE_PATH").expect("Certificate path is missing");
    let key_path = env::var("KEY_PATH").expect("Key path is missing");
    let port = env::var("HTTPS_PORT").expect("Https port is missing");
    let connection = connection(node_ws_eth.as_str()).await;

    let connection1 = connection.clone();
    let connection2 = connection.clone();
    let mut thread_handles: Vec<JoinHandle<()>> = vec![];

    thread_handles.push(tokio::spawn(async move {
        let mut blocks_stream = connection1.subscribe_blocks().await.unwrap();
        while let Some(block) = blocks_stream.next().await {
            println!("Block: {:?}", block);
        }
    }));

    thread_handles.push(tokio::spawn(async move {
        let mut txs_stream = connection2.subscribe_pending_txs().await.unwrap();
        while let Some(tx) = txs_stream.next().await {
            println!("Transaction hash: {:?}", tx);
        }
    }));

    thread_handles.push(tokio::spawn(async move {
        if let Err(e) =
            run_server(certificate_path.as_str(), key_path.as_str(), port.as_str()).await
        {
            eprintln!("FAILED: {}", e);
            std::process::exit(1);
        }
    }));

    println!("Started app");
    let join_res = futures::future::join_all(thread_handles).await;

    for e in join_res {
        if e.is_err() {
            return Err(CustomErr {
                msg: e.err().unwrap().to_string(),
            });
        }
    }

    Ok(())
}
