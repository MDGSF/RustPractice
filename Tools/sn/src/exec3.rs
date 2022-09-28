use crate::exec::ExecInput;
use crate::utils;
use actix_web::{web, Responder};
use std::path::PathBuf;
use std::process::Stdio;
use tokio::process::Command;

/// curl -G --data-urlencode "cmd=ls -lh" http://192.168.2.101:8080/exec3
pub async fn exec3(input: web::Query<ExecInput>) -> impl Responder {
    log::info!("exec3: {:?}", input);
    tokio::spawn(async move {
        Command::new(utils::guess_shell())
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
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("exec3 command failed to start")
            .wait()
            .await
            .expect("exec3 command failed to run");
    });
    "Hello world!"
}
