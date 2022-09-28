use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct ExecInput {
    pub cmd: String,
    pub working_directory: Option<String>,
}
