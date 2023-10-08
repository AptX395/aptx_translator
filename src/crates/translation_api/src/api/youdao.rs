use chrono::Utc;
use reqwest::{blocking::Client, Error as ReqwestErr};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;
use crate::{error::{Error, ErrorCode}, language::Language, Api};
use super::{Translate, DisplayTranslation};

#[derive(Deserialize)]
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
}

impl Translate for YoudaoApi {
    fn translate(
        &self,
        content: &str,
        src_lang: &Language,
        target_lang: &Language,
    ) -> Result<Box<dyn DisplayTranslation>, Error> {
        let params = YoudaoParams::new(content, src_lang, target_lang, &self.app_key, &self.app_secret);
        let request_result = self.request(params);

        let Ok(response_text) = request_result else {
            let err_str = request_result.unwrap_err().to_string();
            let request_err = Error::new(Api::Youdao, ErrorCode::RequestError, &err_str);
            return Err(request_err);
        };
        
        let deserialize_result: Result<YoudaoResponse, serde_json::Error> = serde_json::from_str(&response_text);
        
        let Ok(response) = deserialize_result else {
            let err_str = deserialize_result.unwrap_err().to_string();
            let api_err = Error::new(Api::Youdao, ErrorCode::DeserializeError, &err_str);
            return Err(api_err);
        };

        Ok(Box::new(response))
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
    pub fn new(q: &str, from: &Language, to: &Language, app_key: &str, app_secret: &str) -> Self {
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
        let length = q.len();

        if length <= 20 {
            return String::from(q);
        }

        format!("{}{}{}", &q[..10], length, &q[(length - 10)..])
    }
}

#[derive(Debug)]
#[derive(Deserialize)]
pub struct YoudaoResponse {
    #[serde(rename(deserialize = "errorCode"))]
    error_code: String,
    l: String,
    translation: Option<Vec<String>>,
    query: Option<String>,
    #[serde(rename(deserialize = "speakUrl"))]
    speak_url: Option<String>,
    #[serde(rename(deserialize = "tSpeakUrl"))]
    t_speak_url: Option<String>,
    basic: Option<YoudaoBasic>,
    web: Option<Vec<YoudaoWeb>>,
    dict: Option<YoudaoDict>,
    #[serde(rename(deserialize = "webdict"))]
    web_dict: Option<YoudaoWebDict>,
    return_phrase: Option<Vec<String>>,
    #[serde(rename(deserialize = "requestId"))]
    request_id: Option<String>,
    #[serde(rename(deserialize = "isWord"))]
    is_word: Option<bool>,
    #[serde(rename(deserialize = "mTerminalDict"))]
    m_terminal_dict: YoudaoMTerminalDict,
}

impl std::fmt::Display for YoudaoResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Some(translation) = &self.translation else {
            return write!(f, "[None...]");
        };

        let translation_str: String = translation.iter()
            .map(|x| {
                format!("{}\n", x)
            })
            .collect();

        write!(f, "{}", translation_str)
    }
}

impl DisplayTranslation for YoudaoResponse {}

#[derive(Debug)]
#[derive(Deserialize)]
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

#[derive(Debug)]
#[derive(Deserialize)]
struct YoudaoWeb {
    key: String,
    value: Vec<String>,
}

#[derive(Debug)]
#[derive(Deserialize)]
struct YoudaoDict {
    url: String,
}

#[derive(Debug)]
#[derive(Deserialize)]
struct YoudaoWebDict {
    url: String,
}

#[derive(Debug)]
#[derive(Deserialize)]
struct YoudaoMTerminalDict {
    url: String,
}

#[derive(Debug)]
#[derive(Deserialize)]
struct YoudaoWFContainer {
    wf: YoudaoWF,
}

#[derive(Debug)]
#[derive(Deserialize)]
struct YoudaoWF {
    name: String,
    value: String,
}
