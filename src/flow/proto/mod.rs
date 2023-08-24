// @generated
include!("model/mod.rs");
pub use flow::access;
pub use flow::entities;
pub use flow::execution;

use self::access::PingRequest;

impl access::access_api_client::AccessApiClient<tonic::transport::Channel> {
    pub async fn get_client(url: &'static str) -> Result<Self, tonic::transport::Error> {
        let conn = tonic::transport::Endpoint::new(url)?.connect().await?;
        let mut client = Self::new(conn);
        let _ = client.ping(PingRequest {}).await;
        Ok(client)
    }
    pub async fn mainnet() -> Result<Self, tonic::transport::Error> {
        Self::get_client("http://access.mainnet.nodes.onflow.org:9000").await
    }
    pub async fn testnet() -> Result<Self, tonic::transport::Error> {
        Self::get_client("http://access.devnet.nodes.onflow.org:9000").await
    }
    pub async fn sandbox() -> Result<Self, tonic::transport::Error> {
        Self::get_client("http://access.sandboxnet.nodes.onflow.org:9000").await
    }
}
