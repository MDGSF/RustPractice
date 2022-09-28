use crate::exec::ExecInput;
use crate::utils;
use actix_web::{web, Responder};
use std::path::PathBuf;
use tokio::process::Command;

/// ### example1
/// ```sh
/// curl -G \
///   --data-urlencode "cmd=ls -lh" \
///   http://192.168.2.101:8080/exec1
/// ```
///
/// ### example2
/// ```sh
/// curl -G \
///   --data-urlencode "cmd=ls -lh" \
///   --data-urlencode "working_directory=/home/huangjian" \
///   http://192.168.2.101:8080/exec1
/// ```
pub async fn exec1(input: web::Query<ExecInput>) -> impl Responder {
    log::info!("exec1: {:?}", input);
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
        .expect("exec1 command failed to run");
    output.stdout
}
