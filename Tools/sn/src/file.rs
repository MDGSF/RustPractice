use crate::utils;
use actix_multipart::Multipart;
use actix_web::{web, Error, HttpRequest, HttpResponse, Responder};
use futures_util::stream::StreamExt as _;
use futures_util::TryStreamExt as _;
use serde::{Deserialize, Serialize};
use std::io::Write;
use uuid::Uuid;

pub async fn view_upload() -> HttpResponse {
    let html = include_str!("../static/file.html");
    HttpResponse::Ok().body(html)
}

#[derive(Clone, Debug, Deserialize)]
pub struct UploadInput {
    pub directory: String,
}

pub async fn upload_file(
    input: web::Query<UploadInput>,
    mut payload: Multipart,
) -> Result<HttpResponse, Error> {
    if !utils::dir_exists(&input.directory) {
        let directory = input.directory.clone();
        web::block(move || std::fs::create_dir_all(&directory)).await??;
    }

    // iterate over multipart stream
    while let Some(mut field) = payload.try_next().await? {
        // A multipart/form-data stream has to contain `content_disposition`
        let content_disposition = field.content_disposition();

        let filename = content_disposition
            .get_filename()
            .map_or_else(|| Uuid::new_v4().to_string(), sanitize_filename::sanitize);
        let filepath = format!("{0}/{filename}", input.directory);
        log::info!("filepath: {}", filepath);

        // File::create is blocking operation, use threadpool
        let mut f = web::block(|| std::fs::File::create(filepath)).await??;

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.try_next().await? {
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.write_all(&chunk).map(|_| f)).await??;
        }
    }

    Ok(HttpResponse::Ok().into())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadBinaryInput {
    filename: String,
}

pub async fn upload_file_binary(
    input: web::Query<UploadBinaryInput>,
    mut body: web::Payload,
) -> Result<HttpResponse, Error> {
    let filepath = std::path::PathBuf::from(&input.filename.clone())
        .as_path()
        .to_owned();
    log::info!("upload file binary: {:?}", filepath);

    if let Some(parent) = filepath.parent() {
        if !utils::dir_exists(&parent.to_str().unwrap()) {
            let parent = parent.to_str().unwrap().to_owned();
            web::block(move || std::fs::create_dir_all(&parent)).await??;
        }
    }

    let mut file = web::block(|| std::fs::File::create(filepath)).await??;

    while let Some(item) = body.next().await {
        let item = item?;
        file = web::block(move || file.write_all(&item).map(|_| file)).await??;
    }
    web::block(move || file.sync_data()).await??;

    Ok(HttpResponse::Ok().into())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DownLoadInput {
    filename: String,
}

/// curl -o foo http://192.168.2.101:8080/download\?filename\=/home/huangjian/foo
pub async fn download_file(req: HttpRequest, input: web::Query<DownLoadInput>) -> impl Responder {
    let filepath = std::path::PathBuf::from(&input.filename.clone())
        .as_path()
        .to_owned();

    let file = actix_files::NamedFile::open_async(filepath).await.unwrap();

    file.into_response(&req)
}
