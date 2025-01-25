use ethers::prelude::*;
use ethers::providers::{Provider, Ws, Http};

use eyre::{Report, eyre};

#[derive(Debug, Clone)]
pub enum EndPoint {
    WebSocket(Provider<Ws>),
    Https(Provider<Http>),
    None
}

impl EndPoint{
    pub fn default() -> Self {
        EndPoint::None
    }

    pub async fn connect(url: String) -> eyre::Result<Self, Report> {
        let provider : EndPoint;
        if url.starts_with("wss://") | url.starts_with("ws://") {
            // Connect to node via WebSocket
            provider = EndPoint::WebSocket(Provider::<Ws>::connect(&url)
                .await?);
        }
        else if url.starts_with("https://") | url.starts_with("http://") {
            // Connect to node via HTTPS
            provider = EndPoint::Https(Provider::<Http>::try_from(&url)?);
        }
        else {
            return Err(eyre!("Invalid URL format. Please provide a valid WebSocket (wss) or HTTPS URL."));
            provider = EndPoint::None;
        }
        Ok(provider)
    }

    pub async fn current_block(&self) -> eyre::Result<U64> {
        let block : U64;
        match self {
            EndPoint::WebSocket(provider) => {
                let inner_provider = provider;
                block = inner_provider.get_block_number().await.expect("Failed to fetch block");
            }
            EndPoint::Https(provider) => {
                let inner_provider = provider;
                block = inner_provider.get_block_number().await.expect("Failed to fetch block");
            }
            EndPoint::None => {
                return Err(eyre!("No endpoint provided"));
            }
        };
        Ok(block)
    }

    pub async fn get_chainid(&self) -> eyre::Result<U256> {
        let chain_id : U256;
        match self {
            EndPoint::WebSocket(provider) => {
                let inner_provider = provider;
                chain_id = inner_provider.get_chainid().await.expect("Failed to fetch chain id");
            }
            EndPoint::Https(provider) => {
                let inner_provider = provider;
                chain_id = inner_provider.get_chainid().await.expect("Failed to fetch chain id");
            }
            EndPoint::None => {
                return Err(eyre!("No endpoint provided"));
            }
        };
        Ok(chain_id)
    }

    pub async fn get_balance(&self, address: H160) -> eyre::Result<U256> {
        let balance : U256;
        match self {
            EndPoint::WebSocket(provider) => {
                let inner_provider = provider;
                balance = inner_provider.get_balance(address, None).await.expect("Failed to fetch balance");
            }
            EndPoint::Https(provider) => {
                let inner_provider = provider;
                balance = inner_provider.get_balance(address, None).await.expect("Failed to fetch balance");
            }
            EndPoint::None => {
                return Err(eyre!("No endpoint provided"));
            }
        };
        Ok(balance)
    }
}