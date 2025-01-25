use console::{style, Color, Term};
use grin_cli::ui_menus::{master::{master_menu},
                         connect::{first_try}};

#[tokio::main]
async fn main() {
    let mut term = Term::stdout();
    term.clear_screen().unwrap();
    println!(
"{}
{}
{}
{}{}
{}
{}",
style("───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────").color256(238),
style("                                             ◀ Projeto Blockchain ▶                                                    ").green().bold().bg(Color::Color256(238)),
style("───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────").color256(238),
style("                                dBBBBBBP            dBb      dBP    dBb  dBP          dBP
                               dBP      dBBBBBBb       dB   dBP    dBP  d8P          dBP
                              dBP dBBBBBBBP   dBP dBP dBPb dBP    dBBBBBBBbdBBBBBb  dBP
                             dBP  dBP  dBP dBBP  dBP dBBPdBBP dBBP    dBP dBP dBP
                            dBBBBBBP  dBP   dBBPdBP dBP  dBP         dBP dBBBBBP  dBP
").yellow().bold(),
style("───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────").color256(238),
style("                            ◀  Grin-CLI v0.1.0 Disclaimer: Use at your own risk. DYOR ▶                                ").green().bold().bg(Color::Color256(238)),
style("───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────").color256(238));

    let onchain_req = first_try::begin().await;
    let mut is_connected : bool = false;
    match &onchain_req {
        Ok(onchain) => { let infos = grin_cli::connect_infos(onchain.clone());
                        println!("                   {} {} | {} | {} {} | {} {} | {}", style("UTC:").bold(),
                        style(format!("{:02}:{:02}:{:02}", infos.2.hours, infos.2.minutes, infos.2.seconds)).bold(),
                        style("CONNECTED").green().bold(),
                        style("CHAIN:").yellow().bold(),
                        style(format!("{}", infos.1)).yellow().bold(),
                        style("AT BLOCK:").yellow().bold(),
                        style(format!("{}", infos.3)).yellow().bold(),
                        grin_cli::ping_color(infos.0));
                        is_connected = true;
                        }
        Err(_) => println!("{}", style("                 Error: Consider changing or inserting a new RPC_ENDPOINT in env.json then hit connect.                ").red().bold())
    }
    println!("{}", style("───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────\n").color256(238) );
    if is_connected == true {
        let mut onchain = onchain_req.unwrap();  
        let mut settings = grin_cli::Settings::new();
        master_menu::master_menu(&mut settings, &mut term, &mut onchain).await;
    }
    else {
        let mut onchain = on_chain::OnChain::default();
        let mut settings = grin_cli::Settings::new();
        master_menu::master_menu(&mut settings, &mut term, &mut onchain).await;
    }
}

