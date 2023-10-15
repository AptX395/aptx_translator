use std::{fs, io, process};

use clap::Parser;
use sysexits::ExitCode;

use translation_api::{Api, api::*, language::Language};

const INPUT_HINT: &str = "[Text]: ";
const QUIT_HINT: &str = "[Quit...]";

#[derive(Debug, Parser)]
#[command(
    author,
    version,
    about = "A lightweight translator written in pure Rust, supports multiple translation APIs",
)]
struct Args {
    #[arg(long, short, name = "API name")]
    api: Api,
    #[arg(long, short, name = "configuration file path")]
    config_path: String,
    #[arg(long, short, name = "source language")]
    src_lang: Language,
    #[arg(long, short, name = "target language")]
    target_lang: Language,
}

fn main() {
    ctrlc::set_handler(|| {
        println!("{}", QUIT_HINT);
        process::exit(ExitCode::Ok.into());
    }).unwrap();

    let args = Args::parse();
    let translation_api = load_translation_api(&args.config_path, &args.api).unwrap();

    loop {
        let mut buf = String::new();
        println!("{}", INPUT_HINT);

        if let Err(read_line_err) = io::stdin().read_line(&mut buf) {
            eprintln!("{}", read_line_err);
            continue;
        }

        let text = buf.trim();

        let translate_result = translation_api.translate(
            text,
            &args.src_lang,
            &args.target_lang,
        );

        if let Ok(translation) = translate_result {
            print!("{}", translation);
        } else {
            eprintln!("{}", translate_result.unwrap_err());
        }
    };
}

fn load_translation_api(
    config_path: &str,
    api: &Api,
) -> Result<Box<dyn Translate>, Box<dyn std::error::Error>> {
    let file_content = fs::read_to_string(config_path)?;

    match api {
        Api::Baidu =>  {
            let baidu_api: BaiduApi = toml::from_str(&file_content)?;

            Ok(Box::new(baidu_api))
        }
        Api::Youdao => {
            let youdao_api: YoudaoApi = toml::from_str(&file_content)?;

            Ok(Box::new(youdao_api))
        }
    }
}
