#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
extern crate chrono;
extern crate hex;
extern crate hmac;
extern crate mongodb;
extern crate reqwest;
extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;
extern crate sha2;

mod model;
use bson::Bson;
use model::binance;
use model::database;
use model::nomics;

use mongodb::bson;
use mongodb::bson::doc;
use mongodb::options::FindOptions;

use env::VarError;
use std::env;
use std::io;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use rocket::response::{content, NamedFile};
use rocket_contrib::json::Json;

use serde::{Deserialize, Serialize};

use reqwest::blocking::Client;

use chrono::{DateTime, SecondsFormat, TimeZone, Utc, Duration};

use hmac::{Hmac, Mac, NewMac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

struct Auth {
    binance_key: String,
    binance_secret: String,
    nomics_key: String,
}

#[derive(Serialize, Deserialize)]
struct RequestBody {
    start: String,
    end: String,
}

const BINANCE_API_BASE_URL: &str = "https://api.binance.com/sapi/v1/accountSnapshot";
const NOMICS_API_BASE_URL: &str = "https://api.nomics.com/v1/currencies/sparkline";
const MONGODB_URL: &str = "mongodb://root:example@127.0.0.1:27017";

const ACCOUNT_TYPE_SPOT: &str = "SPOT";
const ACCOUNT_TYPE_MARGIN: &str = "MARGIN";
const ACCOUNT_TYPE_FUTURES: &str = "FUTURES";

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

fn get_env_vars() -> Result<Auth, VarError> {
    let binance_key = env::var("BINANCE_API_KEY")?;
    let binance_secret = env::var("BINANCE_API_SECRET")?;
    let nomics_key = env::var("NOMICS_API_KEY")?;

    Ok(Auth {
        binance_key,
        binance_secret,
        nomics_key,
    })
}

fn get_uri_escaped_datetime(datetime: DateTime<Utc>) -> String {
    let formatted = datetime.to_rfc3339_opts(SecondsFormat::Secs, true);
    formatted.replace(":", "%3A")
}

fn get_wallet_snapshots(
    auth: &Auth,
    account_type: &str,
    limit: u8,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
) -> Result<Vec<database::Snapshot>, reqwest::Error> {
    let client = Client::new();
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let params = format!(
        "type={}&limit={}&timestamp={}&startTime={}&endTime={}",
        account_type,
        limit,
        now,
        start_time.timestamp_millis(),
        end_time.timestamp_millis(),
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
    println!("Status: {}", obj.code);

    let snapshots: Vec<database::Snapshot> = obj
        .snapshots
        .iter()
        .map(|snapshot| database::Snapshot {
            time: bson::DateTime(chrono::Utc.timestamp_millis(snapshot.update_time) + Duration::seconds(1)),
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

    Ok(snapshots)
}

fn get_price_history(
    auth: &Auth,
    ids: Vec<String>,
    convert: String,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
) -> Result<Vec<database::CurrencyHistory>, reqwest::Error> {
    let client = Client::new();

    let params = format!(
        "ids={}&convert={}&start={}&end={}",
        ids.join(","),
        convert,
        get_uri_escaped_datetime(start_time),
        get_uri_escaped_datetime(end_time),
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

    Ok(history)
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

    // make database wallet snapshots request
    // make database price history request
    // find start and end of database data
    // compute needed timespans to fill in the blanks
    // - if no timespan
    //   - data is up to date
    //   - aggregate data and return
    // - if 1 or 2 timespans
    //   - do API requests to get the missing data
    //     - split requests in timespans of n days max
    //     - do as many requests as needed
    //   - upload to database
    //   - aggregate data and return

    /*let client = mongodb::sync::Client::with_uri_str(MONGODB_URL).unwrap();
    let database = client.database("crypto-balance");
    let collection = database.collection("snapshots");

    // Sort with older first.
    let find_options = FindOptions::builder().sort(doc! {"time": 1}).build();

    let results: Vec<database::Snapshot> = collection
        .find(
            doc! {
                "time": {
                    "$gte": Bson::DateTime(start),
                    "$lt": Bson::DateTime(end),
                }
            },
            find_options,
        )
        .unwrap()
        .into_iter()
        .flatten()
        .map(|document| bson::from_bson(Bson::Document(document)).unwrap())
        .collect();

    let database_start: DateTime<Utc> = From::from(results.first().unwrap().time);
    let database_end: DateTime<Utc> = From::from(results.last().unwrap().time);

    println!(
        "Database start: {}\nDatabase end: {}",
        database_start, database_end
    );*/
    let env_variables = match get_env_vars() {
        Ok(res) => res,
        Err(err) => return content::Json(err.to_string()),
    };

    let snapshots = get_wallet_snapshots(&env_variables, "SPOT", 5, start, end).unwrap();

    let client = mongodb::sync::Client::with_uri_str(MONGODB_URL).unwrap();
    let database = client.database("crypto-balance");
    let snapshots_collection = database.collection("snapshots");

    let docs: Vec<bson::Document> = snapshots
        .iter()
        .map(|history| bson::ser::to_document(history).unwrap())
        .collect();

    snapshots_collection.insert_many(docs, None).unwrap();

    let assets: Vec<String> = snapshots_collection
        .distinct("balances.asset", None, None)
        .unwrap()
        .iter()
        .map(|document| bson::from_bson(document.to_owned()).unwrap())
        .collect();

    let price_history =
        get_price_history(&env_variables, assets, "EUR".to_string(), start, end).unwrap();

    let history_collection = database.collection("history");

    let docs: Vec<bson::Document> = price_history
        .iter()
        .map(|history| bson::ser::to_document(history).unwrap())
        .collect();

    history_collection.insert_many(docs, None).unwrap();

    let result = serde_json::to_string_pretty(&price_history).unwrap();
    content::Json(result)
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, api, files])
        .launch();
}
