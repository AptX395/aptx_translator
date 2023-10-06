use std::collections::HashMap;
use md5::{Digest, Md5};
use reqwest::{blocking::Client, Error as ReqwestErr};
use serde::Deserialize;
use crate::{error::Error, language::Language, Api};
use super::{Translate, Translation};

#[derive(Debug)]
#[derive(Deserialize)]
pub struct BaiduApi {
    url: String,
    app_id: String,
    secret: String,
}

impl BaiduApi {
    fn generate_params(&self, q: &str, from: &str, to: &str) -> HashMap<&str, String> {
        let salt = rand::random::<i32>()
            .to_string();

        let sign_str = format!("{}{}{}{}", self.app_id, q, salt, self.secret);
        let mut hasher = Md5::new();
        hasher.update(sign_str.as_bytes());
        let sign = format!("{:x}", hasher.finalize());

        let mut param = HashMap::new();
        param.insert("q", q.to_string());
        param.insert("from", from.to_string());
        param.insert("to", to.to_string());
        param.insert("appid", self.app_id.to_string());
        param.insert("salt", salt);
        param.insert("sign", sign);

        param
    }

    fn request_translation(&self, param: HashMap<&str, String>) -> Result<Box<dyn Translation>, Error> {
        let request_result = self.request(param);
        
        let Ok(response_text) = request_result else {
            let send_err = Error::new(Api::Baidu, 1, request_result.unwrap_err().to_string());
            return Err(send_err);
        };

        let deserialize_result = serde_json::from_str::<BaiduTranslation>(&response_text);

        let Ok(translation) = deserialize_result else {
            let api_err = Error::new(Api::Baidu, 2, response_text);
            return Err(api_err);
        };

        Ok(Box::new(translation))
    }

    fn request(&self, param: HashMap<&str, String>) -> Result<String, ReqwestErr> {
        let response_text = Client::new()
            .post(self.url.clone())
            .form(&param)
            .send()?
            .text()?;

        Ok(response_text)
    }
}

impl Translate for BaiduApi {
    fn translate(&self, text: &str, src_lang: Language, target_lang: Language) -> Result<Box<dyn Translation>, Error> {
        let param = self.generate_params(text, &src_lang.to_baidu_param(), &target_lang.to_baidu_param());
        
        self.request_translation(param)
    }
}

#[derive(Debug)]
#[derive(Deserialize)]
pub struct BaiduTranslation {
    from: String,
    to: String,
    trans_result: Vec<TransResult>,
}

impl Translation for BaiduTranslation {
    fn text(&self) -> String {
        self.trans_result.iter()
            .map(|x| {
                format!("{}\n", x.dst)
            })
            .collect::<String>()
    }
}

#[derive(Debug)]
#[derive(Deserialize)]
pub struct TransResult {
    src: String,
    dst: String,
}
