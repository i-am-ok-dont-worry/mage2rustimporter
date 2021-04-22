use crate::config::{MagentoConfiguration};
use serde::{Serialize, Deserialize};
use serde_json::{Value};
use reqwest;
use reqwest::header::{HeaderValue};
use crate::entities::attribute::{Attribute};
use crate::entities::category::{Category};
use crate::entities::product::{Product, ProductMapper};
use elasticsearch::params::Conflicts::Proceed;
use log::{error, info};
use urlencoding::encode;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchCriteria {
    pub page_size: i32,
    pub current_page: i32
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MagentoResponse {
    pub items: Vec<Value>,
    pub total_count: i32,
    pub search_criteria: SearchCriteria
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MagentoSimpleResponse {
    pub items: Vec<Value>,
    pub total_count: i32
}

pub trait SerializableMagentoObject {
    fn id(&self) -> String;
    fn value(&self) -> Value;
}

pub type MagentoError = Box<dyn std::error::Error>;

#[derive(Clone)]
pub struct MagentoRestClient {
    pub config: MagentoConfiguration,
    pub api_version: String
}

impl MagentoRestClient {
    pub fn new(config: MagentoConfiguration) -> Self {
        MagentoRestClient { config: config.clone(), api_version: "V1".to_string() }
    }

    fn get_auth_header(&self) -> HeaderValue {
        HeaderValue::from_str(&format!("Bearer {}", self.config.accessToken.to_string())).unwrap()
    }

    fn get_api_url(&self) -> String {
        format!("{}/{}", self.config.url, self.api_version)
    }

    fn get_client(&self) -> Result<reqwest::blocking::Client, reqwest::Error> {
        let mut headers = reqwest::header::HeaderMap::new();
        let auth_header = self.get_auth_header();
        headers.insert("Authorization", self.get_auth_header());

        reqwest::blocking::Client::builder()
            .default_headers(headers)
            .build()
    }

    pub fn fetch_attributes(&self) -> Result<(Vec<Value>, bool), MagentoError> {
        let client = self.get_client()?;

        let api_url = format!("{}/products/attributes?searchCriteria=", self.get_api_url());
        let body: MagentoSimpleResponse = client.get(api_url).send()?.json()?;

        // let output = body.items.iter().map(|attribute| Attribute::new(attribute)).collect();

        Ok((body.items, true))
    }

    pub fn fetch_categories(&self) -> Result<(Vec<Value>, bool), MagentoError> {
        let client = self.get_client()?;

        let api_url = format!("{}/categories/list?searchCriteria", self.get_api_url());
        let body: MagentoSimpleResponse = client.get(api_url).send()?.json()?;

        Ok((body.items, true))
    }

    pub fn fetch_categories_by_ids(&self, ids: Vec<&str>) -> Result<(Vec<Value>, bool), MagentoError> {
        let client = self.get_client()?;

        let joined_ids = ids.join(",");
        let api_url = format!("{}/categories/list?searchCriteria[filter_groups][0][filters][0][field]=entity_id&searchCriteria[filter_groups][0][filters][0][value]={}&searchCriteria[filter_groups][0][filters][0][condition_type]=in", self.get_api_url(), joined_ids);
        let body: MagentoSimpleResponse = client.get(api_url).send()?.json()?;

        info!("Importing categories: {:?}", ids);

        Ok((body.items, true))
    }

    pub fn fetch_products(&self, page_size: i32, page: i32) -> Result<(Vec<Value>, bool), MagentoError> {
        let client = self.get_client()?;

        let api_url = format!("{}/products?searchCriteria%5BpageSize%5D={}&searchCriteria%5BcurrentPage%5D={}", self.get_api_url(), page_size, page);
        let body: MagentoResponse = client.get(api_url).send()?.json()?;

        let finished = (page * page_size) > body.total_count;

        Ok((body.items, finished))
    }

    pub fn fetch_products_by_ids(&self, ids: Vec<&str>) -> Result<(Vec<Value>, bool), MagentoError> {
        let client = self.get_client()?;

        let joined_ids = ids.join(",");
        let api_url = format!("{}/products?searchCriteria[filter_groups][0][filters][0][field]=entity_id&searchCriteria[filter_groups][0][filters][0][value]={}&searchCriteria[filter_groups][0][filters][0][condition_type]=in", self.get_api_url(), joined_ids);
        let body: MagentoSimpleResponse = client.get(api_url).send()?.json()?;

        info!("Importing products: {:?}", ids);

        Ok((body.items, true))
    }

    pub fn fetch_configurable_products(&self, sku: String) -> Result<Vec<Value>, MagentoError> {
        let client = self.get_client()?;

        let encoded_sku = encode(&sku);
        let api_url = format!("{}/configurable-products/{}/children", self.get_api_url(), encoded_sku);
        let resp = client.get(api_url).send()?;

        if resp.status().is_success() == false {
            let status = resp.status();
            let result = resp.text()?;
            error!("Cannot fetch configurable options. Status {:?}. Skus: {}", status, sku);
            error!("{:?}", result);

            Ok(Vec::new())
        } else {
            let body: Vec<Value> = resp.json()?;
            Ok(body)
        }
    }
}
