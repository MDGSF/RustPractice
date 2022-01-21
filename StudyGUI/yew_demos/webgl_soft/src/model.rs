use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Model {
    pub vertices: Vec<f32>,
    pub colors: Vec<f32>,
    pub indices: Vec<u16>,
}

impl Model {
    pub fn new_from_json(data: &str) -> Self {
        serde_json::from_str(&data).unwrap()
    }
}

pub const MODEL_CUBE: &'static str = r#"
{
  "vertices": [
    -1,-1,-1,
     1,-1,-1,
     1, 1,-1,
    -1, 1,-1,
    -1,-1, 1,
     1,-1, 1,
     1, 1, 1,
    -1, 1, 1,
    -1,-1,-1,
    -1, 1,-1,
    -1, 1, 1,
    -1,-1, 1,
     1,-1,-1,
     1, 1,-1,
     1, 1, 1,
     1,-1, 1,
    -1,-1,-1,
    -1,-1, 1,
     1,-1, 1,
     1,-1,-1,
    -1, 1,-1,
    -1, 1, 1,
     1, 1, 1,
     1, 1,-1
  ],
  "colors": [
    5,3,7, 5,3,7, 5,3,7, 5,3,7,
    1,1,3, 1,1,3, 1,1,3, 1,1,3,
    0,0,1, 0,0,1, 0,0,1, 0,0,1,
    1,0,0, 1,0,0, 1,0,0, 1,0,0,
    1,1,0, 1,1,0, 1,1,0, 1,1,0,
    0,1,0, 0,1,0, 0,1,0, 0,1,0
  ],
  "indices": [
    0,1,2, 0,2,3, 4,5,6, 4,6,7,
    8,9,10, 8,10,11, 12,13,14, 12,14,15,
    16,17,18, 16,18,19, 20,21,22, 20,22,23 
  ]
}
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_new() {
        let _m = Model::new_from_json(MODEL_CUBE);
    }
}
