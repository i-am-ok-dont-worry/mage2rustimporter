use elasticsearch::{Elasticsearch, Error, SearchParts, IndexParts, BulkParts};
use elasticsearch::http::request::JsonBody;
use serde_json::{json, Value};
use std::process;
use log::{error, info};
use serde::{Serialize, Deserialize};
use crate::magentoclient::SerializableMagentoObject;
use std::time::{Instant, Duration};
use std::rc::Rc;
use crate::config::ElasticsearchConfiguration;

#[derive(Clone)]
pub struct ESClient {
    client: Elasticsearch,
    config: ElasticsearchConfiguration
}

impl ESClient {
    pub fn new(config: ElasticsearchConfiguration) -> ESClient {
        ESClient { client: Elasticsearch::default(), config }
    }


    #[tokio::main]
    pub async fn index<T>(&self, index: &str, docs: Vec<T>) -> Result<(), Box<dyn std::error::Error>> where T: SerializableMagentoObject {
        let mut body: Vec<JsonBody<_>> = Vec::with_capacity(docs.len());
        for doc in docs.iter() {
            let id = doc.id();
            let value = doc.value();
            body.push(json!({"index": {"_id": id }}).into());
            body.push(json!(&value).into());
        }

        let response = self.client
            .bulk(BulkParts::Index(&format!("{}_{}", self.config.index, index)))
            .body(body)
            .send()
            .await?;

        let successful = response.status_code().is_success();
        let response_body = response.json::<Value>().await?;

        if successful {
            info!("Successfully indexed {:?} documents", docs.len());
        } else {
            error!("Bulk operation failed while saving {:?} entity", index);
        }

        Ok(())
    }
}
