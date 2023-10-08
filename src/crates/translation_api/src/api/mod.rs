mod baidu;
mod youdao;

pub use baidu::BaiduApi;
pub use youdao::YoudaoApi;

use crate::{error::Error, language::Language};

pub trait Translate {
    fn translate(
        &self,
        content: &str,
        src_lang: &Language,
        target_lang: &Language,
    ) -> Result<Box<dyn DisplayTranslation>, Error>;
}

pub trait DisplayTranslation: std::fmt::Debug + std::fmt::Display {}
