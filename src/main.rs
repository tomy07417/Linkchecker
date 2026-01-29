mod parser;
mod custom_errors;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let file_path = std::path::Path::new(&args[1]);

    let urls: Vec<String> = parser::extract_urls_from_input(file_path).
        unwrap_or_else(|e| {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        });

    println!("Extracted URLs: {:?}", urls);
}
