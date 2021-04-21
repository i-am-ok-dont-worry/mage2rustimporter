use crate::magentoclient::MagentoRestClient;
use crate::elasticsearch::ESClient;
use crate::entities::product::ProductMapper;
use std::time::{Duration, Instant};
use log::{info};
use std::sync::Mutex;
use crate::adapters::product::ProductAdapter;
use crate::adapters::ImportAdapter;
use crate::adapters::category::CategoryAdapter;
use crate::adapters::attributes::AttributeAdapter;

pub struct ImportWorker {
    magento_rest_client: MagentoRestClient,
    es: ESClient,
    page: i32,
    page_size: i32
}

impl ImportWorker {
    pub fn new(client: MagentoRestClient, es: ESClient) -> Self {
        ImportWorker { magento_rest_client: client, es, page_size: 500, page: 0 }
    }

    pub fn get_adapter(&self, adapter_name: &str) -> Option<Box<ImportAdapter>> {
        match adapter_name {
            "product" => {
                let mut product_adapter = ProductAdapter::new(self.magento_rest_client.clone(), self.es.clone());
                Some(Box::new(product_adapter))
            },
            "category" => {
                let mut category_adapter = CategoryAdapter::new(self.magento_rest_client.clone(), self.es.clone());
                Some(Box::new(category_adapter))
            },
            "attribute" => {
                let mut attribute_adapter = AttributeAdapter::new(self.magento_rest_client.clone(), self.es.clone());
                Some(Box::new(attribute_adapter))
            }
            _ => None
        }
    }

    pub fn start(&mut self, adapter_name: String, ids: Option<Vec<&str>>) {
        let start = Instant::now();

        match self.get_adapter(&adapter_name) {
            Some(mut adapter) => adapter.run(ids),
            None => {
                info!("Adapter {} not found. Terminating", "df");
                std::process::exit(0);
            }
        }

        let elapsed = start.elapsed();

        info!("Finished import task in {:?}", elapsed);
    }
}
