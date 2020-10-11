use serde::{Deserialize, Serialize};
use std::fs::File;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Conf {
    pub nickname: String,
    pub region: String,
    pub url: String,
    pub capacity: u64,
    pub unit_price: u64,

    pub meta_path: String,
    pub ipfs_url: String,
    pub chain_url: String,
}

pub fn load_conf(fpath: &str) -> Conf {
    let f = File::open(fpath).expect("load config file failed");
    serde_yaml::from_reader::<File, Conf>(f).expect("parse config file failed")
}
