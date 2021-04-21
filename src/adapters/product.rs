use crate::magentoclient::MagentoRestClient;
use crate::elasticsearch::ESClient;
use crate::adapters::ImportAdapter;
use crate::entities::product::ProductMapper;
use std::time::{Duration, Instant};
use log::{info};

pub struct ProductAdapter {
    client: MagentoRestClient,
    es: ESClient,
    page: i32,
    page_size: i32
}

impl ProductAdapter {
    pub fn new(client: MagentoRestClient, es: ESClient) -> Self {
        ProductAdapter { client: client, es, page: 0, page_size: 500 }
    }
}

impl ImportAdapter for ProductAdapter {
    fn run(&mut self, ids: Option<Vec<&str>>) {
        info!("Running import for page: {}. Page size: {}", self.page, self.page_size);
        let start = Instant::now();

        match ids.clone() {
            Some(selected_ids) => {
                match self.client.fetch_products_by_ids(selected_ids) {
                    Ok((res, finished)) => {
                        let product_mapper = ProductMapper::new(&self.client);
                        let products = product_mapper.map(res);
                        self.es.index("product", products.unwrap());

                        let elapsed = start.elapsed();
                        info!("Finished page {} in {:?}", self.page, elapsed);

                        if finished == false {
                            self.page = self.page + 1;
                            self.run(ids);
                        }
                    },
                    Err(err) => info!("Cannot fetch products {:?}", err)
                }
            }
            None => {
                match self.client.fetch_products(self.page_size, self.page) {
                    Ok((res, finished)) => {
                        let product_mapper = ProductMapper::new(&self.client);
                        let products = product_mapper.map(res);
                        self.es.index("product", products.unwrap());

                        let elapsed = start.elapsed();
                        info!("Finished page {} in {:?}", self.page, elapsed);

                        if finished == false {
                            self.page = self.page + 1;
                            self.run(ids);
                        }
                    },
                    Err(err) => info!("Cannot fetch products {:?}", err)
                }
            }
        }
    }
}
