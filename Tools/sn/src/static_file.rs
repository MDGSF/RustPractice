use crate::utils;
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use serde::Serialize;
use std::fs::DirEntry;
use std::path::Path;
use tera::{Context, Tera};

#[derive(Serialize)]
struct OneEntry {
    filepath: String,
    name: String,
    is_file: bool,
    is_dir: bool,
}

impl From<DirEntry> for OneEntry {
    fn from(e: DirEntry) -> Self {
        let filepath = e.path().to_str().unwrap().to_owned();
        let name = e.file_name().into_string().unwrap();
        let is_file = e.file_type().map_or(false, |file_type| file_type.is_file());
        let is_dir = e.file_type().map_or(false, |file_type| file_type.is_dir());
        Self {
            filepath,
            name,
            is_file,
            is_dir,
        }
    }
}

impl From<&Path> for OneEntry {
    fn from(p: &Path) -> Self {
        let filepath = p.to_str().unwrap().to_owned();
        let name = p
            .file_name()
            .map_or("".to_string(), |v| v.to_str().unwrap().to_string());
        let is_file = p.is_file();
        let is_dir = p.is_dir();
        Self {
            filepath,
            name,
            is_file,
            is_dir,
        }
    }
}

#[derive(Serialize)]
struct EntryList {
    entries: Vec<OneEntry>,
}

impl EntryList {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct StaticInput {
    pub directory: String,
}

pub async fn view_static(input: web::Query<StaticInput>) -> HttpResponse {
    if !utils::dir_exists(&input.directory) {
        return HttpResponse::NotFound().body("directory not exists.");
    }

    let mut entries = EntryList::new();

    let current = Path::new(&input.directory);
    if let Some(parent) = current.parent() {
        let mut e: OneEntry = parent.into();
        e.name = "..".to_string();
        entries.entries.push(e);
    }

    for entry in current.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {
            let e: OneEntry = entry.into();
            entries.entries.push(e);
        }
    }

    //for entry in entries.entries.iter() {
    //    println!(
    //        "name:{}, is_file:{}, is_dir:{}, {}",
    //        entry.name, entry.is_file, entry.is_dir, entry.filepath
    //    );
    //}

    let v = serde_json::to_value(entries).unwrap();
    let context = Context::from_value(v).unwrap();

    let mut tpl: Tera = Default::default();
    let rendered = tpl
        .render_str(include_str!("../static/static_file.html"), &context)
        .unwrap();

    HttpResponse::Ok().body(rendered)
}
