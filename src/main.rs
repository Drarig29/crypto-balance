#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
extern crate hex;
extern crate hmac;
extern crate mongodb;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate sha2;

mod model;
use mongodb::bson::Document;
use model::binance;
use model::database;

use env::VarError;
use std::env;
use std::io;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use rocket::response::content;
use rocket::response::NamedFile;

use reqwest::blocking::Client;

use mongodb::bson::doc;

use hmac::{Hmac, Mac, NewMac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

struct BinanceAuth {
    key: String,
    secret: String,
}

const API_BASE_URL: &str = "https://api.binance.com/sapi/v1/accountSnapshot";

const ACCOUNT_TYPE_SPOT: &str = "SPOT";
const ACCOUNT_TYPE_MARGIN: &str = "MARGIN";
const ACCOUNT_TYPE_FUTURES: &str = "FUTURES";

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

fn get_env_vars() -> Result<BinanceAuth, VarError> {
    let key = match env::var("BINANCE_API_KEY") {
        Ok(res) => res,
        Err(e) => return Err(e),
    };
    let secret = match env::var("BINANCE_API_SECRET") {
        Ok(res) => res,
        Err(e) => return Err(e),
    };

    Ok(BinanceAuth { key, secret })
}

// TODO: make a struct with accountType, startTime, endTime and limit as properties

fn get_binance_snapshots(auth: BinanceAuth) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let params = format!("type=SPOT&limit={}&timestamp={}", 5, now);
    println!("params: {}", params);

    let mut mac = HmacSha256::new_varkey(auth.secret.as_bytes()).unwrap();
    mac.update(params.as_bytes());

    let hash_message = mac.finalize().into_bytes();
    let signature = hex::encode(&hash_message);
    println!("signature: {}", signature);

    let url = format!("{}?{}&signature={}", API_BASE_URL, params, signature);
    let res = client.get(url).header("X-MBX-APIKEY", auth.key).send()?;

    let json = match res.text() {
        Ok(res) => res,
        Err(e) => return Err(e),
    };

    let obj: binance::RootObject = serde_json::from_str(&json).unwrap();
    println!("Status: {}", obj.code);

    let new_obj = database::RootObject {
        snapshots: obj
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
            .collect(),
    };

    let result = serde_json::to_string_pretty(&new_obj).unwrap();

    let client = mongodb::sync::Client::with_uri_str("mongodb://root:example@127.0.0.1:27017").unwrap();
    let database = client.database("crypto-balance");
    let collection = database.collection("snapshots");

    let docs: Vec<Document> = new_obj.snapshots.iter().map(|snapshot| mongodb::bson::ser::to_document(snapshot).unwrap()).collect();

    collection.insert_many(docs, None).unwrap();

    Ok(result)
}

#[get("/api")]
fn api() -> content::Json<String> {
    let env_variables = match get_env_vars() {
        Ok(res) => res,
        Err(err) => return content::Json(err.to_string()),
    };

    let snapshots = get_binance_snapshots(env_variables);

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
