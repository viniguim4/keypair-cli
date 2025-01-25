use console::{Term, style, Color, StyledObject};
use dialoguer::{theme::ColorfulTheme, Select, Confirm, Input};
use indicatif::{ProgressBar, ProgressStyle};
use eyre::{eyre, Report};
use on_chain::{self, OnChain};
use async_recursion::async_recursion;

use crate::{ui_menus::master::master_menu,
            Settings};

#[async_recursion]
pub async fn wallet_mngr_menu(globalsettings: &mut Settings, term: &mut Term, onchain: &mut OnChain) -> eyre::Result<()> {

    let items = vec!["● Create Wallet", "● Load Wallets", "▶ Settings\n" , "◀ Back"];

    globalsettings.getwlltsettings().update();

    let n_wallets = globalsettings.getwlltsettings().get_n_wallets();
    let n_child = globalsettings.getwlltsettings().get_n_child();
    let w_size = globalsettings.getwlltsettings().get_mnemonic_size();

    println!("{}", style("─────────────────────────────────────────────────────────").color256(238));
    println!("{}", style("                   Wallet Manager Menu                   ").cyan().bold().bg(Color::Color256(238)));
    println!("{}", style("─────────────────────────────────────────────────────────").color256(238));
    wllt_settings_printer(globalsettings);
    wllt_load_settings_printer(globalsettings);
    println!("{}", style("─────────────────────────────────────────────────────────\n").color256(238));

    let selection = Select::with_theme(&ColorfulTheme::default())
                            .with_prompt("Select an option you want to use: ")
                            .items(&items)
                            .default(0)
                            .interact_on_opt(&Term::stderr())?;

    match selection {
        Some(index) => {
            match index {
                0 => {// CREATIN WALLETS
                        globalsettings.getwlltsettings().update();
                        if globalsettings.getwlltsettings().get_exists() == false {
                            globalsettings.getwlltsettings().set_is_loaded(false);
                            println!("{}", style("Generating...\n").color256(235));
                            let bar = new_bar_obj(n_wallets);
                            bar.set_position(0);
                            for i in 0..n_wallets{
                                match wllt_gen::exec(i, n_child, w_size) {
                                    Ok(()) => {clear_for_pb(term);
                                                bar.inc(1);
                                                println!("\n\n{}{}{}{}{}\n", style("Wallets successfully created!   [").green().bold().bg(Color::Color256(22)), style(i+1).green().bold().bg(Color::Color256(22)),style("/").green().bold().bg(Color::Color256(22)), style(n_wallets).green().bold().bg(Color::Color256(22)), style("]").green().bold().bg(Color::Color256(22)))
                                    },
                                    Err(_err) =>{clear_for_pb(term);
                                                bar.inc(1);
                                                eprintln!("\n\n{} {}\n", style("Error:").red().bold(), style("Wallet exists or wrong root path").red() )
                                    }
                                }                                
                            }
                        } else {
                            println!("\n{}{}\n", style("WARNING: ").red().bold().bg(Color::Color256(124)), style("Wallets already created. Consider DELETING MANUALLY!!! Make sure to BACKUP it before.").red().bold().bg(Color::Color256(124)));
                        }
                        confirm(term);
                        clear_screen(term);
                        wallet_mngr_menu(globalsettings, term, onchain).await
                    },
                1 => {// Load Wallets logic
                    let present_wllts = globalsettings.getwlltsettings().get_present_wllts();
                    globalsettings.getwlltsettings().clear_wallets_vec();
                    globalsettings.getwlltsettings().update();
                    println!("{}", style("Loading...\n").color256(235));
                    let bar = new_bar_obj(present_wllts);
                    bar.set_position(0);
                    if present_wllts > 0 {
                        for i in 0..present_wllts{
                            clear_for_pb(term);
                            bar.inc(1);
                            let wallet_to_append = wllt_gen::exec_load(i).unwrap();
                            globalsettings.getwlltsettings().append_wallets_vec(wallet_to_append);
                            println!("\n\n{}{}{}{}{}\n", style("Wallets successfully loaded!   [").green().bold().bg(Color::Color256(22)), style(i+1).green().bold().bg(Color::Color256(22)),style("/").green().bold().bg(Color::Color256(22)), style(present_wllts).green().bold().bg(Color::Color256(22)), style("]").green().bold().bg(Color::Color256(22)))
                        }
                    }
                    else {
                        clear_for_pb(term);
                        println!("\n{}{}\n", style("WARNING: ").red().bold().bg(Color::Color256(124)), style("No wallets to load. Consider CREATING WALLETS!!!").red().bold().bg(Color::Color256(124)));
                    }
                    globalsettings.getwlltsettings().set_is_loaded(true);
                    confirm(term);
                    clear_screen(term);
                    wallet_mngr_menu(globalsettings, term, onchain).await

                }, // Load Wallets logic
                2 => wallet_mngr_settings(globalsettings, term, onchain).await, // Settings logic
                3 => main_callback(globalsettings, term, onchain).await, // BACK LOGIC
                _ => Ok(println!("Error"))
            };
        },
        None => println!("User did not select anything")
    }

    Ok(())
}

#[async_recursion]
pub async fn wallet_mngr_settings(globalsettings: &mut Settings, term: &mut Term, onchain: &mut OnChain) -> eyre::Result<()>{
    println!("{}", style("─────────────────────────────────────────────────────────").color256(238));
    println!("{}", style("                 Wallet Manager Settings                 ").cyan().bold().bg(Color::Color256(238)));
    println!("{}", style("─────────────────────────────────────────────────────────").color256(238));

    let items = vec!["⚙ Mnemonic Size", "⚙ Number of Wallets", "⚙ Number of Children\n" , "◀ Back"];
    let selection = Select::with_theme(&ColorfulTheme::default())
                            .with_prompt("Select an option you want to change: ")
                            .items(&items)
                            .default(0)
                            .interact_on_opt(&Term::stderr())?;
    match selection {
        Some(index) =>  {
            match index {
                0 => {
                        let new_m_size = Input::new().with_prompt("Enter new Mnemonic Size").default("12".to_string()).show_default(false).interact_text_on(&Term::stderr()).unwrap();
                        globalsettings.getwlltsettings().set_mnemonic_size(new_m_size.parse::<usize>().unwrap());
                        wllt_mngr_settings_callback(globalsettings, term, onchain).await
                    },
                1 => {
                        let new_n_wallets = Input::new().with_prompt("Enter new Number of Wallets").default("1".to_string()).show_default(false).interact_text_on(&Term::stderr()).unwrap() ;
                        globalsettings.getwlltsettings().set_n_wallets(new_n_wallets.parse::<usize>().unwrap());
                        wllt_mngr_settings_callback(globalsettings, term, onchain).await
                },
                2 => {
                        let new_n_child = Input::new().with_prompt("Enter new Number of Children").default("20".to_string()).show_default(false).interact_text_on(&Term::stderr()).unwrap() ;
                        globalsettings.getwlltsettings().set_n_child(new_n_child.parse::<usize>().unwrap());
                        wllt_mngr_settings_callback(globalsettings, term, onchain).await
                },
                3 => wllt_mngr_menu_callback(globalsettings, term, onchain).await,
                _ => Ok(println!("Error"))
            };
        },
        None => println!("User did not select anything")
    }
    Ok(())
}

pub fn confirm(term: &mut Term){
    println!("{}", style("Press ENTER to continue...").color256(235));
    Confirm::new().default(false).interact();
}

async fn wllt_mngr_menu_callback(globalsettings: &mut Settings, term: &mut Term, onchain: &mut OnChain) -> eyre::Result<()>{
    clear_screen(term);
    wallet_mngr_menu(globalsettings, term, onchain).await
}

#[async_recursion]
async fn wllt_mngr_settings_callback(globalsettings: &mut Settings, term: &mut Term, onchain: &mut OnChain) -> eyre::Result<()>{
    clear_screen(term);
    wllt_settings_printer(globalsettings);
    wllt_load_settings_printer(globalsettings);
    println!("");
    wallet_mngr_settings(globalsettings, term, onchain).await
}

async fn main_callback(globalsettings: &mut Settings, term: &mut Term, onchain: &mut OnChain) -> eyre::Result<()>{
    clear_screen(term);
    master_menu::master_menu(globalsettings, term, onchain).await
}

fn clear_screen(term: &mut Term){
    term.move_cursor_to(0,0);
    term.move_cursor_down(14);
    term.clear_to_end_of_screen();
}

fn w_size_matcher(w_size: usize){
    match w_size {
        12 |15 |18 |21 |24 => println!("{} {}","  Mnemonic size: ", style(w_size).yellow().bold()),
        _ => println!("{} {} {}","  Mnemonic size: ", style(w_size).red().bold(), style("(Invalid, default set)").red().bold())
    }
}

fn wllt_settings_printer(globalsettings: &mut Settings){
    let n_wallets = globalsettings.getwlltsettings().get_n_wallets();
    let n_child = globalsettings.getwlltsettings().get_n_child();
    let w_size = globalsettings.getwlltsettings().get_mnemonic_size();
    println!("{}", style("Wallet generator settings:").green());
    w_size_matcher(w_size);
    println!("{} {}","  Number of wallets to generate: ", style(n_wallets).yellow().bold());
    println!("{} {}","  Number of children per wallet to generate: ", style(n_child).yellow().bold());
}

fn wllt_load_settings_printer(globalsettings: &mut Settings){
    let is_loaded = globalsettings.getwlltsettings().get_is_loaded();
    let present_wllts = globalsettings.getwlltsettings().get_present_wllts();
    println!("{}", style("Wallets infos:").green());
    is_loaded_printer(is_loaded);
    println!("{} {}","  Identified wallets: ", style(present_wllts).yellow().bold());
}

fn clear_for_pb(term: &mut Term){
    term.move_cursor_up(4);
    term.clear_to_end_of_screen();
}

fn is_loaded_printer(is_loaded: bool){
    if is_loaded {
        println!("{} {}","  Wallets are loaded: ", style("YES").green().bold());
    }
    else {
        println!("{} {}","  Wallets are loaded: ", style("NO").red().bold());
    }
}

fn new_bar_obj(n : usize) -> ProgressBar {
    let bar = ProgressBar::new(n.try_into().unwrap());
    bar.set_style(ProgressStyle::with_template("[{elapsed_precise}] [{spinner:.green}] {bar:42.green/white}")
    .unwrap());
    bar
}
