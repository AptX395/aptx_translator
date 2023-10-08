use md5::{Digest, Md5};
use reqwest::{blocking::Client, Error as ReqwestErr};
use serde::{Deserialize, Serialize};
use crate::{Api, error::{Error, ErrorCode}, language::Language};
use super::{Translate, DisplayTranslation};

#[derive(Debug)]
#[derive(Deserialize)]
pub struct BaiduApi {
    url: String,
    app_id: String,
    secret: String,
}

impl BaiduApi {
    fn request(&self, params: BaiduParams) -> Result<String, ReqwestErr> {
        let response_text = Client::new()
            .post(&self.url)
            .form(&params)
            .send()?
            .text()?;

        Ok(response_text)
    }
}

impl Translate for BaiduApi {
    fn translate(
        &self,
        content: &str,
        src_lang: &Language,
        target_lang: &Language,
    ) -> Result<Box<dyn DisplayTranslation>, Error> {
        let params = BaiduParams::new(content, src_lang, target_lang, &self.app_id, &self.secret);
        let request_result = self.request(params);
        
        let Ok(response_text) = request_result else {
            let err_str = request_result.unwrap_err().to_string();
            let request_err = Error::new(Api::Baidu, ErrorCode::RequestError, &err_str);
            return Err(request_err);
        };

        let deserialize_result: Result<BaiduResponse, serde_json::Error> = serde_json::from_str(&response_text);

        let Ok(response) = deserialize_result else {
            let err_str = deserialize_result.unwrap_err().to_string();
            let api_err = Error::new(Api::Baidu, ErrorCode::DeserializeError, &err_str);
            return Err(api_err);
        };

        Ok(Box::new(response))
    }
}

#[derive(Serialize)]
struct BaiduParams {
    q: String,
    from: String,
    to: String,
    #[serde(rename(serialize = "appid"))]
    app_id: String,
    salt: String,
    sign: String,
}

impl BaiduParams {
    pub fn new(q: &str, from: &Language, to: &Language, app_id: &str, secret: &str) -> Self {
        let salt = rand::random::<i32>().to_string();
        let sign_str = format!("{}{}{}{}", app_id, q, salt, secret);
        let mut hasher = Md5::new();
        hasher.update(sign_str.as_bytes());
        let sign = format!("{:x}", hasher.finalize());

        Self {
            q: String::from(q),
            from: from.to_baidu_param(),
            to: to.to_baidu_param(),
            app_id: String::from(app_id),
            salt: String::from(salt),
            sign: String::from(sign),
        }
    }
}

#[derive(Debug)]
#[derive(Deserialize)]
pub struct BaiduResponse {
    from: String,
    to: String,
    trans_result: Vec<BaiduTransResult>,
}

impl std::fmt::Display for BaiduResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let translation_str: String = self.trans_result.iter()
            .map(|x| {
                format!("{}\n", x.dst)
            })
            .collect();

        write!(f, "{}", translation_str)
    }
}

impl DisplayTranslation for BaiduResponse {}

#[derive(Debug)]
#[derive(Deserialize)]
pub struct BaiduTransResult {
    src: String,
    dst: String,
}
