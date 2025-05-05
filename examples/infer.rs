use ort::session::{Session, builder::GraphOptimizationLevel};
use ndarray::{Array, ArrayD};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {  // 修改返回类型以处理多种错误
    // 创建会话
    let model = Session::builder()?
        .with_optimization_level(GraphOptimizationLevel::Level3)?
        .with_intra_threads(4)?
        .commit_from_file("/Users/fangf/opensource/d2l/rssi-detect/hybrid_model.onnx")?;

    let static_sq: Vec<f32> = vec![-46.0, -47.0, -47.0, -49.0, -49.0, -48.0, -51.0, -51.0, -51.0];
    let move_sq: Vec<f32> = vec![-50.0, -62.0, -71.0, -68.0, -65.0, -73.0, -72.0, -79.0, -75.0];
    let batch_size = 1;
    let input_seq_len = 9;

    println!("--- Inference for static_sq ---");
    // 创建输入数组
    let static_input_array = Array::from_shape_vec((batch_size, input_seq_len), static_sq)
        .map_err(|e| Box::new(e) as Box<dyn Error>)?;
    
    // 运行推理
    let outputs = model.run(ort::inputs! {
        "input" => static_input_array
    }?)?;

    // 提取输出
    let static_output = outputs["output"].try_extract_tensor::<f32>()?;
    println!("Static SQ Output: {:?}", static_output);
    process_output(&static_output.to_owned());  // 转换为owned类型

    println!("\n--- Inference for move_sq ---");
    let move_input_array = Array::from_shape_vec((batch_size, input_seq_len), move_sq)
        .map_err(|e| Box::new(e) as Box<dyn Error>)?;
    
    let outputs = model.run(ort::inputs! {
        "input" => move_input_array
    }?)?;

    let move_output = outputs["output"].try_extract_tensor::<f32>()?;
    println!("Move SQ Output: {:?}", move_output);
    process_output(&move_output.to_owned());  // 转换为owned类型

    Ok(())
}

fn process_output(output_array: &ArrayD<f32>) {
    if output_array.ndim() > 1 {
        for row in output_array.outer_iter() {
            let max_index = row.iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                .map(|(index, _)| index);
            if let Some(index) = max_index {
                println!("Predicted class index: {}", index);
            } else {
                println!("Could not determine predicted class for a row.");
            }
        }
    } else if let Some((index, _)) = output_array.iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap()) {
        println!("Predicted class index: {}", index);
    } else {
        println!("Could not determine predicted class.");
    }
}
