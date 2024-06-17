use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;
use mime::Mime;
use reqwest::{header, Client, Response, Url};
use std::{collections::HashMap, str::FromStr};
use std::{fmt::Display, path::PathBuf};
use html_parser::Dom;


// 定义 HTTPie 的 CLI 的主入口，它包含若干个子命令
// 下面 /// 的注释是文档，clap 会将其作为 CLI 的帮助
/// A naive httpie implementation with Rust, can you imagine how easy it is?
#[derive(Parser, Debug)]
#[command(version, about, long_about = None, author = "Vincent")]
struct Cli {
    /// Optional name to operate on
    name: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

// 子命令对应不同的Http方法
#[derive(Subcommand, Debug)]
enum Commands {
    #[command(about = "Get From Url")]
    Get(Get),
    #[command(about = "Post key-value pairs")]
    Post(Post),
}

#[derive(Parser, Debug)]
struct Get {
    // parse_url 校验url是否合法
    #[arg(value_parser = parse_url)]
    url: String,
}

#[derive(Parser, Debug)]
struct Post {
    url: String,
    #[arg(value_parser = KvPair::from_str)]
    body: Vec<KvPair>,
}

#[derive(Debug, Clone)]
struct KvPair {
    k: String,
    v: String,
}

impl FromStr for KvPair {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('=').collect();
        if parts.len() != 2 {
            Err("Each key-value pair must be in the form key=value".to_string())
        } else {
            Ok(KvPair {
                k: parts[0].to_string(),
                v: parts[1].to_string(),
            })
        }
    }
}

impl Display for KvPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", self.k, self.v)
    }
}

fn parse_url(s: &str) -> Result<String> {
    let _url: Url = s.parse()?;

    Ok(s.into())
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // You can check the value provided by positional arguments, or option arguments
    if let Some(name) = cli.name.as_deref() {
        println!("Value for name: {name}");
    }

    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    let client = Client::new();
    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    let result = match &cli.command {
        Some(Commands::Get(ref args)) => get(client, args).await?,
        Some(Commands::Post(ref args)) => post(client, args).await?,
        None => {
            println!("No subcommand")
        }
    };

    Ok(result)
}

async fn get(client: Client, args: &Get) -> Result<()> {
    let resp = client.get(&args.url).send().await?;
    Ok(print_resp(resp).await?)
}

async fn post(client: Client, args: &Post) -> Result<()> {
    let mut body = HashMap::new();
    for pair in &args.body {
        body.insert(&pair.k, &pair.v);
    }
    let resp = client.post(&args.url).json(&body).send().await?;
    Ok(print_resp(resp).await?)
}

// 打印服务器版本号 + 状态码
fn print_status(resp: &Response) {
    let status = format!("{:?} {}", resp.version(), resp.status()).blue();
    println!("{}\n", status);
}
// 打印服务器返回的 HTTP header
fn print_headers(resp: &Response) {
    for (name, value) in resp.headers() {
        println!("{}: {:?}", name.to_string().green(), value);
    }
    print!("\n");
}
/// 打印服务器返回的 HTTP body
fn print_body(m: Option<Mime>, body: &String) {

    match m {
        // 对于 "application/json" 我们 pretty print
        Some(v) if v == mime::APPLICATION_JSON => {
            println!("{}", jsonxf::pretty_print(body).unwrap().cyan())
        }
        Some(v) if v == mime::TEXT_HTML => {
            println!("{}", Dom::parse(body).unwrap().to_json_pretty().unwrap())
        }
        // 其它 mime type，我们就直接输出
        _ => println!("{}", body),
    }
}

/// 打印整个响应
async fn print_resp(resp: Response) -> Result<()> {
    print_status(&resp);
    print_headers(&resp);
    let mime = get_content_type(&resp);
    let body = resp.text().await?;
    print_body(mime, &body);
    Ok(())
}

/// 将服务器返回的 content-type 解析成 Mime 类型
fn get_content_type(resp: &Response) -> Option<Mime> {
    resp.headers()
        .get(header::CONTENT_TYPE)
        .map(|v| v.to_str()
        .unwrap().parse()
        .unwrap())
}