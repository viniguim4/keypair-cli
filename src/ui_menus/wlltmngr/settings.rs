use std::fs;
use tokio::runtime::Handle;

#[derive(Debug, Clone)]
pub struct WlltmngrSettings {
    n_wallets: usize,       
    mnemonic_size: usize,   
    n_child: usize,         
    exists: bool,
    present_wllts: usize,
    is_loaded: bool,
    wallets: Vec<wllt_gen::JsonWallet>
}

impl WlltmngrSettings {
    
    pub fn default() -> Self {
        let counter = count_wallets();
        WlltmngrSettings { n_wallets: 10,
                           mnemonic_size: 12,
                           n_child: 0,
                           exists: false, 
                           present_wllts: counter, 
                           is_loaded: false, 
                           wallets: Vec::new() }  
    }

    pub fn set_n_wallets(&mut self, n_wallets: usize){
        self.n_wallets = n_wallets;
    }

    pub fn set_mnemonic_size(&mut self, mnemonic_size: usize) {
        self.mnemonic_size = mnemonic_size;
    }

    pub fn set_n_child(&mut self, n_child: usize) {
        self.n_child = n_child;
    }

    pub fn set_is_loaded(&mut self, is_loaded: bool) {
        self.is_loaded = is_loaded;
    }

    pub fn get_n_wallets(&self) -> usize {
        self.n_wallets
    }

    pub fn get_mnemonic_size(&self) -> usize {
        self.mnemonic_size
    }

    pub fn get_n_child(&self) -> usize {
        self.n_child
    }

    pub fn get_exists(&self) -> bool {
        self.exists
    }

    pub fn get_is_loaded(&self) -> bool {
        self.is_loaded
    }

    pub fn get_present_wllts(&self) -> usize {
        self.present_wllts
    }

    pub fn get_wallets(&self) -> &Vec<wllt_gen::JsonWallet> {
        &self.wallets
    }

    pub fn clear_wallets_vec(&mut self) {
        self.wallets.clear();
    }

    pub fn  append_wallets_vec(&mut self, wallet: wllt_gen::JsonWallet) {
        self.wallets.push(wallet);
    }

    pub fn update(&mut self) {
        self.exists = exists();
        self.present_wllts = count_wallets();
    }

    pub fn load(&mut self) -> bool {
        // load wllts logic
        false
    }
}

pub fn exists() -> bool {
    let file_path = "genwallets/wallets0.json";
    match fs::metadata(file_path) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn count_wallets() -> usize {
    let path = "genwallets/";
    let iterator = fs::read_dir(path).unwrap();
    iterator.count()-1
}