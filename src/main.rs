extern crate logger;
use crate::mageimpoter::Mage2Importer;
use logger::Logger;
use log::{info};

mod mageimpoter;
mod magentoclient;
mod config;
mod elasticsearch;
mod entities;
mod adapters;
use clap::App;

fn main() {
    // Initialize logger
    env_logger::init();

    let matches = App::new("MyApp")
        .arg("<adapter> 'Sets a adapter'")
        .arg("-i, --ids=[IDS] 'Sets a custom ids'")
        .arg("-c, --config=[CONFIG] 'Sets a config file location'")
        .get_matches();

    let adapter_name = matches.value_of("adapter").unwrap().to_string();
    let config_path = matches.value_of("config").unwrap_or("config.json").to_string();
    let ids = match matches.value_of("ids") {
        Some(ids) => Some(ids.split(",").collect()),
        None => None
    };

    let impoter = Mage2Importer::new(&config_path);
    impoter.run(adapter_name, ids);
}
