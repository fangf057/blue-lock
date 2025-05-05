use ort::session::{Session, builder::GraphOptimizationLevel};
use ndarray::Array;
use std::error::Error;
use tokio::sync::{mpsc, oneshot};

#[derive(Debug)]
enum DetectionState {
    Static,
    Moving,
}

struct RssiDetector {
    session: Session,
}

impl RssiDetector {
    fn new(model: &[u8]) -> Result<Self, Box<dyn Error>> {
        let session = Session::builder()?
            .with_optimization_level(GraphOptimizationLevel::Level3)?
            .with_intra_threads(4)?
            .commit_from_memory(model)?;

        Ok(RssiDetector { session })
    }

    fn inference(&self, data: Vec<f32>) -> Result<DetectionState, Box<dyn Error>> {
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
                0 => DetectionState::Static,
                1 => DetectionState::Moving,
                _ => return Err("未知状态".into()),
            })
        } else {
            Err("无法确定预测类别".into())
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let detector = RssiDetector::new(include_bytes!("/Users/fangf/opensource/d2l/rssi-detect/hybrid_model.onnx"))?;
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
        DetectionState::Static => println!("检测结果: 静止"),
        DetectionState::Moving => println!("检测结果: 移动"),
    }

    Ok(())
}
