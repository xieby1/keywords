use yake_rust::{get_fn_best, Config, StopWords};
use poppler::{PopplerDocument};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    file: std::path::PathBuf,
    #[arg(short, long, default_value_t=3)]
    ngrams: usize,
    #[arg(short, long, default_value_t=5)]
    minimum_chars: usize,
}

fn main() {
    let cli = Cli::parse();
    let file = PopplerDocument::new_from_file(cli.file, None).expect("Failed to open PDF file");
    let mut text = String::new();
    for page in file.pages() { text.push_str(&page.get_text().unwrap_or("")); }

    let config = Config {
        ngrams: cli.ngrams,
        minimum_chars: cli.minimum_chars,
        only_alphanumeric_and_hyphen: true,
        ..Config::default()
    };
    let ignored = StopWords::custom(
        include_str!("./stopwords.txt").lines().map(ToOwned::to_owned).collect()
    );
    let keywords = get_fn_best(
        // <50->0, 100->4, 1k->6, 10k->8
        |n|{if n<50 {0} else {(f64::log10(n as f64)*2.0) as usize} },
        &text, &ignored, &config
    );

    for keyword in keywords { println!("{}", keyword.raw); }
}
