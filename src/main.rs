mod wordlist;

use std::{path::PathBuf, time::Instant, usize};

use clap::Parser;
use futures::stream::{FuturesOrdered, StreamExt};
use prettytable::{row, Table};
use reqwest::{Client, RequestBuilder};
use tokio::task::JoinHandle;
use wordlist::Wordlist;

#[derive(Parser)]
#[command(version = "1.0", about = "Burteforce Tool")]
struct Cli {
    #[arg(short, long)]
    url: String,
    #[arg(short, long, default_value = "GET")]
    method: Option<String>,

    #[arg(short, long, value_parser=parse_param )]
    params: Option<Vec<(String, String)>>,

    #[arg(short, long, value_parser=parse_wordlist)]
    wordlist: Option<Vec<(String, PathBuf)>>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let params: Vec<Vec<(String, String)>> = match cli.params {
        Some(params) => [params].to_vec(),
        None => match cli.wordlist {
            Some(wordlist) => param_combination(Wordlist::from(wordlist)?.data, 0, Vec::new()),
            None => [].to_vec(),
        },
    };
    let method = cli.method.ok_or(String::from("GET"))?;

    let mut tasks: FuturesOrdered<JoinHandle<Option<(String, String, u16, u128, usize)>>> =
        FuturesOrdered::new();

    let client = Client::new();

    match params.is_empty() {
        true => {
            let client = client.clone();
            let url = cli.url.clone();
            let request_builder: RequestBuilder =
                create_request_builder(&client, &method, &url, None)?;
            push_to_task(&mut tasks, request_builder, client);
        }
        false => {
            for p in params {
                let client = client.clone();
                let url = cli.url.clone();
                let request_builder: RequestBuilder =
                    create_request_builder(&client, &method, &url, Some(&p))?;
                push_to_task(&mut tasks, request_builder, client);
            }
        }
    }
    let mut table = Table::new();
    table.add_row(row!["URL", "Method", "Status", "RTT(ms)", "Size Body"]);
    while let Some(Ok(Some((url, method, status, duration, size)))) = tasks.next().await {
        table.add_row(row![url, method, status, duration, size]);
    }
    table.printstd();

    Ok(())
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
    params: Vec<(String, Vec<String>)>,
    index: usize,
    current: Vec<(String, String)>,
) -> Vec<Vec<(String, String)>> {
    if index == params.len() {
        return [current.clone()].to_vec();
    }

    let (key, values) = &params[index];
    let mut out: Vec<Vec<(String, String)>> = Vec::new();

    for value in values {
        let mut next = current.clone();
        next.push((key.clone(), value.clone()));
        out.extend(param_combination(params.clone(), index + 1, next))
    }

    out
}
fn create_request_builder(
    client: &Client,
    method: &String,
    url: &String,
    params: Option<&Vec<(String, String)>>,
) -> Result<RequestBuilder, Box<dyn std::error::Error>> {
    let builder = match method.to_lowercase().as_str() {
        "get" => params.map_or(client.get(url), |p| client.get(url).query(p)),
        "post" => params.map_or(client.post(url), |p| client.post(url).form(p)),
        "put" => params.map_or(client.put(url), |p| client.put(url).form(p)),
        "delete" => params.map_or(client.delete(url), |p| client.delete(url).query(p)),
        _ => {
            return Err(format!(
                "unsuported method {}, only support(POST, GET,PUT, DELETE )",
                method
            )
            .into());
        }
    };

    Ok(builder)
}
fn push_to_task(
    task: &mut FuturesOrdered<JoinHandle<Option<(String, String, u16, u128, usize)>>>,
    request_builder: RequestBuilder,
    client: Client,
) {
    task.push_back(tokio::spawn(async move {
        let start = Instant::now();
        let request = request_builder.build().unwrap();
        let url = request.url().clone();
        let method = request.method().clone();
        let result = client.execute(request).await;
        let duration = start.elapsed().as_millis();
        match result {
            Ok(res) => Some((
                url.to_string(),
                method.to_string(),
                res.status().as_u16(),
                duration,
                match res.text().await {
                    Ok(text) => text.len(),
                    Err(_) => 0,
                },
            )),
            Err(e) => {
                eprintln!("x{} =>{} ", url, e);
                None
            }
        }
    }));
}
