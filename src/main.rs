#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
extern crate reqwest;
extern crate hmac;
extern crate sha2;
extern crate hex;

use env::VarError;
use std::io;
use std::env;
use std::time::SystemTime;
use std::path::{Path, PathBuf};

use rocket::response::NamedFile;
use rocket::response::content;

use reqwest::blocking::{Client};

use sha2::Sha256;
use hmac::{Hmac, Mac, NewMac};

// Create alias for HMAC-SHA256
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

    Ok(BinanceAuth {
        key,
        secret,
    })
}

// TODO: make a struct with accountType, startTime, endTime and limit as properties

fn get_binance_snapshots(auth: BinanceAuth) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis();

    let params = format!("type=SPOT&limit={}&timestamp={}", 5, now);
    println!("params: {}", params);

    let mut mac = HmacSha256::new_varkey(auth.secret.as_bytes()).unwrap();
    mac.update(params.as_bytes());

    let hash_message = mac.finalize().into_bytes();
    let signature = hex::encode(&hash_message);
    println!("signature: {}", signature);

    let url = format!("{}?{}&signature={}", API_BASE_URL, params, signature);
    let res = client.get(url).header("X-MBX-APIKEY", auth.key).send()?;

    match res.text() {
        Ok(res) => Ok(res),
        Err(e) => Err(e),
    }
}

#[get("/api")]
fn api() -> content::Json<String> {
    let env_variables = match get_env_vars() {
        Ok(res) => res,
        Err(err) => return content::Json(err.to_string())
    };

    let snapshots = get_binance_snapshots(env_variables);

    match snapshots {
        Ok(res) => content::Json(res),
        Err(err) => content::Json(err.to_string())
    }
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

fn main() {
    rocket::ignite().mount("/", routes![index, api, files]).launch();
}
