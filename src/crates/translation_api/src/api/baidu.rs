use std::collections::HashMap;
use super::Translate;


pub struct Baidu {
    // TODO: Implement Baidu API
    url: String,
    app_id: String,
    secret: String,
}

impl Baidu {
    pub fn new(url: &str, app_id: &str, secret: &str) -> Self {
        Self { url: String::from(url), app_id: String::from(app_id), secret: String::from(secret) }
    }
}

impl Translate for Baidu {
    fn translate(&self, _text: &str) -> Result<std::collections::HashMap<String, String>, crate::error::Error> {
        // TODO: Implement Baidu API
        let translate_result = HashMap::new();

        Ok(translate_result)
    }
}
