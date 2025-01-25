mod connect {pub mod client;
             pub mod provider;
            }
use connect::{client::{Client},
              provider::{EndPoint},
             };

pub mod utils {pub mod time;
           }
use utils::{time::{self, Clock}
            };

use ethers::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};
use eyre::Report;

#[derive(Debug, Clone)]
pub struct OnChain{
    is_connected: bool,
    url: String,
    endpoint: EndPoint,
    chain_id: ChainReq,
    start_block: BlockReq,
    client: Option<Client>,
    valid_client: bool,
}

#[derive(Debug, Clone)]
pub struct BlockReq{
    block: U64,
    latency: u64,
    timestamp: Clock,
    //is_synced: bool, //FFU
}

#[derive(Debug, Clone)]
pub struct ChainReq{
    chain_id: Chain,
}

impl OnChain{
    async fn connect(url:String) -> eyre::Result<Self>{
        let endpoint = EndPoint::connect(url.clone()).await?;
        let chain_id = ChainReq::id(endpoint.clone()).await?;
        let start_block = BlockReq::current_block(endpoint.clone()).await?;        
        Ok(Self{is_connected: true, url, endpoint, chain_id, start_block, client: None, valid_client: false})
    }

    pub fn default() -> Self {
        Self {is_connected: false, url: String::new(), endpoint: EndPoint::default(), chain_id: ChainReq::default(), start_block: BlockReq::default(), client: None, valid_client: false}
    }

    async fn connect_client(&mut self, private_key: String) -> eyre::Result<()>{
        let client = Client::new(self.get_endpoint(), private_key, self.get_chain_req().get_chain_id());
        self.set_client(client.unwrap());
        self.set_valid_client(true);
        Ok(())
    }

    async fn change_client(&mut self, private_key: String) -> eyre::Result<()>{
        let chain_id = self.get_chain_req().get_chain_id();
        self.get_mut_client().change(private_key, chain_id);
        self.set_valid_client(true);
        Ok(())
    }

    //GETTERS
    pub fn get_is_connected(&self) -> bool{
        self.is_connected
    }
    pub fn get_url(&self) -> &String{
        &self.url
    }
    pub fn get_endpoint(&self) -> EndPoint{
        self.endpoint.clone()
    }
    pub fn get_chain_req(&self) -> &ChainReq{
        &self.chain_id
    }
    pub fn get_start_block(&self) -> &BlockReq{
        &self.start_block
    }
    pub fn get_client(&self) -> Client{
        self.client.clone().unwrap()
    }
    pub fn get_mut_client(&mut self) -> &mut Client{
        self.client.as_mut().unwrap()
    }
    pub fn get_valid_client(&self) -> &bool{
        &self.valid_client
    }

    //SETTERS
    pub fn set_is_connected(&mut self, is_connected: bool){
        self.is_connected = is_connected;
    }
    pub fn set_url(&mut self, url: String){
        self.url = url;
    }
    pub fn set_endpoint(&mut self, endpoint: EndPoint){
        self.endpoint = endpoint;
    }
    pub fn set_chain_req(&mut self, chain_id: ChainReq){
        self.chain_id = chain_id;
    }
    pub fn set_start_block(&mut self, start_block: BlockReq){
        self.start_block = start_block;
    }
    pub fn set_client(&mut self, client: Client){
        self.client = Some(client);
    }
    pub fn set_valid_client(&mut self, valid_client: bool){
        self.valid_client = valid_client;
    }

    //replacer
    pub async fn replace_provider(&mut self, url: String) -> eyre::Result<Self>{
        let endpoint = EndPoint::connect(url.clone()).await?;
        let chain_id = ChainReq::id(endpoint.clone()).await?;
        let start_block = BlockReq::current_block(endpoint.clone()).await?;  
        self.set_is_connected(true);      
        self.set_url(url);
        self.set_endpoint(endpoint);
        self.set_chain_req(chain_id);
        self.set_start_block(start_block);
        self.set_valid_client(false);
        Ok(Self{is_connected: self.get_is_connected(), url: self.get_url().clone(), endpoint: self.get_endpoint(), chain_id: self.get_chain_req().clone(), start_block: self.get_start_block().clone(), client: None, valid_client: self.get_valid_client().clone()})
    }
}

impl BlockReq{
    fn default() -> Self{
        Self{ block: U64::zero(), latency: 0, timestamp: Clock::current_time() }
    }

    async fn current_block(endpoint: EndPoint) -> eyre::Result<Self>{
        let clock1 = Clock::current_time();
        let block = endpoint.current_block().await.expect("Failed to fetch block");
        let clock2 = Clock::current_time();
        let latency = time::latency(&clock1, &clock2); 
        Ok(Self{block, latency, timestamp: clock2})
    }

    //GETTERS
    pub fn get_block(&self) -> &U64{
        &self.block
    }
    pub fn get_latency(&self) -> &u64{
        &self.latency
    }
    pub fn get_timestamp(&self) -> &Clock{
        &self.timestamp
    }
}

impl ChainReq{
    fn default() -> Self{
        Self{chain_id: Chain::Mainnet}
    }

    async fn id(endpoint: EndPoint) -> eyre::Result<Self>{
        let chain_id_u256 = endpoint.get_chainid().await.expect("Failed to fetch chain id");
        let chain_id : Chain = match chain_id_u256.as_u64(){
            1        => Chain::Mainnet,                  //ETH MAINNET
            5        => Chain::Goerli,                   //Goerli
            11155111 => Chain::Sepolia,                  //Sepolia
            56       => Chain::BinanceSmartChain,        //BinanceSmartChain (bsc)
            97       => Chain::BinanceSmartChainTestnet, //BinanceSmartChainTestnet (bsc-testnet)
            42161    => Chain::Arbitrum,                 //Arbitrum
            25       => Chain::Cronos,                   //Cronos
            137      => Chain::Polygon,                  //Polygon
            80001    => Chain::PolygonMumbai,            //Mumbai
            43114    => Chain::Avalanche,                //Avalanche
            _        => Chain::Mainnet                   //ETH MAINNET for default but will display a warning!
        };
        Ok(Self{chain_id})
    }

    //GETTERS
    pub fn get_chain_id(&self) -> Chain{
        self.chain_id.clone()
    }
}

pub async fn exec_connect(url: String) -> eyre::Result<OnChain> {
    let connect_obj = OnChain::connect(url).await?;
    Ok(connect_obj)
}

pub async fn exec_connect_client(onchain: &mut OnChain, private_key: String) -> eyre::Result<()> {
    onchain.connect_client(private_key).await.expect("Failed to connect");
    Ok(())
}

pub async fn exec_change_client(onchain: &mut OnChain, private_key: String) -> eyre::Result<()> {
    onchain.change_client(private_key).await.expect("Failed to connect");
    Ok(())
}
