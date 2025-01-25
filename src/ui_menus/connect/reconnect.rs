use console::{style, Color, Term};
use eyre::{eyre, Report};
use crate::{Settings};

use on_chain::{self, OnChain};
use crate::{Environment, connect_infos, ping_color};

pub async fn reconnect(globalsettings: &mut Settings, term: &mut Term, onchain : &mut OnChain) ->  eyre::Result<OnChain, Report> {
    clear_screen(term);;
    let env = Environment::loadenv();
    let rpc = env.get_rpc();
    let mut onchain_clone = onchain.clone();
    let new_onchain = onchain_clone.replace_provider(rpc).await;
    let mut is_connected : bool = false;
    match &new_onchain {
        Ok(insider) => { let infos = connect_infos(insider.clone());
                    println!("                   {} {} | {} | {} {} | {} {} | {}", style("UTC:").bold(),
                    style(format!("{:02}:{:02}:{:02}", infos.2.hours, infos.2.minutes, infos.2.seconds)).bold(),
                    style("CONNECTED").green().bold(),
                    style("CHAIN:").yellow().bold(),
                    style(format!("{}", infos.1)).yellow().bold(),
                    style("AT BLOCK:").yellow().bold(),
                    style(format!("{}", infos.3)).yellow().bold(),
                    ping_color(infos.0));
                    is_connected = true;
                    println!("{}", style("───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────\n").color256(238) );
                }
        Err(_) => {println!("{}", style("                 Error: Consider changing or inserting a new RPC_ENDPOINT in env.json then hit connect.                ").red().bold());
                   println!("{}", style("───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────\n").color256(238) );}
    };
        // Return the appropriate result
    new_onchain
}

fn clear_screen(term: &mut Term){
    term.move_cursor_to(0,0);
    term.move_cursor_down(11);
    term.clear_to_end_of_screen();
}
