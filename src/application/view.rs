use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct SampleView{
    pub id:i32,
    pub device:String,
    pub sample:Vec<f32>
}