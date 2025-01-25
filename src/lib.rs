pub mod ui_menus;

use console::{style, Color, Term};
use on_chain::utils::time::{Clock};
use ui_menus::wlltmngr::settings;
use on_chain::{OnChain};
use ethers::types::{U64, Chain};
use tokio::runtime::Handle;
use eyre::{eyre, Report};

use std::fs::File;
use std::io::Read;
use serde_json;
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct Settings{
    wllt : settings::WlltmngrSettings,
}

impl Settings {
    pub fn new() -> Self {
        Settings { wllt: settings::WlltmngrSettings::default() }
    }

    pub fn getwlltsettings(&mut self) -> &mut settings::WlltmngrSettings {
        &mut self.wllt
    }
}

#[derive(Debug, Deserialize)]
pub struct Environment {
    RPC_ENDPOINT : String,
}

impl Environment{
    pub fn loadenv() -> Self {
        let parsed_env = readenv().expect("Failed to find JSON file.");

        let RPC_ENDPOINT = parsed_env.RPC_ENDPOINT;

        Environment { RPC_ENDPOINT }
    }
    
    pub fn get_rpc(&self) -> String {
        self.RPC_ENDPOINT.clone()
    }

}

pub fn connect_infos(onchain: OnChain) -> (u64, Chain, Clock, U64) {
    let ping = onchain.get_start_block().get_latency();
    let chain_id = onchain.get_chain_req().get_chain_id();
    let clock = onchain.get_start_block().get_timestamp();
    let block = onchain.get_start_block().get_block();
    (ping.clone(), chain_id, clock.clone(), block.clone())
}

pub fn ping_color(ping: u64) -> console::StyledObject<String> {
    if ping < 100 {
        style(format!("PING: {} ms", ping)).green().bold()
    } else if ping < 200 {
        style(format!("PING: {} ms", ping)).yellow().bold()
    } else {
        style(format!("PING: {} ms", ping)).red().bold()
    }
}

pub fn readenv() -> eyre::Result<Environment>{
    // Read the contents of the file into a string
    let mut file = File::open("env.json")?;
    let mut json_str = String::new();
    file.read_to_string(&mut json_str)?;
            // Parse the JSON string into a serde_json::Value
    let parsed_json: Environment = serde_json::from_str(&json_str)?;
    Ok(parsed_json)
}
