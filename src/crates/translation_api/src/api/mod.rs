mod baidu;
mod youdao;

pub use baidu::Baidu;
pub use youdao::Youdao;

use std::collections::HashMap;
use crate::error::Error;

pub trait Translate {
    // TODO: The abstract of translating text via the translation API
    fn translate(&self, text: &str) -> Result<HashMap<String, String>, Error>;
}
