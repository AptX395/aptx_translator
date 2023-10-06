use serde::Deserialize;
use crate::{error::Error, language::Language};
use super::{Translate, Translation};

#[derive(Deserialize)]
pub struct YoudaoApi {
    url: String,
    app_key: String,
    app_secret: String,
    // TODO: Implement Youdao API
}

impl YoudaoApi {}

impl Translate for YoudaoApi {
    fn translate(&self, text: &str, src_lang: Language, target_lang: Language) -> Result<Box<dyn Translation>, Error> {
        // TODO: Implement Youdao API

        Ok(Box::new(YoudaoTranslation {}))
    }
}

#[derive(Debug)]
#[derive(Deserialize)]
pub struct YoudaoTranslation {}

impl Translation for YoudaoTranslation {
    fn text(&self) -> String {
        // TODO: Implement Youdao API

        String::new()
    }
}
