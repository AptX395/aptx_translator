use std::collections::HashMap;
use super::Translate;

pub struct Youdao {
    // TODO: Implement Youdao API
    url: String,
    app_key: String,
    app_secret: String,
}

impl Youdao {
    pub fn new(url: &str, app_key: &str, app_secret: &str) -> Self {
        Self { url: String::from(url), app_key: String::from(app_key), app_secret: String::from(app_secret) }
    }
}

impl Translate for Youdao {
    fn translate(&self, _text: &str) -> Result<std::collections::HashMap<String, String>, crate::error::Error> {
        // TODO: Implement Youdao API
        let translate_result = HashMap::new();

        Ok(translate_result)
    }
}
