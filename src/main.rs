#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
extern crate chrono;
extern crate dotenv;
extern crate hex;
extern crate hmac;
extern crate mongodb;
extern crate reqwest;
extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;
extern crate sha2;

use dotenv::dotenv;

mod model;
use bson::Bson;
use model::binance;
use model::database;
use model::nomics;

use mongodb::bson::doc;
use mongodb::options::FindOptions;
use mongodb::{bson, sync::Database};

use env::VarError;
use std::io;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use std::{env, vec};

use rocket::response::{content, NamedFile};
use rocket_contrib::json::Json;

use serde::{Deserialize, Serialize};

use reqwest::blocking::Client;

use chrono::{DateTime, Duration, SecondsFormat, TimeZone, Utc};

use hmac::{Hmac, Mac, NewMac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

struct Environment {
    binance_key: String,
    binance_secret: String,
    nomics_key: String,
    mongodb_host: String,
    mongodb_port: String,
    mongodb_username: String,
    mongodb_password: String,
}

#[derive(Serialize, Deserialize)]
struct RequestBody {
    conversion: String,
    start: String,
    end: String,
}

#[derive(Debug)]
struct TimeSpan {
    start: DateTime<Utc>,
    end: DateTime<Utc>,
}

const BINANCE_API_BASE_URL: &str = "https://api.binance.com/sapi/v1/accountSnapshot";
const NOMICS_API_BASE_URL: &str = "https://api.nomics.com/v1/currencies/sparkline";

const ACCOUNT_TYPE_SPOT: &str = "SPOT";
const ACCOUNT_TYPE_MARGIN: &str = "MARGIN";
const ACCOUNT_TYPE_FUTURES: &str = "FUTURES";

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

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

fn get_missing_timespans(needed: TimeSpan, available: TimeSpan) -> Vec<TimeSpan> {
    assert!(needed.start <= needed.end);
    assert!(available.start <= available.end);

    if needed.start >= available.start && needed.end <= available.end {
        return vec![];
    }

    if needed.start >= available.end && needed.end >= available.end {
        let start = if needed.start == available.end {
            needed.start + Duration::days(1)
        } else {
            needed.start
        };

        return vec![TimeSpan { start, ..needed }];
    }

    if needed.start <= available.start && needed.end <= available.start {
        let end = if needed.end == available.start {
            needed.end - Duration::days(1)
        } else {
            needed.end
        };

        return vec![TimeSpan { end, ..needed }];
    }

    if needed.start <= available.start && needed.end <= available.end {
        let end = available.start - Duration::days(1);
        return vec![TimeSpan { end, ..needed }];
    }

    if needed.start >= available.start && needed.end >= available.end {
        let start = available.end + Duration::days(1);
        return vec![TimeSpan { start, ..needed }];
    }

    if needed.start <= available.start && needed.end >= available.end {
        let end = available.start - Duration::days(1);
        let start = available.end + Duration::days(1);
        return vec![TimeSpan { end, ..needed }, TimeSpan { start, ..needed }];
    }

    panic!("Unsupported case!");
}

fn get_timespans_to_retrieve(
    snapshots: Vec<database::Snapshot>,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
) -> Vec<TimeSpan> {
    if snapshots.is_empty() {
        println!("No available data.");
        return vec![TimeSpan { start, end }];
    }

    let database_start: DateTime<Utc> = From::from(snapshots.first().unwrap().time);
    let database_end: DateTime<Utc> = From::from(snapshots.last().unwrap().time);

    println!(
        "Database start: {}\nDatabase end: {}",
        database_start, database_end
    );

    let needed = TimeSpan { start, end };

    let available = TimeSpan {
        start: database_start,
        end: database_end,
    };

    let missing = get_missing_timespans(needed, available);

    println!("Missing: {:?}", missing);

    missing
}

fn split_timespan_max_days(timespan: &TimeSpan, max_days: i64) -> Vec<TimeSpan> {
    if (timespan.end - timespan.start).num_days() < max_days {
        return vec![TimeSpan {
            start: timespan.start,
            end: timespan.end,
        }];
    }

    let mut timespans: Vec<TimeSpan> = vec![];

    let mut current_start = timespan.start.clone();
    let mut current_end = timespan.start + Duration::days(max_days - 1);

    while current_end < timespan.end {
        timespans.push(TimeSpan {
            start: current_start,
            end: current_end,
        });

        current_start = current_end + Duration::days(1);
        current_end = current_start + Duration::days(max_days - 1);
    }

    timespans.push(TimeSpan {
        start: current_start,
        end: timespan.end,
    });

    timespans
}

fn split_all_timespans_max_days(timespans: &Vec<TimeSpan>, max_days: i64) -> Vec<TimeSpan> {
    let mut results: Vec<TimeSpan> = vec![];

    for timespan in timespans {
        let mut intermediate_results = split_timespan_max_days(timespan, max_days);
        results.append(&mut intermediate_results);
    }

    results
}

fn run_request_loop<T>(
    timespans: &Vec<TimeSpan>,
    func: &mut dyn FnMut(&TimeSpan) -> Result<Vec<T>, reqwest::Error>,
) -> vec::Vec<T> {
    let mut results: Vec<T> = vec![];

    for timespan in timespans {
        let mut intermediate_results = func(timespan).unwrap();
        results.append(&mut intermediate_results);
    }

    results
}

fn get_uri_escaped_datetime(datetime: DateTime<Utc>) -> String {
    let formatted = datetime.to_rfc3339_opts(SecondsFormat::Secs, true);
    formatted.replace(":", "%3A")
}

fn get_api_snapshots(
    auth: &Environment,
    account_type: &str,
    limit: u8,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
) -> Result<Vec<database::Snapshot>, reqwest::Error> {
    let client = Client::new();

    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let shifted_start = if start == end {
        start - Duration::days(1)
    } else {
        start - Duration::seconds(1)
    };

    let shifted_end = end - Duration::seconds(1);

    println!(
        "Call Binance API (start: {}, end: {})",
        shifted_start, shifted_end
    );

    let params = format!(
        "type={}&limit={}&timestamp={}&startTime={}&endTime={}",
        account_type,
        limit,
        now,
        shifted_start.timestamp_millis(),
        shifted_end.timestamp_millis(),
    );

    let mut mac = HmacSha256::new_varkey(auth.binance_secret.as_bytes()).unwrap();
    mac.update(params.as_bytes());

    let hash_message = mac.finalize().into_bytes();
    let signature = hex::encode(&hash_message);

    let url = format!(
        "{}?{}&signature={}",
        BINANCE_API_BASE_URL, params, signature
    );
    let res = client
        .get(url)
        .header("X-MBX-APIKEY", auth.binance_key.to_owned())
        .send()?;

    let json = match res.text() {
        Ok(res) => res,
        Err(e) => return Err(e),
    };

    let obj: binance::RootObject = serde_json::from_str(&json).unwrap();

    let snapshots: Vec<database::Snapshot> = obj
        .snapshots
        .iter()
        .map(|snapshot| database::Snapshot {
            time: bson::DateTime(
                chrono::Utc.timestamp_millis(snapshot.update_time) + Duration::seconds(1),
            ),
            balances: snapshot
                .data
                .balances
                .iter()
                .filter(|balance| balance.free.parse::<f32>().unwrap() > 0.)
                .map(|balance| database::Balance {
                    asset: balance.asset.to_owned(),
                    amount: balance.free.parse::<f32>().unwrap(),
                })
                .collect(),
            total_asset_of_btc: snapshot.data.total_asset_of_btc.parse::<f32>().unwrap(),
        })
        .collect();

    println!("Got {} snapshots.", snapshots.len());

    Ok(snapshots)
}

fn get_api_history(
    auth: &Environment,
    ids: Vec<String>,
    convert: String,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
) -> Result<Vec<database::CurrencyHistory>, reqwest::Error> {
    let client = Client::new();

    println!("Call Nomics API (start: {}, end: {})", start, end);

    let params = format!(
        "ids={}&convert={}&start={}&end={}",
        ids.join(","),
        convert,
        get_uri_escaped_datetime(start),
        get_uri_escaped_datetime(end),
    );

    let url = format!("{}?key={}&{}", NOMICS_API_BASE_URL, auth.nomics_key, params);
    let res = client.get(url).send()?;

    let json = match res.text() {
        Ok(res) => res,
        Err(e) => return Err(e),
    };

    let obj: Vec<nomics::Sparkline> = serde_json::from_str(&json).unwrap();

    let history: Vec<database::CurrencyHistory> = obj
        .iter()
        .map(|history| {
            history
                .timestamps
                .iter()
                .enumerate()
                .map(|(i, timestamp)| database::CurrencyHistory {
                    asset: history.currency.to_owned(),
                    time: bson::DateTime(
                        DateTime::parse_from_rfc3339(timestamp)
                            .unwrap()
                            .with_timezone(&Utc),
                    ),
                    price: history.prices[i].parse::<f32>().unwrap(),
                })
                .collect::<Vec<database::CurrencyHistory>>()
        })
        .flatten()
        .collect();

    println!("Got {} currencies history.", history.len());

    Ok(history)
}

fn get_database_snapshots(
    database: &Database,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
) -> Vec<database::Snapshot> {
    let collection = database.collection("snapshots");

    // Sort with older first.
    let find_options = FindOptions::builder().sort(doc! {"time": 1}).build();

    let results: Vec<database::Snapshot> = collection
        .find(
            doc! {
                "time": {
                    "$gte": Bson::DateTime(start),
                    "$lte": Bson::DateTime(end),
                }
            },
            find_options,
        )
        .unwrap()
        .into_iter()
        .flatten()
        .map(|document| bson::from_bson(Bson::Document(document)).unwrap())
        .collect();

    results
}

fn push_database_snapshots(database: &Database, snapshots: Vec<database::Snapshot>) {
    let collection = database.collection("snapshots");

    let docs: Vec<bson::Document> = snapshots
        .iter()
        .map(|history| bson::ser::to_document(history).unwrap())
        .collect();

    collection.insert_many(docs, None).unwrap();
}

fn get_possessed_assets(database: &Database) -> Vec<String> {
    let collection = database.collection("snapshots");

    let mut assets: Vec<String> = collection
        .distinct("balances.asset", None, None)
        .unwrap()
        .iter()
        .map(|document| bson::from_bson(document.to_owned()).unwrap())
        .collect();

    let bitcoin_asset = "BTC".to_string();

    if !assets.contains(&bitcoin_asset) {
        assets.push(bitcoin_asset);
    }

    assets.sort_unstable();
    assets
}

fn push_database_history(database: &Database, price_history: Vec<database::CurrencyHistory>) {
    let collection = database.collection("history");

    let docs: Vec<bson::Document> = price_history
        .iter()
        .map(|history| bson::ser::to_document(history).unwrap())
        .collect();

    collection.insert_many(docs, None).unwrap();
}

fn get_computed_snapshots(
    database: &Database,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
) -> Vec<database::ComputedSnapshot> {
    let collection = database.collection("snapshots");

    let computed_snapshots: Vec<database::ComputedSnapshot> = collection.aggregate(vec![
        doc!{
            "$lookup": {
              "from": "history",
              "localField": "time",
              "foreignField": "time",
              "as": "prices"
            }
          },
          doc!{
            "$match": {
              "time": {
                "$gte": Bson::DateTime(start),
                "$lt": Bson::DateTime(end)
              }
            }
          },
          doc!{
            "$project": {
              "time": 1,
              "total_asset_of_btc": {
                "amount": "$total_asset_of_btc",
                "price": {
                  "$first": {
                    "$filter": {
                      "input": "$prices",
                      "as": "price",
                      "cond": {
                        "$eq": ["$$price.asset", "BTC"]
                      }
                    }
                  }
                }
              },
              "together": {
                "$map": {
                  "input": "$balances",
                  "as": "balance",
                  "in": {
                    "balance": "$$balance",
                    "price": {
                      "$first": {
                        "$filter": {
                          "input": "$prices",
                          "as": "price",
                          "cond": {
                            "$eq": ["$$price.asset", "$$balance.asset"]
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          },
          doc!{
            "$project": {
              "time": 1,
              "total_asset_of_btc": {
                "asset": "BTC",
                "amount": "$total_asset_of_btc.amount",
                "price": "$total_asset_of_btc.price.price",
                "value": {
                  "$multiply": ["$total_asset_of_btc.amount", "$total_asset_of_btc.price.price"]
                }
              },
              "balances": {
                "$map": {
                  "input": "$together",
                  "as": "pair",
                  "in": {
                    "asset": "$$pair.balance.asset",
                    "amount": "$$pair.balance.amount",
                    "price": "$$pair.price.price",
                    "value": {
                      "$multiply": ["$$pair.balance.amount", "$$pair.price.price"]
                    }
                  }
                }
              }
            }
          }
    ], None).unwrap()
    .into_iter()
    .flatten()
    .map(|document| bson::from_bson(Bson::Document(document)).unwrap())
    .collect();

    computed_snapshots
}

#[post("/api", format = "application/json", data = "<body>")]
fn api(body: Json<RequestBody>) -> content::Json<String> {
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

    let client = mongodb::sync::Client::with_uri_str(&mongodb_url).unwrap();
    let database = client.database("crypto-balance");

    let available_snapshots = get_database_snapshots(&database, start, end);
    let needed_timespans = get_timespans_to_retrieve(available_snapshots, start, end);

    if needed_timespans.is_empty() {
        let computed_snapshots = get_computed_snapshots(&database, start, end);
        let result = serde_json::to_string_pretty(&computed_snapshots).unwrap();
        return content::Json(result);
    }

    let split_by_30_days = split_all_timespans_max_days(&needed_timespans, 30);

    let snapshots = run_request_loop(&split_by_30_days, &mut |timespan: &TimeSpan| {
        get_api_snapshots(&env_variables, "SPOT", 30, timespan.start, timespan.end)
    });

    push_database_snapshots(&database, snapshots);

    let assets = get_possessed_assets(&database);
    let split_by_45_days = split_all_timespans_max_days(&needed_timespans, 45);

    let price_history = run_request_loop(&split_by_45_days, &mut |timespan: &TimeSpan| {
        get_api_history(
            &env_variables,
            assets.to_owned(),
            body.conversion.to_owned(),
            timespan.start,
            timespan.end,
        )
    });

    push_database_history(&database, price_history);

    let computed_snapshots = get_computed_snapshots(&database, start, end);
    let result = serde_json::to_string_pretty(&computed_snapshots).unwrap();
    content::Json(result)
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

fn main() {
    dotenv().ok();

    rocket::ignite()
        .mount("/", routes![index, api, files])
        .launch();
}
