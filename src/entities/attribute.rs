use serde_json::{Value};
use log::{info};
use crate::magentoclient::SerializableMagentoObject;

pub struct Attribute {
    id: String,
    value: Value
}

impl Attribute {
    pub fn new(val: &Value) -> Self {
        let id = val["attribute_id"].to_string();
        Attribute { id, value: val.clone() }
    }
}

impl SerializableMagentoObject for Attribute {
    fn id(&self) -> String {
        self.id.to_owned()
    }
    fn value(&self) -> Value {
        self.value.to_owned()
    }
}


pub struct AttributeMapper {}

impl AttributeMapper {

    pub fn new() -> Self {
        AttributeMapper {}
    }

    fn map_single(&self, input: &Value) -> Result<Attribute, serde_json::error::Error> {
        Ok(Attribute::new(input))
    }

    pub fn map(&self, attributes: Vec<Value>) -> Result<Vec<Attribute>, serde_json::error::Error> {
        let output = attributes.iter().map(|attribute| {
            match self.map_single(attribute) {
                Ok(p) => p,
                Err(err) => {
                    info!("Cannot parse attribute object {:?}", err);
                    Attribute { id: "".to_string(), value: Value::Null }
                }
            }
        })
            .filter(|val| val.value().is_null() == false)
            .collect::<Vec<Attribute>>();

        Ok(output)
    }
}
