use serde_json::{Map, Value};
use serde_json::error::Error;
use log::{info};
use crate::magentoclient::SerializableMagentoObject;

pub struct Category {
    id: String,
    value: Value
}

impl SerializableMagentoObject for Category {
    fn id(&self) -> String {
        self.id.to_owned()
    }
    fn value(&self) -> Value {
        self.value.to_owned()
    }
}


impl Category {
    pub fn new(value: &Value) -> Self {
        let id = value["id"].to_string();
        Category { id, value: value.clone() }
    }

    pub fn from_value(value: &Value) -> Result<Self, serde_json::error::Error> {
        let id = value["id"].to_string();
        let mut output = Map::new();

        let category = serde_json::from_value::<Map<String, Value>>(value.to_owned())?;

        for (key, value) in category.iter() {
            if key != "custom_attributes" {
                output.insert(key.to_string(), value.to_owned());
            }
        };

        let custom_attributes = serde_json::from_value::<Vec<Value>>(value["custom_attributes"].to_owned())?;
        let mut map = Map::new();

        for item in custom_attributes.iter() {
            let attribute_code = item["attribute_code"].as_str().unwrap().to_string();
            let value = item["value"].to_owned();
            map.insert(attribute_code, value);
        }

        output.extend(map);

        let val = serde_json::to_value(output).unwrap();
        Ok(Category { id, value: val })
    }
}

pub struct CategoryMapper {}

impl CategoryMapper {

    pub fn new() -> Self {
        CategoryMapper {}
    }

    fn map_single(&self, input: &Value) -> Result<Category, serde_json::error::Error> {
        Category::from_value(input)
    }

    pub fn map(&self, categories: Vec<Value>) -> Result<Vec<Category>, serde_json::error::Error> {
        let output = categories.iter().map(|category| {
            match self.map_single(category) {
                Ok(p) => p,
                Err(err) => {
                    info!("Cannot parse category object {:?}", err);
                    Category { id: "".to_string(), value: Value::Null }
                }
            }
        })
            .filter(|val| val.value().is_null() == false)
            .collect::<Vec<Category>>();

        Ok(output)
    }
}
