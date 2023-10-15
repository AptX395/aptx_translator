use md5::{Digest, Md5};
use reqwest::{blocking::Client, Error as ReqwestErr};
use serde::{Deserialize, Serialize};

use crate::{Api, error::{DESERIALIZE_RESPONSE_ERR_MSG, Error, ErrCode}, language::Language};
use super::{DisplayTranslation, Translate};

#[derive(Debug, Deserialize)]
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

    fn parse_response(&self, response: &str) -> Result<Box<dyn DisplayTranslation>, Error> {
        let translation_response_result: Result<BaiduTranslationResponse, serde_json::Error> =
            serde_json::from_str(response);

        if let Ok(translation_response) = translation_response_result {
            return Ok(Box::new(translation_response));
        }
        
        let err_response_result: Result<BaiduErrResponse, serde_json::Error> =
            serde_json::from_str(response);

        if let Ok(err_response) = err_response_result {
            let api_err = Error::new(
                Api::Baidu,
                ErrCode::ApiError,
                &err_response.error_msg,
            );

            return Err(api_err);
        }

        let deserialize_err = Error::new(
            Api::Baidu,
            ErrCode::DeserializeError,
            DESERIALIZE_RESPONSE_ERR_MSG,
        );

        Err(deserialize_err)
    }
}

impl Translate for BaiduApi {
    fn translate(
        &self,
        text: &str,
        src_lang: &Language,
        target_lang: &Language,
    ) -> Result<Box<dyn DisplayTranslation>, Error> {
        let params = BaiduParams::new(
            text,
            src_lang,
            target_lang,
            &self.app_id,
            &self.secret,
        );

        let request_result = self.request(params);
        
        let Ok(response_text) = request_result else {
            let request_err = Error::new(
                Api::Baidu,
                ErrCode::RequestError,
                &request_result.unwrap_err().to_string(),
            );

            return Err(request_err);
        };

        self.parse_response(&response_text)
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
    pub fn new(
        q: &str,
        from: &Language,
        to: &Language,
        app_id: &str,
        secret: &str,
    ) -> Self {
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

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct BaiduTranslationResponse {
    from: String,
    to: String,
    trans_result: Vec<BaiduTransResult>,
}

impl std::fmt::Display for BaiduTranslationResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let translation_str: String = self.trans_result.iter()
            .map(|x| {
                format!("{}\n", x.dst)
            })
            .collect();

        write!(f, "{}", translation_str)
    }
}

impl DisplayTranslation for BaiduTranslationResponse {}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct BaiduTransResult {
    src: String,
    dst: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct BaiduErrResponse {
    error_code: String,
    error_msg: String,
}
