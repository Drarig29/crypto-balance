#[macro_use]
extern crate rocket;
extern crate chrono;
extern crate dotenv;
extern crate hex;
extern crate hmac;
extern crate mongodb;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate sha2;

mod aggregate;
mod database;
mod model;
mod requests;
mod utils;

use chrono::{DateTime, Utc};
use dotenv::dotenv;
use env::VarError;
use rocket::fs::NamedFile;
use rocket::response::content;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::{env, vec};

#[derive(Clone)]
pub struct Environment {
    binance_key: String,
    binance_secret: String,
    nomics_key: String,
    mongodb_host: String,
    mongodb_port: String,
    mongodb_username: String,
    mongodb_password: String,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody {
    conversion: String,
    start: String,
    end: String,
}

#[derive(Debug)]
pub struct TimeSpan {
    start: DateTime<Utc>,
    end: DateTime<Utc>,
}

pub const BINANCE_API_BASE_URL: &str = "https://api.binance.com/sapi/v1/accountSnapshot";
pub const NOMICS_API_BASE_URL: &str = "https://api.nomics.com/v1/currencies/sparkline";
pub const ACCOUNT_TYPE: &str = "SPOT"; // Can be MARGIN or FUTURES too

fn get_env_vars() -> Result<Environment, VarError> {
    let binance_key = env::var("BINANCE_API_KEY")?;
    let binance_secret = env::var("BINANCE_API_SECRET")?;
    let nomics_key = env::var("NOMICS_API_KEY")?;
    let mongodb_host = env::var("MONGODB_HOST")?;
    let mongodb_port = env::var("MONGODB_PORT")?;
    let mongodb_username = env::var("MONGODB_USERNAME")?;
    let mongodb_password = env::var("MONGODB_PASSWORD")?;

    Ok(Environment {
        binance_key,
        binance_secret,
        nomics_key,
        mongodb_host,
        mongodb_port,
        mongodb_username,
        mongodb_password,
    })
}

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open("static/index.html").await.ok()
}

#[get("/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).await.ok()
}

#[post("/api", format = "json", data = "<body>")]
async fn api(body: Json<RequestBody>) -> content::Json<String> {
    println!("Start: {}\nEnd: {}", body.start, body.end);
    let start = DateTime::parse_from_rfc3339(&body.start)
        .unwrap()
        .with_timezone(&Utc);

    let end = DateTime::parse_from_rfc3339(&body.end)
        .unwrap()
        .with_timezone(&Utc);

    let env_variables = match get_env_vars() {
        Ok(res) => res,
        Err(err) => return content::Json(err.to_string()),
    };

    let mongodb_url = format!(
        "mongodb://{}:{}@{}:{}",
        env_variables.mongodb_username,
        env_variables.mongodb_password,
        env_variables.mongodb_host,
        env_variables.mongodb_port,
    );

    let client = mongodb::Client::with_uri_str(&mongodb_url).await.unwrap();
    let database = client.database("crypto-balance");

    let available_snapshots = database::get_snapshots(&database, start, end).await;
    let needed_timespans = utils::get_timespans_to_retrieve(available_snapshots, start, end);

    if needed_timespans.is_empty() {
        let computed_snapshots = database::get_computed_snapshots(&database, start, end).await;
        let result = serde_json::to_string_pretty(&computed_snapshots).unwrap();
        return content::Json(result);
    }

    let split_by_30_days = utils::split_all_timespans_max_days(&needed_timespans, 30);

    let snapshots =
        requests::get_all_snapshots(&env_variables, ACCOUNT_TYPE, 30, &split_by_30_days).await;

    if let Ok(snapshots) = snapshots {
        database::push_snapshots(&database, snapshots).await;
    }

    let assets = database::get_possessed_assets(&database).await;
    let split_by_45_days = utils::split_all_timespans_max_days(&needed_timespans, 45);

    let price_history =
        requests::get_all_history(&env_variables, &assets, &body.conversion, &split_by_45_days)
            .await;

    if let Ok(price_history) = price_history {
        database::push_history(&database, price_history).await;
    }

    let computed_snapshots = database::get_computed_snapshots(&database, start, end).await;
    let result = serde_json::to_string_pretty(&computed_snapshots).unwrap();
    content::Json(result)
}

#[rocket::main]
async fn main() {
    dotenv().ok();

    rocket::build()
        .mount("/", routes![index, api, files])
        .launch()
        .await;
}
