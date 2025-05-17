use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Result {
    result_count: i32,
    offset: i32,
    limit: i32,
    query_time: i32,
    results: Vec<SsnResult>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SsnResult {
    testpersonnummer: String,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = String::from(".*"))]
    pattern: String,

    #[arg(short, long, default_value_t = 100)]
    limit: i32,

    #[arg(short, long, default_value_t = 0)]
    offset: i32,

    #[arg(short, long, default_value_t = false)]
    json: bool,
}

fn main() {
    let args = Args::parse();
    let pattern = args.pattern;
    let limit = args.limit;
    let offset = args.offset;
    let url = format!(
        "https://skatteverket.entryscape.net/rowstore/dataset/b4de7df7-63c0-4e7e-bb59-1f156a591763?testpersonnummer={pattern}&_limit={limit}&_offset={offset}"
    );

    let items = get_items(url);

    print_items(args.json, items);
}

fn print_items(json: bool, items: Vec<String>) {
    match json {
        false => {
            for item in items {
                println!("{}", item);
            }
        }
        true => {
            let json_str = serde_json::to_string(&items).unwrap();
            println!("{}", json_str);
        }
    }
}

fn get_items(url: String) -> std::vec::Vec<std::string::String> {
    let response = reqwest::blocking::get(url).unwrap();
    let result: Result = response.json().unwrap();
    result
        .results
        .iter()
        .map(|v| v.testpersonnummer.to_string())
        .collect()
}
