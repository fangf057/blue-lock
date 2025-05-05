use ort::session::{builder::GraphOptimizationLevel, Session};
use ndarray::Array;
use std::{error::Error, fmt::Display, time::Instant};
use tokio::sync::{mpsc, oneshot};

pub enum DetectionState {
    /// 物体静止不动  
    Stationary,  
    /// 物体正在远离检测器（距离增加）  
    MovingAway,  
    /// 物体正在靠近检测器（距离减小）  
    MovingCloser,  
    /// 物体状态未知（如遮挡或短暂丢失）  
    Unknown,  
}


impl Display for DetectionState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Stationary => write!(f, "静止"),
            Self::MovingAway => write!(f, "远离"),
            Self::MovingCloser => write!(f, "靠近"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}


pub struct Model {
    session: Session,
}

pub struct InstantTimer{
    pub start_time: Instant,
    pub end_time: Instant,
}

impl Default for InstantTimer {
    fn default() -> Self {
        Self::new()
    }
}

impl InstantTimer {
    pub fn new() -> Self {
        Self{
            start_time: Instant::now(),
            end_time: Instant::now(),
        }
    }
}

impl Drop for InstantTimer {
    fn drop(&mut self) {
        self.end_time = Instant::now();
        let duration_ns = self.end_time.duration_since(self.start_time).as_nanos();
        let duration_ms = duration_ns as f64 / 1_000_000.0; // 转为毫秒
        println!("Elapsed: {:.6} ms", duration_ms); // 保留6位小数
    }
}



impl Model {
    pub fn new(model: &[u8]) -> Result<Self, Box<dyn Error>> {
        let session = Session::builder()?
            .with_optimization_level(GraphOptimizationLevel::Level3)?
            .with_intra_threads(4)?
            // .with_execution_providers(vec![CoreMLExecutionProvider::default().build()])?
            .commit_from_memory(model)?;

        Ok(Model { session })
    }

    pub fn inference(&self, data: Vec<f32>) -> Result<DetectionState, Box<dyn Error>> {
        let _tm   = InstantTimer::new();
        let input_array = Array::from_shape_vec((1, 9), data)?;
        
        let outputs = self.session.run(ort::inputs! {
            "input" => input_array
        }?)?;

        let output = outputs["output"].try_extract_tensor::<f32>()?;
        if let Some((index, _)) = output.outer_iter().next().unwrap()
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap()) 
        {
            Ok(match index {
                0 => DetectionState::Stationary,
                1 => DetectionState::MovingAway,
                2 => DetectionState::MovingCloser,
                _ => return Err("未知状态".into()),
            })
        } else {
            Err("无法确定预测类别".into())
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let detector = Model::new(include_bytes!("/Users/fangf/opensource/d2l/rssi-detect/hybrid_model.onnx"))?;
    // 创建数据接收channel和结果返回channel
    let (tx, mut rx) = mpsc::channel::<(Vec<f32>, oneshot::Sender<DetectionState>)>(10);

    // 启动推理处理循环
    tokio::spawn(async move {
        while let Some((data, resp_tx)) = rx.recv().await {
            match detector.inference(data) {
                Ok(state) => { let _ = resp_tx.send(state); },
                Err(e) => eprintln!("推理错误: {}", e),
            }
        }
    });

    // 示例使用
    let test_data = vec![-50.0, -62.0, -71.0, -68.0, -65.0, -73.0, -72.0, -79.0, -75.0];
    let (resp_tx, resp_rx) = oneshot::channel();
    tx.send((test_data.clone(), resp_tx)).await?;
    
    match resp_rx.await? {
        DetectionState::Stationary => println!("检测结果: 静止"),
        DetectionState::MovingCloser => println!("检测结果: 靠近"),
        DetectionState::MovingAway => println!("检测结果: 远离"),
        DetectionState::Unknown => println!("检测结果: 未知"),
    }

    Ok(())
}
