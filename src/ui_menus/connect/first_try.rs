use on_chain::{self, OnChain};
use crate::{Environment};

pub async fn begin() -> Result<OnChain, Box<dyn std::error::Error>> {
    let env = Environment::loadenv();
    let rpc = env.get_rpc();
    let on_chain_struct = on_chain::exec_connect(rpc).await?;
    Ok(on_chain_struct)
}
