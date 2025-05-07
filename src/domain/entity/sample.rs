use crate::domain::value_objects::{ModelResult, SampleData, SampleID};

#[derive(Default)]
pub struct SampleAggregate{
    pub id:SampleID,
    pub device:String,
    pub data:SampleData,
    pub predict:ModelResult,
    pub actual:ModelResult
}

impl SampleAggregate{
    
    pub fn new (id:SampleID, device:String, data:SampleData, predict:ModelResult, actual:ModelResult)->Self{
        Self{
            id,
            device,
            data,
            predict,
            actual
        }
    }
    pub fn add_sample(&mut self, s:SampleData){
        self.data = s;
    }

    pub fn change_actual(&mut self, a:ModelResult){
        self.actual = a;
    }
    pub fn is_correct(&self)->bool{
        self.predict == self.actual
    }
}


