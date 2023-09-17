use std::{collections::HashMap, io::{self, Write}, process};
use clap::Parser;
use sysexits::ExitCode;
use translation_api::{api::*, Api};

#[derive(Debug)]
#[derive(Parser)]
#[command(author, version, about="Press `Ctrl + C` to quit")]
struct Args {
    #[arg(long, short, name = "api_name")]
    api: Api,
}

fn main() {
    ctrlc::set_handler(|| {
        println!("\nQuit...");
        process::exit(ExitCode::Ok.into());
    })
        .unwrap();

    let args = Args::parse();
    // TODO: Create or load the translation APIs' configurations
    let baidu = Baidu::new("", "", "");
    let youdao = Youdao::new("", "", "");
    let mut translation_apis: HashMap<Api, &dyn Translate> = HashMap::new();
    translation_apis.insert(Api::Baidu, &baidu);
    translation_apis.insert(Api::Youdao, &youdao);

    loop {
        let mut buf = String::new();
        print!("Source: ");

        if let Err(_) = io::stdout().flush() {
            continue;
        }

        let read_result = io::stdin().read_line(&mut buf);

        if read_result.is_err() {
            continue;
        }

        let src_text = buf.trim();
        // TODO: Translate the source text via the specific translation API and show the translation

        let Ok(translation) = translation_apis[&args.api].translate(src_text) else {
            return;
        };

        println!("[{:?}] Translation: {:?}", args.api, translation);
    }
}
