pub mod ws;

use crate::ws::conn::connection;
use ethers::prelude::*;
use tokio::task::JoinHandle;

#[derive(Debug)]
pub struct CustomErr {
    pub msg: String,
}

#[tokio::main]
async fn main() -> Result<(), CustomErr> {
    let connection = connection().await;

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
