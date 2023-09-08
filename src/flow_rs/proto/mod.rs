// @generated
include!("model/mod.rs");
use cadence_json::ValueOwned;
pub use flow::access;
pub use flow::entities;
pub use flow::execution;

use self::access::PingRequest;
use self::entities::Event;

impl access::access_api_client::AccessApiClient<tonic::transport::Channel> {
    pub async fn get_client(url: String) -> Result<Self, tonic::transport::Error> {
        let conn = tonic::transport::Endpoint::new(url)?.connect().await?;
        let mut client = Self::new(conn);
        let _ = client.ping(PingRequest {}).await;
        Ok(client)
    }
    pub async fn mainnet() -> Result<Self, tonic::transport::Error> {
        Self::get_client("http://access.mainnet.nodes.onflow.org:9000".to_string()).await
    }
    pub async fn testnet() -> Result<Self, tonic::transport::Error> {
        Self::get_client("http://access.devnet.nodes.onflow.org:9000".to_string()).await
    }
    pub async fn sandbox() -> Result<Self, tonic::transport::Error> {
        Self::get_client("http://access.sandboxnet.nodes.onflow.org:9000".to_string()).await
    }
}

impl Event {
    /// Parses the payload of this event as a cadence JSON value.
    pub fn parse_payload_as_value(&self) -> serde_json::Result<cadence_json::ValueOwned> {
        serde_json::from_slice(&self.payload)
    }

    /// Parses the payload of this event.
    pub fn parse_payload(&self) -> serde_json::Result<cadence_json::CompositeOwned> {
        match self.parse_payload_as_value()? {
            ValueOwned::Event(composite) => Ok(composite),
            _ => panic!("Invalid payload for Event"),
        }
    }
}
