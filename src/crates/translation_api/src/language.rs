use clap::ValueEnum;

#[derive(Debug, Clone, Copy)]
#[derive(ValueEnum)]
pub enum Language {
    Chinese,
    English,
}

impl Language {
    pub fn to_baidu_param(&self) -> String {
        match self {
            Language::Chinese => String::from("zh"),
            Language::English => String::from("en"),
        }
    }
}
