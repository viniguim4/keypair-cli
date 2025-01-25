mod wallet {pub mod entropy;
            pub mod mnemonic;
            pub mod hmac_sha512;
            pub mod derivation_path;
               pub mod ethereum{
                    pub mod eth_wallet;
                }
            }
use wallet::{mnemonic::{MnemonicStruct},
            entropy::{WordsSize, Entropy},
            hmac_sha512::{KeyPair},
            derivation_path::{DerivationPath, DeriveChild},
            ethereum::eth_wallet::{ETHWallet},
            };

use std::{thread,
          fs::OpenOptions, 
          io::{BufReader, BufWriter,}
        };
use serde::{Deserialize, Serialize};
use anyhow::{Result};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Child{
    pub index: u32,
    pub wallet: ETHWallet,
}

impl Child {
    pub fn new(index: u32, wallet: ETHWallet) -> Self {
        Child { index, wallet }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonWallet {
    mnemonic: String,
    size: usize,
    master_wallet: ETHWallet,
    childs_wallet: Vec<Child>,
}

impl JsonWallet {
    pub fn new(mnemonic: String, size: usize, master_wallet: ETHWallet, childs_wallet: Vec<Child>) -> Self {
        JsonWallet { mnemonic, size, master_wallet, childs_wallet }
    }

    pub fn save(&self, file_path: &str) -> Result<()> {
        let file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(file_path)?;
        let buf_writer = BufWriter::new(file);
        serde_json::to_writer_pretty(buf_writer, self)?;
        Ok(())
    }

    pub fn load(file_path: &str) -> Result<Self> {
        let file = OpenOptions::new()
            .read(true)
            .open(file_path)?;
        let buf_reader = BufReader::new(file);
        let json_wallet = serde_json::from_reader(buf_reader)?;
        Ok(json_wallet)
    }
}

fn eth_wallet(gen : Entropy) -> (DeriveChild, String, usize, ETHWallet) {
        // Generate entropy.
        let entropy = gen.gen_entropy();
    
        // Generate mnemonic and bip39 mnemonic/seed.
        let passwrd = "";
        let mnemonic = MnemonicStruct::new(&entropy, &passwrd);
    
        // Stores in keypair the generated private key and chain code from hmac-sha512 function.
        let keypair = KeyPair::new(&mnemonic.get_seed());
    
        //Derive to path m/44'/60'/0'/0 for aplly to wallet generator.
        let path = DerivationPath::new(DerivationPath::Ethereum).expect("Failed to create derivation path");
        let mut derivation = DeriveChild::new(keypair);
        let derivation2path = derivation.path_derivation(&path);    
    
        //Generate master wallet Keypair.
        let master_wallet = DeriveChild::wllt_derivation(&derivation2path, 0);
    
        //Parse the master wallet.
        let master_wallet = ETHWallet::new(&master_wallet.get_keypair().get_pk());
        (derivation2path.clone(), mnemonic.get_mnemonic().to_string(), mnemonic.mnemonic_len(), master_wallet)
}

fn eth_new_childs_thread(derivation2path: &DeriveChild, n: u32) -> Vec<Child> {
    let handles: Vec<_> = (1..=n)
        .map(|index| {
            let derivation2path = derivation2path.clone();
            thread::spawn(move || {
                let wallet = DeriveChild::wllt_derivation(&derivation2path, index);
                let wallet = ETHWallet::new(&wallet.get_keypair().get_pk());
                Child {index, wallet}
            })
        })
        .collect();
    
    handles.into_iter().map(|handle| handle.join().unwrap()).collect()
}

pub fn exec(n_wllt: usize, n_child: usize, w_size: usize) -> Result<()> {
    
    let (master2derive, mnemonic, msize, masterwllt) = match w_size{
        12 => eth_wallet(Entropy::new(WordsSize::W12)),
        15 => eth_wallet(Entropy::new(WordsSize::W15)),
        18 => eth_wallet(Entropy::new(WordsSize::W18)),
        21 => eth_wallet(Entropy::new(WordsSize::W21)),
        24 => eth_wallet(Entropy::new(WordsSize::W24)),
        _ => eth_wallet(Entropy::new(WordsSize::W12)),
    };
    let child_wallets = eth_new_childs_thread(&master2derive, n_child.try_into().unwrap());
    let json_wallet = JsonWallet::new(mnemonic, msize, masterwllt, child_wallets);
    let wallet_file_path = format!("genwallets/wallets{}.json", n_wllt);
    json_wallet.save(&wallet_file_path)?;

    Ok(())
}

pub fn exec_load(n_wllt: usize) -> Result<JsonWallet> {
    let wallet_file_path = format!("genwallets/wallets{}.json", n_wllt);
    let json_wallet = JsonWallet::load(&wallet_file_path)?;
    Ok(json_wallet)
}