#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
extern crate chrono;
extern crate hex;
extern crate hmac;
extern crate mongodb;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate sha2;

mod model;
use model::binance;
use model::database;
use model::nomics;

use mongodb::bson::Document;

use env::VarError;
use std::env;
use std::io;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use rocket::response::content;
use rocket::response::NamedFile;

use reqwest::blocking::Client;

use mongodb::bson::doc;

use chrono::DateTime;

use hmac::{Hmac, Mac, NewMac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

struct Auth {
    binance_key: String,
    binance_secret: String,
    nomics_key: String,
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

// TODO: make a struct with accountType, startTime, endTime and limit as properties

fn get_wallet_snapshots(auth: &Auth) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let params = format!("type={}&limit={}&timestamp={}", "SPOT", 5, now);

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

    let new_obj: Vec<database::Snapshot> = obj
        .snapshots
        .iter()
        .map(|snapshot| database::Snapshot {
            time: snapshot.update_time,
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

    let result = serde_json::to_string_pretty(&new_obj).unwrap();

    let client = mongodb::sync::Client::with_uri_str(MONGODB_URL).unwrap();
    let database = client.database("crypto-balance");
    let collection = database.collection("snapshots");

    let docs: Vec<Document> = new_obj
        .iter()
        .map(|snapshot| mongodb::bson::ser::to_document(snapshot).unwrap())
        .collect();

    collection.insert_many(docs, None).unwrap();

    Ok(result)
}

fn get_price_history(auth: &Auth) -> Result<String, reqwest::Error> {
    let client = Client::new();

    let start = "2021-04-07T00:00:00Z";
    let params = format!(
        "ids={}&convert={}&start={}",
        "BTC,ETH,XRP",
        "EUR",
        start.replace(":", "%3A")
    );

    let url = format!("{}?key={}&{}", NOMICS_API_BASE_URL, auth.nomics_key, params);
    let res = client.get(url).send()?;

    let json = match res.text() {
        Ok(res) => res,
        Err(e) => return Err(e),
    };

    let obj: Vec<nomics::Sparkline> = serde_json::from_str(&json).unwrap();

    let new_obj: Vec<database::CurrencyHistory> = obj
        .iter()
        .map(|history| database::CurrencyHistory {
            asset: history.currency.to_owned(),
            history: history
                .timestamps
                .iter()
                .enumerate()
                .map(|(i, timestamp)| database::HistoricPrice {
                    time: DateTime::parse_from_rfc3339(timestamp).unwrap().timestamp(),
                    price: history.prices[i].parse::<f32>().unwrap(),
                })
                .collect(),
        })
        .collect();

    let result = serde_json::to_string_pretty(&new_obj).unwrap();

    Ok(result)
}

#[get("/api")]
fn api() -> content::Json<String> {
    let env_variables = match get_env_vars() {
        Ok(res) => res,
        Err(err) => return content::Json(err.to_string()),
    };

    let snapshots = get_wallet_snapshots(&env_variables);
    let price_history = get_price_history(&env_variables);

    println!("{}", price_history.unwrap());

    match snapshots {
        Ok(res) => content::Json(res),
        Err(err) => content::Json(err.to_string()),
    }
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
