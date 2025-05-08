use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct SampleView{
    pub id:i32,
    pub device:String,
    pub sample:Vec<f32>,
    pub predict:i32,
    pub actual:i32,
    pub created_at:String,
}

#[derive(Debug, FromQueryResult)]
pub struct SampleViewRow {
    pub id: i32,
    pub device: String,
    pub created_at: String,
    pub sample: String,
    pub predict: i32,
    pub actual: i32,
}