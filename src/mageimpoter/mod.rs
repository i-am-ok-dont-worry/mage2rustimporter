use crate::config::AppConfiguration;
use crate::magentoclient::MagentoRestClient;
use log::{info};
use serde_json::to_string_pretty;
use crate::elasticsearch::ESClient;
use std::time::{Duration, Instant};
use std::rc::Rc;

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
        let es_client = ESClient::new();
        info!("Initialized Magento2 importer");

        Mage2Importer { config: configuration, magento_rest_client: rest_client, es: es_client }
    }

    pub fn run (&self) {
        let start = Instant::now();
        match self.magento_rest_client.fetch_categories() {
            Ok(res) => {
                self.es.index("category", res, Rc::new(start));
            },
            Err(err) => info!("Cannot fetch attributes {:?}", err)
        }

    }
}
