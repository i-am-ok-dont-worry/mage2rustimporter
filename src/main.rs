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

fn main() {
    // Initialize logger
    env_logger::init();

    let impoter = Mage2Importer::new("config.json");
    impoter.run();
}
