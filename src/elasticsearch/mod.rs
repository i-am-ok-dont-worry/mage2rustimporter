use elasticsearch::{Elasticsearch, Error, SearchParts, IndexParts, BulkParts};
use elasticsearch::http::request::JsonBody;
use serde_json::{json, Value};
use std::process;
use log::{error, info};
use serde::{Serialize, Deserialize};
use crate::magentoclient::SerializableMagentoObject;
use std::time::{Instant, Duration};
use std::rc::Rc;

pub struct ESClient {
    client: Elasticsearch
}

impl ESClient {
    pub fn new() -> ESClient {
        ESClient { client: Elasticsearch::default() }
    }

    #[tokio::main]
    pub async fn test (&self) -> Result<(), Box<dyn std::error::Error>> {
        /*let search_response = self.client
            .search(SearchParts::None)
            .body(json!({
                "query": {
                    "match_all": {}
                }
            }))
            .allow_no_indices(true)
            .send()
            .await
            .unwrap_or_else(|e| {
                println!("Error while connecting to Elasticsearch. Check your connection on localhost:9200");
                process::exit(1);
            });*/
        let search_response = self.client
            .index(IndexParts::IndexId("tweets", "1"))
            .body(json!({
                "id": 1,
                "user": "kimchy",
                "post_date": "2009-11-15T00:00:00Z",
                "message": "Trying out Elasticsearch, so far so good?"
            }))
            .send()
            .await?;

        // get the HTTP response status code
        let status_code = search_response.status_code();

        // read the response body. Consumes search_response
        let response_body = search_response.json::<Value>().await?;

        let failed_shards = response_body["_shards"]["failed"].as_i64().unwrap();
        println!("Status code: {}", status_code);
        println!("Failed shards: {}", failed_shards);

        Ok(())
    }

    #[tokio::main]
    pub async fn index<T>(&self, index: &str, docs: Vec<T>, time: Rc<Instant>) -> Result<(), Box<dyn std::error::Error>> where T: SerializableMagentoObject {
        let mut body: Vec<JsonBody<_>> = Vec::with_capacity(docs.len());
        for doc in docs.iter() {
            let id = doc.id();
            let value = doc.value();
            body.push(json!({"index": {"_id": id }}).into());
            body.push(json!(&value).into());
        }

        let response = self.client
            .bulk(BulkParts::Index(index))
            .body(body)
            .send()
            .await?;

        let successful = response.status_code().is_success();
        let response_body = response.json::<Value>().await?;
        let elapsed = time.elapsed();

        if successful {
            info!("Successfully indexed {:?} documents in {:?}", docs.len(), elapsed);
        } else {
            error!("Bulk operation failed while saving {:?} entity", index);
        }

        Ok(())
    }
}
