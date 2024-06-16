use std::fs;

fn main() {
   let args: Vec<String> =  std::env::args().collect();
   let url = &args[1];
   let file_to_write = &args[2];

   let _ = fetch_rust_lang_as_md(url, file_to_write);

}



fn fetch_rust_lang_as_md(url: &str, file_to_write: &str) -> Result<(), Box<dyn std::error::Error>> {

    println!("Fetching url: {}", url);
    let body = reqwest::blocking::get(url)?.text()?;

    println!("Converting html to md...");
    let md = html2md::parse_html(&body);

    fs::write(file_to_write,md.as_bytes())?;
    println!("Converted markdown has been saved in {}.", file_to_write);

    Ok(())
}
