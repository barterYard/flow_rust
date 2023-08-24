use std::env;

use tonic::transport::Channel;

use crate::access::access_api_client::AccessApiClient;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlowNetwork {
    Testnet,
    Mainnet,
    Sandbox,
    None,
}

impl FlowNetwork {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Mainnet => "mainnet",
            Self::Testnet => "testnet",
            Self::Sandbox => "sandbox",
            _ => "none",
        }
    }

    fn from_string(network: &str) -> FlowNetwork {
        match network {
            "mainnet" => FlowNetwork::Mainnet,
            "testnet" => FlowNetwork::Testnet,
            "sandbox" => FlowNetwork::Sandbox,
            _ => FlowNetwork::None,
        }
    }

    pub fn get() -> FlowNetwork {
        if let Ok(x) = env::var("FLOW_ENV") {
            let flow_network = FlowNetwork::from_string(&x);
            if flow_network != FlowNetwork::None {
                return flow_network;
            }
            panic!("{} is not valid flow environment set. Please set the FLOW_ENV var to either mainnet, testnet, sandbox",x)
        }
        panic!("Please set the FLOW_ENV var to either mainnet, testnet, sandbox")
    }

    pub async fn get_client() -> AccessApiClient<Channel> {
        Self::get().get_flow_client().await
    }

    pub async fn get_flow_client(&self) -> AccessApiClient<Channel> {
        match self {
            Self::Mainnet => AccessApiClient::mainnet().await.unwrap(),
            Self::Testnet => AccessApiClient::testnet().await.unwrap(),
            Self::Sandbox => AccessApiClient::sandbox().await.unwrap(),
            Self::None => panic!("No valid flow environment set. Please set the FLOW_ENV var to either mainnet, testnet, sandbox"),
        }
    }
}
