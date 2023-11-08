use clap::{ App, Arg };
use glob::glob;
use std::fs;
use std::path::Path;

fn main() {
    let matches = App::new("c2p")
        .version("0.2.0")
        .author("Your Name")
        .about("Prints contents of files specified by filenames or glob patterns")
        .arg(
            Arg::new("files_or_patterns")
                .multiple_values(true)
                .required(true)
                .help("The list of files or glob patterns")
        )
        .get_matches();

    let inputs: Vec<String> = matches
        .values_of("files_or_patterns")
        .unwrap()
        .map(|s| s.to_string())
        .collect();

    for input in inputs {
        // Check if the input is a file
        if Path::new(&input).is_file() {
            print_file(&input);
        } else {
            // Treat input as a glob pattern
            for entry in glob(&input).expect("Failed to read glob pattern") {
                match entry {
                    Ok(path) => {
                        if path.is_file() {
                            print_file(path.to_str().unwrap());
                        }
                    }
                    Err(e) => eprintln!("Error matching pattern {}: {}", input, e),
                }
            }
        }
    }
}

fn print_file(file_path: &str) {
    match fs::read_to_string(file_path) {
        Ok(contents) => {
            println!("File: {}\n{}", file_path, contents);
        }
        Err(e) => {
            eprintln!("Error reading file {}: {}", file_path, e);
        }
    }
}
