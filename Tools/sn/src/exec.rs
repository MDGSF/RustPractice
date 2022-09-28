use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ExecInput {
    pub cmd: String,
    pub working_directory: Option<String>,
}
