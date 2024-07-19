use clap::Parser;
use glob::glob;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// regexp to grep in file
    params: Vec<String>,
}

fn main() {
    let args = Args::parse();

    if args.params.len() == 0 {
        println!("no params");
        return;
    }

    if args.params.len() == 1 {
        println!("regexp should be specified");
        return;
    }

    let regexp = args.params[0].as_str();
    let filename = args.params[1].as_str();
    println!("regexp: {}, filename: {}", regexp, filename);

    // glob files
    for entry in glob(filename).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                println!("path: {}", path.display());
            },
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

}