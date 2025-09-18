use yake_rust::{get_fn_best, Config, StopWords};
use poppler;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    file: std::path::PathBuf,
}

fn main() {
    let cli = Cli::parse();
    let file: poppler::PopplerDocument = poppler::PopplerDocument::new_from_file(cli.file, None)
        .expect("Failed to open PDF file");
    let mut text = String::new();
    for page in file.pages() {
        // This is how to type a loop variable?
        let page: poppler::PopplerPage = page;
        text.push_str(&page.get_text().unwrap_or(""));
    }
    println!("{}", text);

    let config = Config {
        ngrams: 3,
        minimum_chars: 5,
        only_alphanumeric_and_hyphen: true,
        ..Config::default()
    };
    let ignored = StopWords::custom(
        include_str!("./stopwords.txt").lines().map(ToOwned::to_owned).collect()
    );
    // TODO: replace `n_keywords` with a ratio of number of words in text
    // <50->0, 100->4, 1k->6, 10k->8
    let keywords = get_fn_best(
        |n|{if n<50 {0} else {(f64::log10(n as f64)*2.0) as usize} },
        &text, &ignored, &config
    );

    println!("{:#?}", keywords);
}
