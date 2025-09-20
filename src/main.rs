use yake_rust::{get_fn_best, Config, StopWords};
use poppler::{PopplerDocument, PopplerPage};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    file: std::path::PathBuf,
    #[arg(short, long, default_value_t=3)]
    ngrams: usize,
    #[arg(short, long, default_value_t=5)]
    min_chars: usize,
    #[arg(short, long, default_value_t=20)]
    batch_max_pages: usize,
}

fn main() {
    let cli = Cli::parse();
    let file = PopplerDocument::new_from_file(cli.file, None).expect("Failed to open PDF file");
    let mut keywords = std::collections::HashSet::new();
    let config = Config {
        ngrams: cli.ngrams,
        minimum_chars: cli.min_chars,
        only_alphanumeric_and_hyphen: true,
        ..Config::default()
    };
    let ignored = StopWords::custom(
        include_str!("./stopwords.txt").lines().map(ToOwned::to_owned).collect()
    );

    for batch_pages in file.pages().collect::<Vec<PopplerPage>>().chunks(
        (file.get_n_pages() + cli.batch_max_pages - 1) / cli.batch_max_pages
    ) {
        let mut text = String::new();
        for page in batch_pages.iter() {
            text.push_str(page.get_text().unwrap_or(""))
        }
        keywords.extend(get_fn_best(
            // <50->0, 100->2, 1k->3, 10k->4
            |n|{if n<50 {0} else {(f64::log10(n as f64)) as usize} },
            &text, &ignored, &config
        ).iter().map(|item|item.raw.clone()));
    }
    for keyword in keywords { println!("{}", keyword); }
}
