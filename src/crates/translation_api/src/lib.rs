pub mod api;
pub mod error;
pub mod language;

use clap::ValueEnum;

#[derive(Debug)]
#[derive(Clone)]
#[derive(Eq, Hash, PartialEq)]
#[derive(ValueEnum)]
#[derive(strum::Display)]
pub enum Api {
    Baidu,
    Youdao,
}
