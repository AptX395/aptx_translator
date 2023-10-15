use chrono::Utc;
use reqwest::{blocking::Client, Error as ReqwestErr};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::{Api, error::{Error, ErrCode, DESERIALIZE_RESPONSE_ERR_MSG}, language::Language};
use super::{DisplayTranslation, Translate};

#[derive(Debug, Deserialize)]
pub struct YoudaoApi {
    url: String,
    app_key: String,
    app_secret: String,
}

impl YoudaoApi {
    fn request(&self, params: YoudaoParams) -> Result<String, ReqwestErr> {
        let response_text = Client::new()
            .post(&self.url)
            .form(&params)
            .send()?
            .text()?;

        Ok(response_text)
    }

    fn parse_response(&self, response: &str) -> Result<Box<dyn DisplayTranslation>, Error> {
        let translation_response_result: Result<YoudaoTranslationResponse, serde_json::Error> =
            serde_json::from_str(response);
        
        if let Ok(translation_response) = translation_response_result {
            return Ok(Box::new(translation_response));
        }

        let err_response_result: Result<YoudaoErrResponse, serde_json::Error> =
            serde_json::from_str(response);

        if let Ok(err_response) = err_response_result {
            let api_err = Error::new(
                Api::Youdao,
                ErrCode::ApiError,
                &err_response.error_code,
            );

            return Err(api_err);
        }

        let deserialize_err = Error::new(
            Api::Youdao,
            ErrCode::DeserializeError,
            DESERIALIZE_RESPONSE_ERR_MSG,
        );

        Err(deserialize_err)
    }
}

impl Translate for YoudaoApi {
    fn translate(
        &self,
        text: &str,
        src_lang: &Language,
        target_lang: &Language,
    ) -> Result<Box<dyn DisplayTranslation>, Error> {
        let params = YoudaoParams::new(
            text,
            src_lang,
            target_lang,
            &self.app_key,
            &self.app_secret,
        );

        let request_result = self.request(params);

        let Ok(response_text) = request_result else {
            let request_err = Error::new(
                Api::Youdao,
                ErrCode::RequestError,
                &request_result.unwrap_err().to_string(),
            );

            return Err(request_err);
        };
        
        self.parse_response(&response_text)
    }
}

#[derive(Serialize)]
struct YoudaoParams {
    q: String,
    from: String,
    to: String,
    #[serde(rename(serialize = "appKey"))]
    app_key: String,
    salt: String,
    sign: String,
    #[serde(rename(serialize = "signType"))]
    sign_type: String,
    #[serde(rename(serialize = "curtime"))]
    cur_time: String,
}

impl YoudaoParams {
    pub fn new(
        q: &str,
        from: &Language,
        to: &Language,
        app_key: &str,
        app_secret: &str,
    ) -> Self {
        let input = Self::generate_input(q);
        let salt = Uuid::new_v4().to_string();
        let cur_time = Utc::now().timestamp().to_string();
        let sign_str = format!("{}{}{}{}{}", app_key, input, salt, cur_time, app_secret);
        let mut hasher = Sha256::new();
        hasher.update(sign_str.as_bytes());
        let sign = format!("{:x}", hasher.finalize());

        Self {
            q: String::from(q),
            from: from.to_youdao_param(),
            to: to.to_youdao_param(),
            app_key: String::from(app_key),
            salt,
            sign,
            sign_type: String::from("v3"),
            cur_time,
        }
    }

    fn generate_input(q: &str) -> String {
        let length = q.chars().count();

        if length <= 20 {
            String::from(q)
        } else {
            format!(
                "{}{}{}",
                q.chars().take(10).collect::<String>(),
                length,
                q.chars().skip(length - 10).collect::<String>(),
            )
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct YoudaoTranslationResponse {
    #[serde(rename(deserialize = "errorCode"))]
    error_code: String,
    l: String,
    query: String,
    translation: Vec<String>,
    #[serde(rename(deserialize = "speakUrl"))]
    speak_url: String,
    #[serde(rename(deserialize = "tSpeakUrl"))]
    t_speak_url: String,
    basic: Option<YoudaoBasic>,
    web: Option<Vec<YoudaoWeb>>,
    dict: Option<YoudaoDict>,
    #[serde(rename(deserialize = "webdict"))]
    web_dict: Option<YoudaoWebDict>,
    return_phrase: Option<Vec<String>>,
    #[serde(rename(deserialize = "requestId"))]
    request_id: String,
    #[serde(rename(deserialize = "isWord"))]
    is_word: bool,
    #[serde(rename(deserialize = "mTerminalDict"))]
    m_terminal_dict: Option<YoudaoMTerminalDict>,
}

impl std::fmt::Display for YoudaoTranslationResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let translation_str: String = self.translation.iter()
            .map(|x| {
                format!("{}\n", x)
            })
            .collect();

        write!(f, "{}", translation_str)
    }
}

impl DisplayTranslation for YoudaoTranslationResponse {}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct YoudaoBasic {
    explains: Vec<String>,
    phonetic: Option<String>,
    #[serde(rename(deserialize = "uk-phonetic"))]
    uk_phonetic: Option<String>,
    #[serde(rename(deserialize = "uk-speech"))]
    uk_speech: Option<String>,
    #[serde(rename(deserialize = "us-phonetic"))]
    us_phonetic: Option<String>,
    #[serde(rename(deserialize = "us-speech"))]
    us_speech: Option<String>,
    exam_type: Option<Vec<String>>,
    wfs: Option<Vec<YoudaoWFContainer>>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct YoudaoWeb {
    key: String,
    value: Vec<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct YoudaoDict {
    url: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct YoudaoWebDict {
    url: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct YoudaoMTerminalDict {
    url: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct YoudaoWFContainer {
    wf: YoudaoWF,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct YoudaoWF {
    name: String,
    value: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct YoudaoErrResponse {
    #[serde(rename(deserialize = "errorCode"))]
    error_code: String,
    l: String,
    #[serde(rename(deserialize = "requestId"))]
    request_id: String,
}
