use crate::exec::ExecInput;
use crate::utils;
use actix_web::{web, Responder};
use std::path::PathBuf;
use tokio::process::Command;

/// curl -G --data-urlencode "cmd=ls -lh" http://192.168.2.101:8080/exec1
pub async fn exec2(input: web::Query<ExecInput>) -> impl Responder {
    log::info!("exec2: {:?}", input);
    let output: std::process::Output = Command::new(utils::guess_shell())
        .arg("-c")
        .arg(&input.cmd)
        .current_dir(
            input
                .working_directory
                .as_ref()
                .map_or(std::env::current_dir().unwrap(), |working_directory| {
                    PathBuf::from(&working_directory)
                }),
        )
        .output()
        .await
        .expect("exec2 command failed to run");
    output.stderr
}
