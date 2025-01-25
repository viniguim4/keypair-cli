use ethers::prelude::*;
use ethers::providers::{Provider, Ws, Http};
use eyre::{Report, eyre};

use crate::connect::provider::EndPoint;

pub type ClientWs = SignerMiddleware<Provider<Ws>, Wallet<k256::ecdsa::SigningKey>>;
pub type ClientHttp = SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>;

#[derive(Debug, Clone)]
pub enum Client{
    WebSocket(ClientWs),
    Https(ClientHttp),
}

impl Client{
    pub fn new(provider: EndPoint, private_key: String, chain_id: Chain) -> eyre::Result<Self> {
        let wallet: LocalWallet = private_key.parse::<LocalWallet>()?.with_chain_id(chain_id);  //Chain::BinanceSmartChainTestnet
        let client : Client = match provider {
            EndPoint::WebSocket(provider) => {
                let inner_provider = provider;
                Client::WebSocket(SignerMiddleware::new(inner_provider, wallet))
            }
            EndPoint::Https(provider) => {
                let inner_provider = provider;
                Client::Https(SignerMiddleware::new(inner_provider, wallet))
            }
            EndPoint::None => {
                return Err(eyre!("No endpoint provided"));
            }
        }; 
        Ok(client)
    }

    fn content(&self, wallet: LocalWallet) -> Self {
        let client : Client = match self {
            Client::WebSocket(inner_client) => {
                Client::WebSocket(inner_client.with_signer(wallet))
            }
            Client::Https(inner_client) => {
                Client::Https(inner_client.with_signer(wallet))
            }
        }; 
        client
    }

    pub fn change(&mut self, private_key: String, chain_id: Chain) -> eyre::Result<()> {
            let wallet: LocalWallet = private_key.parse::<LocalWallet>()?.with_chain_id(chain_id);
            let content = self.content(wallet);
            *self = content;
        Ok(())
    }

    pub fn address(&self) -> H160 {
        let address : Address = match self {
            Client::WebSocket(inner_client) => {
                inner_client.address()
            }
            Client::Https(inner_client) => {
                inner_client.address()
            }
        }; 
        address
    }
}