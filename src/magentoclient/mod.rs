use crate::config::{MagentoConfiguration};
use serde::{Serialize, Deserialize};
use serde_json::{Value};
use reqwest;
use reqwest::header::{HeaderValue};
use crate::entities::attribute::{Attribute};
use crate::entities::category::{Category};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MagentoResponse {
    pub items: Vec<Value>,
    pub total_count: usize
}

pub trait SerializableMagentoObject {
    fn id(&self) -> String;
    fn value(&self) -> Value;
}

pub type MagentoError = Box<dyn std::error::Error>;

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

    pub fn fetch_attributes(&self) -> Result<Vec<Attribute>, MagentoError> {
        let client = self.get_client()?;

        let api_url = format!("{}/products/attributes?searchCriteria=", self.get_api_url());
        let body: MagentoResponse = client.get(api_url).send()?.json()?;

        let output = body.items.iter().map(|attribute| Attribute::new(attribute)).collect();

        Ok(output)
    }

    pub fn fetch_categories(&self) -> Result<Vec<Category>, MagentoError> {
        let client = self.get_client()?;

        let api_url = format!("{}/categories/list?searchCriteria", self.get_api_url());
        let body: MagentoResponse = client.get(api_url).send()?.json()?;

        let output = body.items.iter().map(|category| Category::from_value(category).unwrap()).collect();

        Ok(output)
    }
}
