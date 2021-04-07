#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
extern crate reqwest;

use env::VarError;
use std::io;
use std::env;
use std::path::{Path, PathBuf};
use std::collections::HashMap;

use rocket::response::NamedFile;
use rocket::response::content;

use reqwest::{Client, Method};

struct BinanceAuth {
    key: String,
    secret: String,
}

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

fn get_binance_snapshots(auth: BinanceAuth) -> Result<String, reqwest::Error> {
    println!("Key: {}, Secret: {}", auth.key, auth.secret);
    
    match reqwest::blocking::get("https://api.binance.com/sapi/v1/accountSnapshot")?.text() {
        Ok(res) => return Ok(res),
        Err(e) => return Err(e),
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
