use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InputContext {
  pub board: Vec<Vec<usize>>,
  pub fixed: usize,
  pub size: usize,
  pub stage: usize,
}
