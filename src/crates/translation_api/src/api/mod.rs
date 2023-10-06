mod baidu;
mod youdao;

pub use baidu::BaiduApi;
pub use youdao::YoudaoApi;

use std::fmt::Debug;
use crate::{error::Error, language::Language};

pub trait Translate {
    fn translate(&self, text: &str, src_lang: Language, target_lang: Language) -> Result<Box<dyn Translation>, Error>;
}

pub trait Translation: Debug {
    fn text(&self) -> String;
}
