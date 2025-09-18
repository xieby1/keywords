use yake_rust::{get_n_best, Config, StopWords};
use poppler;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename> [n_keywords]", args[0]);
        std::process::exit(1);
    }
    let filename = &args[1];
    let n_keywords: usize = if args.len() >= 3 {
        args[2].parse().expect("n_keywords must be a positive integer")
    } else {
        10
    };

    let file: poppler::PopplerDocument = poppler::PopplerDocument::new_from_file(filename, None)
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
    let ignored = StopWords::predefined("en").unwrap();
    let keywords = get_n_best(n_keywords, &text, &ignored, &config);

    println!("{:#?}", keywords);
}
