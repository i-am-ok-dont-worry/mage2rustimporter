use crate::magentoclient::MagentoRestClient;
use crate::elasticsearch::ESClient;
use crate::adapters::ImportAdapter;
use crate::entities::product::ProductMapper;
use std::time::{Duration, Instant};
use log::{info};
use crate::entities::category::CategoryMapper;

pub struct CategoryAdapter {
    client: MagentoRestClient,
    es: ESClient,
    page: i32,
    page_size: i32
}

impl CategoryAdapter {
    pub fn new(client: MagentoRestClient, es: ESClient) -> Self {
        CategoryAdapter { client: client, es, page: 0, page_size: 500 }
    }
}

impl ImportAdapter for CategoryAdapter {
    fn run(&mut self, ids: Option<Vec<&str>>) {
        info!("Running import for categories");
        let start = Instant::now();

        match ids.clone() {
            Some(selected_ids) => {
                match self.client.fetch_categories_by_ids(selected_ids) {
                    Ok((res, finished)) => {
                        let category_mapper = CategoryMapper::new();
                        let categories = category_mapper.map(res);
                        self.es.index("category", categories.unwrap());

                        let elapsed = start.elapsed();
                        info!("Finished page {} in {:?}", self.page, elapsed);

                        if finished == false {
                            self.page = self.page + 1;
                            self.run(ids);
                        }
                    },
                    Err(err) => info!("Cannot fetch categories {:?}", err)
                }
            },
            None => {
                match self.client.fetch_categories() {
                    Ok((res, finished)) => {
                        let category_mapper = CategoryMapper::new();
                        let categories = category_mapper.map(res);
                        self.es.index("category", categories.unwrap());

                        let elapsed = start.elapsed();
                        info!("Finished page {} in {:?}", self.page, elapsed);

                        if finished == false {
                            self.page = self.page + 1;
                            self.run(ids);
                        }
                    },
                    Err(err) => info!("Cannot fetch categories {:?}", err)
                }
            }
        }
    }
}
