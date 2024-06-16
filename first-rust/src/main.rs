use std::fs;

fn main() {
   fetch_rust_lang_as_md();
    // println!("square of 25 is {}", apply(25, square));
    // println!("cube of 25 is {}", apply(25, cube));
}

fn apply(value: i32, f: fn(i32) -> i32) -> i32 {
    f(value)
}

fn square(value: i32) -> i32 {
    value * value
}

fn cube(value: i32) -> i32 {
    value * value * value
}

fn fetch_rust_lang_as_md() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://www.rust-lang.org/";
    let output = "rust.md";

    println!("Fetching url: {}", url);
    let body = reqwest::blocking::get(url)?.text()?;

    println!("Converting html to md...");
    let md = html2md::parse_html(&body);

    fs::write(output,md.as_bytes())?;
    println!("Converted markdown has been saved in {}.", output);

    Ok(())
}
