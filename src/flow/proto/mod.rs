// @generated
include!("model/mod.rs");
pub use flow::access;
pub use flow::entities;
pub use flow::execution;

impl access::access_api_client::AccessApiClient<tonic::transport::Channel> {
    pub async fn mainnet() -> Result<Self, tonic::transport::Error> {
        let conn = tonic::transport::Endpoint::new("http://access.mainnet.nodes.onflow.org:9000")?
            .connect()
            .await?;
        Ok(Self::new(conn))
    }
    pub async fn testnet() -> Result<Self, tonic::transport::Error> {
        let conn = tonic::transport::Endpoint::new("http://access.devnet.nodes.onflow.org:9000")?
            .connect()
            .await?;
        Ok(Self::new(conn))
    }
}
