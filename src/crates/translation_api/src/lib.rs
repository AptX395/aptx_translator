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
