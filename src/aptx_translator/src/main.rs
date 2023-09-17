use std::{io::{self, Write}, process};
use clap::{Parser, ValueEnum};

#[derive(Debug, Clone)]
#[derive(PartialEq, Eq)]
#[derive(Hash)]
#[derive(ValueEnum)]
enum Api {
    Baidu,
    Youdao,
}

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
        process::exit(0);
    })
        .unwrap();

    let args = Args::parse();
    // TODO: Create or load the translation APIs' configurations

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

        let src_txt = buf.trim();
        // TODO: Translate the source text via the specific translation API and show the translation
        let translation = src_txt;
        println!("[{:?}] Translation: {}", args.api, translation);
    }
}
