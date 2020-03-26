#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

mod paste_id;

use std::fs::File;

use crate::paste_id::PasteId;

use rocket::Data;
use rocket::response::content;
use rocket::request::Form;
use std::io::{Write, Read};

const PASTE_ID_SIZE: usize = 4;
const UPLOAD_LIMIT: usize = 1024;
const HOST: &'static str = "http://localhost:8000";

#[get("/")]
fn index() -> content::Html<&'static str> {
    content::Html(include_str!("../static/index.html"))
}

#[derive(FromForm)]
pub struct PasteData {
    content: String,
}

fn truncate(buf: &[u8], limit: usize) -> &[u8] {
    let len = std::cmp::min(buf.len(), limit);
    &buf[..len]
}
#[post("/", data = "<paste>")]
fn upload(paste: Form<PasteData>) -> Result<String, std::io::Error> {
    let id = PasteId::new(PASTE_ID_SIZE);
    let mut file = File::create(format!("upload/{}", id))?;
    file.write_all(truncate(paste.content.as_bytes(), UPLOAD_LIMIT))?;
    Ok(format!("{host}/{id}", host = HOST, id = id))
}

#[get("/<id>")]
fn retrieve(id: PasteId) -> Option<File> {
    let filename = format!("upload/{id}", id = id);
    File::open(&filename).ok()
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, upload, retrieve])
        .launch();
}
