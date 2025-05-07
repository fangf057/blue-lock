pub type  SampleID = i32;

pub type SampleData = Vec<f32>;

#[derive(PartialEq, Debug,Eq,Default)]
pub enum ModelResult {
    Stationary,
    /// 物体正在远离检测器（距离增加）  
    MovingAway,
    /// 物体正在靠近检测器（距离减小）  
    MovingCloser,
    /// 物体状态未知（如遮挡或短暂丢失）  
    #[default]
    Unknown,
}

impl From<i32> for ModelResult {
    fn from(value: i32) -> Self {
        match value {
            0 => ModelResult::Stationary,
            1 => ModelResult::MovingAway,
            2 => ModelResult::MovingCloser,
            _ => ModelResult::Unknown,
        }
    }
}

impl Into<i32> for ModelResult {
    fn into(self) -> i32 {
        match self {
            ModelResult::Stationary => 0,
            ModelResult::MovingAway => 1,
            ModelResult::MovingCloser => 2,
            ModelResult::Unknown => 3,
        }
    }
}