use crate::config::AppConfiguration;
use crate::magentoclient::MagentoRestClient;
use log::{info};
use serde_json::to_string_pretty;
use crate::elasticsearch::ESClient;
use std::time::{Duration, Instant};
use std::rc::Rc;
use crate::entities::product::ProductMapper;
use crate::mageimpoter::worker::ImportWorker;
use std::thread;
use std::sync::{Arc, Mutex};
use clap::App;

mod worker;

pub struct Mage2Importer {
    config: AppConfiguration,
    magento_rest_client: MagentoRestClient,
    es: ESClient
}


impl Mage2Importer {
    pub fn new(config: &str) -> Self {
        let configuration = AppConfiguration::new(config);
        let mage_configuration = configuration.clone().magento;
        let rest_client = MagentoRestClient::new(mage_configuration);
        let es_client = ESClient::new(configuration.clone().elasticsearch);
        info!("Initialized Magento2 importer");

        Mage2Importer { config: configuration, magento_rest_client: rest_client, es: es_client }
    }

    pub fn run (&self, adapter_name: String, ids: Option<Vec<&str>>) {
        let mut worker = ImportWorker::new(self.magento_rest_client.clone(), self.es.clone());
        worker.start(adapter_name, ids);
    }
}
