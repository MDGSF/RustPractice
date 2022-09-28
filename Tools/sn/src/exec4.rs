use crate::exec::ExecInput;
use actix_web::{web, Responder};
use std::path::PathBuf;
use std::process::Stdio;
use tokio::process::Command;

/// curl -G --data-urlencode "cmd=reboot" http://192.168.2.101:8080/exec4
pub async fn exec4(input: web::Query<ExecInput>) -> impl Responder {
    log::info!("exec4: {:?}", input);
    tokio::spawn(async move {
        let status: std::process::ExitStatus = Command::new(&input.cmd)
            .current_dir(
                input
                    .working_directory
                    .as_ref()
                    .map_or(std::env::current_dir().unwrap(), |working_directory| {
                        PathBuf::from(&working_directory)
                    }),
            )
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .await
            .expect("exec4 command failed to run");
        log::info!("exec4: {:?}, {:?}", input, status);
    });
    "Hello world!"
}
