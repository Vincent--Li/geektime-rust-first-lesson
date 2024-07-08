pub trait Formatter {
    fn format(&self, input: &mut String) -> bool;
    fn get_name(&self) -> &'static str;
}

struct MDFormatter;
impl Formatter for MDFormatter {
    fn format(&self, input: &mut String) -> bool {
        input.push_str("\nformatted with Markdown formatter");
        true
    }

    fn get_name(&self) -> &'static str {
        "Markdown"
    }
}

struct RustFormatter;
impl Formatter for RustFormatter {
    fn format(&self, input: &mut String) -> bool {
        input.push_str("\nformatted with Rust formatter");
        true
    }

    fn get_name(&self) -> &'static str {
        "Rust"
    }
}

struct HtmlFormatter;
impl Formatter for HtmlFormatter {
    fn format(&self, input: &mut String) -> bool {
        input.push_str("\nformatted with HTML formatter");
        true
    }

    fn get_name(&self) -> &'static str {
        "HTML"
    }

}

// formatters: 动态分派. 运行时确定类型
pub fn format(input: &mut String, formatters: Vec<&dyn Formatter>) {
    for formatter in formatters {
        formatter.format(input);
    }
}

pub fn format_2(input: &mut String, formatters: Vec<Box<dyn Formatter>>) {
    for formatter in formatters {
        println!("formatter: {:?}", formatter.get_name());
        formatter.format(input);
    }
}

fn main() {
    let mut text = "Hello world!".to_string();
    // 使用 &dyn Trait 这里必须声明类型. 否则走不下去,
    let md: &dyn Formatter = &MDFormatter;
    let html: &dyn Formatter = &HtmlFormatter;
    let rust: &dyn Formatter = &RustFormatter;
    let formatters = vec![html, rust, md];
    format(&mut text, formatters);
    println!("text: {}", text);

    // 使用 Box<dyn Trait>
    let md: Box<dyn Formatter> = Box::new(MDFormatter);
    let html: Box<dyn Formatter> = Box::new(HtmlFormatter);
    let rust: Box<dyn Formatter> = Box::new(RustFormatter);
    let formatters = vec![html, rust, md];
    format_2(&mut text, formatters);
    println!("text: {}", text);
}
