use std::time::Duration;

use ethers::prelude::*;

pub async fn connection() -> Provider<Ws> {
    let endpoint = "";
    let url = String::from(endpoint);
    let ws = Ws::connect(url).await.unwrap();
    let provider = Provider::new(ws).interval(Duration::from_millis(2000));
    provider
}
