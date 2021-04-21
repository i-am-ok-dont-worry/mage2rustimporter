use serde_json::{Map, Value, json};
use serde_json::error::Error;
use log::{info};
use crate::magentoclient::{SerializableMagentoObject, MagentoRestClient};

#[derive(Clone)]
pub struct Product {
    id: String,
    value: Value
}

impl SerializableMagentoObject for Product {
    fn id(&self) -> String {
        self.id.to_owned()
    }
    fn value(&self) -> Value {
        self.value.to_owned()
    }
}

impl Product {
    pub fn new(value: &Value) -> Self {
        let id = value["id"].to_string();
        Product { id, value: value.clone() }
    }

    pub fn from_value(value: &Value) -> Result<Self, serde_json::error::Error> {
        let id = value["id"].to_string();
        let mut output = Map::new();

        let product = serde_json::from_value::<Map<String, Value>>(value.to_owned())?;

        for (key, value) in product.iter() {
            if key != "custom_attributes" {
                output.insert(key.to_string(), value.to_owned());
            }
        };


        // Process attributes
        let custom_attributes = serde_json::from_value::<Vec<Value>>(value["custom_attributes"].to_owned())?;
        let mut map = Map::new();

        for item in custom_attributes.iter() {
            let attribute_code = item["attribute_code"].as_str().unwrap().to_string();
            let value = item["value"].to_owned();
            map.insert(attribute_code, value);
        }

        output.extend(map);

        // Process media gallery
        let media_gallery_entries: Vec<Value> = serde_json::from_value(value["media_gallery_entries"].to_owned())?;
        let mut media_gallery: Vec<Value> = Vec::with_capacity(media_gallery_entries.len());

        for media in media_gallery_entries.iter() {
            media_gallery.push(json!({
                "image": media["file"].as_str().unwrap_or(""),
                "pos": media["position"].as_i64().unwrap_or(0),
                "typ": media["media_type"].as_str().unwrap_or(""),
                "lab": media["label"].as_str().unwrap_or(""),
            }).into())
        }

        let media_gallery_json = serde_json::to_value(media_gallery).unwrap();
        output.remove(&"media_gallery_entries".to_string());
        output.insert("media_gallery".to_string(), media_gallery_json);

        let val = serde_json::to_value(output).unwrap();
        Ok(Product { id, value: val })
    }
}

pub struct ProductMapper {
    client: MagentoRestClient
}

impl ProductMapper {
    pub fn new(client: &MagentoRestClient) -> Self {
        ProductMapper { client: client.clone() }
    }

    fn map_single(&self, input: &Value) -> Result<Product, serde_json::error::Error> {
        let product = Product::from_value(input).unwrap();
        let id = product.id();
        let type_id = product.value["type_id"].as_str().unwrap();
        let sku = product.value["sku"].as_str().unwrap();
        let is_configurable = type_id == "configurable";
        let mut output_product = serde_json::from_value::<Map<String, Value>>(product.value.to_owned()).unwrap();

        if is_configurable {
            match self.client.fetch_configurable_products(sku.to_string()) {
                Ok(configurable_products) => {
                    let configurable_products = self.client.fetch_configurable_products(sku.to_string()).unwrap();
                    let mut output: Vec<Value> = Vec::with_capacity(configurable_products.len());

                    for c in configurable_products.iter() {
                        let configurable: Value = json!({
                            "id": c["id"],
                            "sku": c["sku"],
                            "status": c["status"],
                            "visibility": c["visibility"],
                            "name": c["name"],
                            "price": c["price"],
                            "tier_prices": c["tier_prices"],
                            "special_price": c["special_price"],
                            "stock": c["stock"]
                        }).into();

                        output.push(configurable);
                    }

                    let configurable_list = serde_json::to_value(output).unwrap();
                    output_product.insert("configurable_children".to_string(), configurable_list);
                },
                Err(err) => {
                    output_product.insert("configurable_children".to_string(), Value::Array(Vec::new()));
                }
            }
        }

        let val = serde_json::to_value(output_product).unwrap();
        Ok(Product { id, value: val })
    }

    pub fn map(&self, products: Vec<Value>) -> Result<Vec<Product>, serde_json::error::Error> {
        let output = products.iter().map(|product| {
            match self.map_single(product) {
                Ok(p) => p,
                Err(err) => {
                    info!("Cannot parse product object {:?}", err);
                    Product { id: "".to_string(), value: Value::Null }
                }
            }
        })
        .filter(|val| val.value.is_null() == false)
        .collect::<Vec<Product>>();

        Ok(output)
    }
}
