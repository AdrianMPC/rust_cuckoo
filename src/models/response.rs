use serde_json::Value;

#[derive(Debug)]
pub struct Response {
    pub json: Value,
}

impl Response {
    pub fn new(json_str: &str) -> Self {
        Self {
            json: serde_json::from_str(json_str).unwrap_or_default(),
        }
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.json.get(key)
    }
}