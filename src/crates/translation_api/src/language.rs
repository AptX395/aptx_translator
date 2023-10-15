use clap::ValueEnum;

#[derive(Clone, Debug, ValueEnum)]
pub enum Language {
    Chinese,
    English,
}

impl Language {
    pub fn to_baidu_param(&self) -> String {
        match self {
            Self::Chinese => String::from("zh"),
            Self::English => String::from("en"),
        }
    }

    pub fn to_youdao_param(&self) -> String {
        match self {
            Self::Chinese => String::from("zh-CHS"),
            Self::English => String::from("en"),
        }
    }
}
