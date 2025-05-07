use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSampleCommand{
    pub device:String,
    pub sample:Vec<f32>,
    pub predict:i32
}