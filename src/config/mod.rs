use serde::{Serialize, Deserialize};
use std::fs::File;
use std::fs;
use std::process;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ElasticsearchConfiguration {
    pub url: String,
    pub index: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MagentoConfiguration {
    pub url: String,
    pub consumerKey: String,
    pub consumerSecret: String,
    pub accessToken: String,
    pub accessTokenSecret: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfiguration {
    pub version: String,
    pub redis: HashMap<String, String>,
    pub elasticsearch: ElasticsearchConfiguration,
    pub magento: MagentoConfiguration
}

impl AppConfiguration {
    pub fn new(filepath: &str) -> AppConfiguration {
        let mut file = File::open(filepath)
            .unwrap_or_else(|err| {
                println!("Error while reading config. Create /config.json file");
                process::exit(1);
            });

        let content: AppConfiguration = serde_json::from_reader(file).unwrap();
        println!("Initialized application: {:?}", content);

        content
    }
}
