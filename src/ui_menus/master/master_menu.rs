use console::{Term,};
use dialoguer::{theme::ColorfulTheme, Select};
use on_chain::{OnChain};
use async_recursion::async_recursion;

use crate::Environment;

use crate::{ui_menus::{wlltmngr::wllt_mngr_menu,
                       connect::reconnect},
            Settings};

#[async_recursion]
pub async fn master_menu(globalsettings: &mut Settings, term: &mut Term, onchain: &mut OnChain) -> eyre::Result<()> {
    let items = vec!["▶ Wallet Manager\n", "▶ Connect\n", "✖ Quit"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an option you want to use: ")
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())?;

    match selection {
        Some(index) =>  {
            match index {
                0 => wllt_mngr_menu::wallet_mngr_menu(globalsettings, term, onchain).await,
                1 => {  let new_onchain = reconnect::reconnect(globalsettings, term, onchain).await;
                        match new_onchain {
                            Ok(mut insider) => master_menu(globalsettings, term, &mut insider).await,
                            Err(_) => master_menu(globalsettings, term, onchain).await
                        };
                        Ok(())
                      }
                2 => Ok(println!("Quit")), // Quit logic
                _ => Ok(println!("Error"))
            };
        },
        None => println!("User did not select anything")
    };

    Ok(())
}

