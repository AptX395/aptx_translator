pub mod api;
pub mod error;
pub mod language;

use clap::ValueEnum;
use strum::Display;

#[derive(Clone, Debug, Display, Eq, Hash, PartialEq, ValueEnum)]
pub enum Api {
    Baidu,
    Youdao,
}
