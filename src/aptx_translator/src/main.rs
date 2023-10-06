use std::{collections::HashMap, io::{self, Write}, process, fs};
use clap::Parser;
use sysexits::ExitCode;
use translation_api::{api::*, Api, language::Language};

#[derive(Debug)]
#[derive(Parser)]
#[command(author, version, about="A lightweight translator write in pure Rust, supports multiple translation APIs")]
struct Args {
    #[arg(long, short, name="API name")]
    api: Api,
    #[arg(long, short, name="configuration file path")]
    config_path: String,
    #[arg(long, short, name="source language")]
    src_lang: Language,
    #[arg(long, short, name="target language")]
    target_lang: Language,
}

fn main() {
    ctrlc::set_handler(|| {
        println!("\n[Quit...]");
        process::exit(ExitCode::Ok.into());
    })
        .unwrap();

    let args = Args::parse();
    let read_file_result = fs::read_to_string(args.config_path);

    let Ok(content) = read_file_result else {
        eprintln!("{}", read_file_result.unwrap_err().to_string());
        return;
    };

    let deserialize_result = toml::from_str::<BaiduApi>(&content);

    let Ok(baidu_api) = deserialize_result else {
        eprintln!("{}", deserialize_result.unwrap_err().to_string());
        return;
    };

    let mut translation_apis: HashMap<Api, &dyn Translate> = HashMap::new();
    translation_apis.insert(Api::Baidu, &baidu_api);

    loop {
        let mut buf = String::new();
        println!("[Content]:");

        if let Err(_) = io::stdin().read_line(&mut buf) {
            continue;
        }

        let src_text = buf.trim();
        let translate_result = translation_apis[&args.api].translate(src_text, args.src_lang, args.target_lang);

        let Ok(translation) = translate_result else {
            eprintln!("{}", translate_result.unwrap_err().to_string());
            continue;
        };

        println!("[{:?} translation]:\n{}", args.api, translation.text());
    }
}
