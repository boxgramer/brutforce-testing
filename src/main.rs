mod wordlist;

use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::{Path, PathBuf},
};

use clap::Parser;
use reqwest::{Client, RequestBuilder};
use wordlist::Wordlist;

#[derive(Parser)]
#[command(version = "1.0", about = "Burteforce Tool")]
struct Cli {
    #[arg(short, long)]
    url: String,
    #[arg(short, long, default_value = "GET")]
    method: String,

    #[arg(short, long, value_parser=parse_param )]
    params: Vec<(String, String)>,

    #[arg(short, long, value_parser=parse_wordlist)]
    wordlist: Vec<(String, PathBuf)>,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let client = Client::new();
    let word_list = Wordlist::from(cli.wordlist).unwrap();
    let params = param_combination(&word_list.data, 0, Vec::new());

    let request_builder: RequestBuilder = match cli.method.to_lowercase().as_str() {
        "get" => client.get(cli.url).query(&cli.params),
        "post" => client.post(cli.url).form(&cli.params),
        "put" => client.post(cli.url).form(&cli.params),
        "delete" => client.post(cli.url).query(&cli.params),
        _ => {
            eprintln!(
                "unsuported method {}, only support(POST, GET,PUT, DELETE )",
                cli.method
            );
            std::process::exit(1);
        }
    };
    let request = request_builder.build().unwrap();
    println!("request to  url : {} ", request.url());

    let result = client.execute(request).await;
    match result {
        Ok(res) => println!("Request send! status : {}", res.status()),
        Err(e) => eprint!("Failed to send request: {}", e),
    }
}

fn parse_param(s: &str) -> Result<(String, String), String> {
    let pos = s
        .find(':')
        .ok_or_else(|| format!("Invalid key:value : no : found in {}", s))?;

    Ok((s[..pos].to_string(), s[pos + 1..].to_string()))
}

fn parse_wordlist(s: &str) -> Result<(String, PathBuf), String> {
    let pos = s
        .find(':')
        .ok_or_else(|| format!("invalid key, `:` not found in {} ", s))?;
    Ok((s[..pos].to_string(), PathBuf::from(&s[pos + 1..])))
}

fn param_combination(
    params: &Vec<(String, Vec<String>)>,
    index: usize,
    mut current: Vec<(String, String)>,
) -> Vec<Vec<(String, String)>> {
    if index == params.len() {
        return [current.clone()].to_vec();
    }

    let (key, values) = &params[index];
    let mut out: Vec<Vec<(String, String)>> = Vec::new();

    for value in values {
        current.push((key.clone(), value.clone()));
        out.extend(param_combination(params, index + 1, current.clone()))
    }

    out
}
