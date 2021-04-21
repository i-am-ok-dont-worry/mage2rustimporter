use crate::magentoclient::MagentoRestClient;
use crate::elasticsearch::ESClient;
use crate::adapters::ImportAdapter;
use crate::entities::product::ProductMapper;
use std::time::{Duration, Instant};
use log::{info};
use crate::entities::category::CategoryMapper;
use crate::entities::attribute::AttributeMapper;

pub struct AttributeAdapter {
    client: MagentoRestClient,
    es: ESClient,
    page: i32,
    page_size: i32
}

impl AttributeAdapter {
    pub fn new(client: MagentoRestClient, es: ESClient) -> Self {
        AttributeAdapter { client: client, es, page: 0, page_size: 500 }
    }
}

impl ImportAdapter for AttributeAdapter {
    fn run(&mut self, ids: Option<Vec<&str>>) {
        info!("Running import for attributes");
        let start = Instant::now();

        match self.client.fetch_attributes() {
            Ok((res, finished)) => {
                let attributes_mapper = AttributeMapper::new();
                let attributes = attributes_mapper.map(res);
                self.es.index("attribute", attributes.unwrap());

                let elapsed = start.elapsed();
                info!("Finished page {} in {:?}", self.page, elapsed);

                if finished == false {
                    self.page = self.page + 1;
                    self.run(ids);
                }
            },
            Err(err) => info!("Cannot fetch attributes {:?}", err)
        }
    }
}
