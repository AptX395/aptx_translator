pub mod api;
pub mod error;
pub mod language;

use clap::ValueEnum;

#[derive(Debug, Clone)]
#[derive(PartialEq, Eq)]
#[derive(Hash)]
#[derive(ValueEnum)]
pub enum Api {
    Baidu,
    Youdao,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        // TODO: Check if the translation APIs can work
    }
}
