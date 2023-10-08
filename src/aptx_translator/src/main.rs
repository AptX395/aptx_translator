use std::{fs, io, process};
use clap::Parser;
use sysexits::ExitCode;
use translation_api::{api::*, Api, language::Language};

#[derive(Debug)]
#[derive(Parser)]
#[command(author, version, about = "A lightweight translator write in pure Rust, supports multiple translation APIs")]
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
        println!("\n[Quit...]");
        process::exit(ExitCode::Ok as i32);
    }).unwrap();

    let args = Args::parse();
    let translation_api = load_translation_api(&args.config_path, &args.api).unwrap();

    loop {
        let mut buf = String::new();
        println!("[Content]:");

        if let Err(_) = io::stdin().read_line(&mut buf) {
            continue;
        }

        let content = buf.trim();
        let translate_result = translation_api.translate(content, &args.src_lang, &args.target_lang);

        let Ok(translation) = translate_result else {
            eprintln!(
                "{}",
                translate_result.unwrap_err()
                    .to_string(),
            );

            continue;
        };

        println!("\n[{} translation]:\n{}", args.api, translation.to_string());
    }
}

fn load_translation_api(config_path: &str, api: &Api) -> Option<Box<dyn Translate>> {
    let read_file_result = fs::read_to_string(config_path);

    let Ok(file_content) = read_file_result else {
        eprintln!("{}", read_file_result.unwrap_err().to_string());
        return None;
    };

    match api {
        Api::Baidu =>  {
            let baidu_api: BaiduApi = toml::from_str(&file_content).unwrap();

            Some(Box::new(baidu_api))
        }
        Api::Youdao => {
            let youdao_api: YoudaoApi = toml::from_str(&file_content).unwrap();

            Some(Box::new(youdao_api))
        }
    }
}