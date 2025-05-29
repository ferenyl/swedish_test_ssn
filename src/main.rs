use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ApiResult {
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
#[command(
    version,
    about,
    long_about = None,
    about = "Get swedish test ssn from The Swedish Tax Agency (Skatteverket). Great for testing without risking breaching GDPR"
)]
struct Args {
    #[arg(short, long, default_value_t = String::from(".*"), help = "Pattern for ssn. Regular expressions can be used")]
    pattern: String,

    #[arg(
        short,
        long,
        default_value_t = 100,
        help = "Limit the number of items returned"
    )]
    limit: i32,

    #[arg(short, long, default_value_t = 0, help = "Number of items to skip")]
    offset: i32,

    #[arg(short, long, default_value_t = false, help = "Return as json array")]
    json: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    match get_items(&args) {
        Ok(items) => print_items(args.json, &items),
        Err(e) => eprintln!("Failed to fetch items: {}", e),
    }

    Ok(())
}

fn print_items(json: bool, items: &[String]) {
    if json {
        let json_str = serde_json::to_string_pretty(items).unwrap();
        println!("{}", json_str);
    } else {
        for item in items {
            println!("{}", item);
        }
    }
}

fn get_items(args: &Args) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let url = format!(
        "https://skatteverket.entryscape.net/rowstore/dataset/b4de7df7-63c0-4e7e-bb59-1f156a591763?testpersonnummer={}&_limit={}&_offset={}",
        args.pattern, args.limit, args.offset
    );
    let response = reqwest::blocking::get(url)?;
    let result: ApiResult = response.json()?;

    if result.results.is_empty() {
        Err(format!("No result for pattern: {}", args.pattern))?
    }

    Ok(result
        .results
        .into_iter()
        .map(|v| v.testpersonnummer)
        .collect())
}
