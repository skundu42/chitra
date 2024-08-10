use std::env;
use alloy::providers::{ProviderBuilder, WsConnect, RootProvider};
use alloy::pubsub::PubSubFrontend;

pub async fn init_provider() -> eyre::Result<RootProvider<PubSubFrontend>> {
    let ws = WsConnect::new(env::var("WSS_URL").expect("WSS_URL environment variable is not set"));
    let provider = ProviderBuilder::new().on_ws(ws).await?;
    Ok(provider)
}
