use crate::utils;
use actix_web::{web, Responder};
use tokio::process::Command;
use crate::exec::ExecInput;

/// curl -G --data-urlencode "cmd=ls -lh" http://192.168.2.101:8080/exec1
pub async fn exec2(input: web::Query<ExecInput>) -> impl Responder {
    log::info!("exec2: {:?}", input);
    let output: std::process::Output = Command::new(utils::guess_shell())
        .arg("-c")
        .arg(&input.cmd)
        .output()
        .await
        .expect("exec2 command failed to run");
    output.stderr
}
