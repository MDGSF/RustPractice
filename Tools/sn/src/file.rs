use crate::utils;
use actix_multipart::Multipart;
use actix_web::{web, Error, HttpRequest, HttpResponse, Responder};
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
        std::fs::create_dir_all(&input.directory)?;
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
pub struct DownLoadInput {
    filename: String,
}

/// curl -o foo http://192.168.2.101:8080/download\?filename\=/home/huangjian/foo
pub async fn download_file(req: HttpRequest, input: web::Query<DownLoadInput>) -> impl Responder {
    let file_path = std::path::PathBuf::from(&input.filename.clone())
        .as_path()
        .to_owned();

    let file = actix_files::NamedFile::open_async(file_path).await.unwrap();

    file.into_response(&req)
}
