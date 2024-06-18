use std::fs;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if let [_path, url, output, ..] = args.as_slice() {
        let _ = fetch_rust_lang_as_md(url, output);
    } else {
        eprintln!("参数缺失");
    }
}

fn fetch_rust_lang_as_md(url: &str, output: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Fetching url: {}", url);
    let body = reqwest::blocking::get(url)?.text()?;

    println!("Converting html to md...");
    let md = html2md::parse_html(&body);

    fs::write(output, md.as_bytes())?;
    println!("Converted markdown has been saved in {}.", output);

    Ok(())
}
