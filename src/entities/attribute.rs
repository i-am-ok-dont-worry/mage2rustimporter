use serde_json::{Value};
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

