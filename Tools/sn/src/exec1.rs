use actix_web::{web, Responder};
use serde::Deserialize;
use tokio::process::Command;
use crate::utils;

#[derive(Debug, Deserialize)]
pub struct Exec1Input {
    cmd: String,
}

/// curl -G --data-urlencode "cmd=ls -lh" http://192.168.2.101:8080/exec1
pub async fn exec1(input: web::Query<Exec1Input>) -> impl Responder {
    log::info!("exec1: {:?}", input);
    let output: std::process::Output = Command::new(utils::guess_shell())
        .arg("-c")
        .arg(&input.cmd)
        .output()
        .await
        .expect("exec1 command failed to run");
    output.stdout
}
