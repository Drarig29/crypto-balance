#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

use env::VarError;
use std::io;
use std::env;
use std::path::{Path, PathBuf};

use rocket::response::NamedFile;
use rocket::response::content;

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

#[get("/api")]
fn api() -> content::Json<String> {
    let env_variables = match get_env_vars() {
        Ok(res) => res,
        Err(err) => return content::Json(err.to_string())
    };

    content::Json(format!("{{ key: {key}, secret: {secret} }}", key=env_variables.key, secret=env_variables.secret))
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

fn main() {
    rocket::ignite().mount("/", routes![index, api, files]).launch();
}
