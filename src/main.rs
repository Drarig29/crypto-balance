#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

use std::io;
use std::path::{Path, PathBuf};

use rocket::response::NamedFile;
use rocket::response::content;

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[get("/api")]
fn api() -> content::Json<&'static str> {
    content::Json("{ test: 'hello' }")
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

fn main() {
    rocket::ignite().mount("/", routes![index, api, files]).launch();
}
