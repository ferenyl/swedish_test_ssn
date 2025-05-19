use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Result {
    result_count: i32,
    offset: i32,
    limit: i32,
    query_time: i32,
    results: Vec<SsnResult>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SsnResult {
    testpersonnummer: String,
}

#[derive(Parser)]
#[command(version, about, long_about = None, about = String::from("Get swedish test ssn from The Swedish Tax Agency (Skatteverket). Great for testing without risking breaching GDPR"))]
struct Args {
    #[arg(short, long, default_value_t = String::from(".*"), help = String::from("Pattern for ssn. Regular expressions can be used"))]
    pattern: String,

    #[arg(short, long, default_value_t = 100, help = String::from("Limit the number of items returned"))]
    limit: i32,

    #[arg(short, long, default_value_t = 0, help = String::from("Number of items to skip"))]
    offset: i32,

    #[arg(short, long, default_value_t = false, help = String::from("Return as json array"))]
    json: bool,
}

fn main() {
    let args = Args::parse();

    let url = format!(
        "https://skatteverket.entryscape.net/rowstore/dataset/b4de7df7-63c0-4e7e-bb59-1f156a591763?testpersonnummer={}&_limit={}&_offset={}",
        args.pattern, args.limit, args.offset
    );

    let items = get_items(&url);

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
            let json_str = serde_json::to_string_pretty(&items).unwrap();
            println!("{}", json_str);
        }
    }
}

fn get_items(url: &str) -> Vec<String> {
    let response = reqwest::blocking::get(url).unwrap();
    let result: Result = response.json().unwrap();
    result
        .results
        .iter()
        .map(|v| v.testpersonnummer.clone())
        .collect()
}
