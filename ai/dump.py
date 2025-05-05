import torch
import onnx
import onnxruntime as ort
import numpy as np
from model import HybridModel

def export_onnx(model, output_path="hybrid_model.onnx"):
    model.eval()
    
    dummy_input = torch.randn(1, 9, dtype=torch.float32)
    
    dynamic_axes = {
        'input': {0: 'batch_size', 1: 'sequence_length'},
        'output': {0: 'batch_size'}
    }
    
    torch.onnx.export(
        model,
        dummy_input,
        output_path,
        input_names=['input'],
        output_names=['output'],
        dynamic_axes=dynamic_axes,
        opset_version=12,
        do_constant_folding=True,
        verbose=True,
        export_params=True,
        training=torch.onnx.TrainingMode.EVAL,
    )
    print(f"模型已导出到 {output_path}")

def verify_onnx(model, onnx_path):
    onnx_model = onnx.load(onnx_path)
    onnx.checker.check_model(onnx_model)
    print("ONNX模型验证通过")
    
    sess_options = ort.SessionOptions()
    sess_options.graph_optimization_level = ort.GraphOptimizationLevel.ORT_ENABLE_ALL
    sess_options.intra_op_num_threads = 4
    
    providers = ['CPUExecutionProvider']
    if 'CUDAExecutionProvider' in ort.get_available_providers():
        providers.insert(0, 'CUDAExecutionProvider')
    
    ort_session = ort.InferenceSession(
        onnx_path, 
        providers=providers,
        sess_options=sess_options
    )
    
    # 添加真实场景测试数据
    static_sq = torch.tensor([-46.0, -47.0, -47.0, -49.0, -49.0, -48.0, -51.0, -51.0, -51.0], dtype=torch.float32)
    move_sq = torch.tensor([-50.0, -62.0, -71.0, -68.0, -65.0, -73.0, -72.0, -79.0, -75.0], dtype=torch.float32)
    
    test_cases = [
        # 随机测试用例
        torch.randn(1, 9, dtype=torch.float32),
        torch.randn(3, 9, dtype=torch.float32),
        torch.randn(10, 9, dtype=torch.float32),
        # 真实场景测试用例
        static_sq.reshape(1, 9),  # 静止场景
        move_sq.reshape(1, 9),    # 移动场景
    ]
    
    test_names = [
        "随机测试 (batch=1)",
        "随机测试 (batch=3)",
        "随机测试 (batch=10)",
        "静止场景测试",
        "移动场景测试"
    ]
    
    for i, (test_input, name) in enumerate(zip(test_cases, test_names)):
        print(f"\n测试用例 {i+1}: {name}")
        print(f"输入形状: {test_input.shape}")
        print(f"输入数据: {test_input}")
        
        # PyTorch推理
        model.eval()
        with torch.no_grad():
            torch_output = model(test_input)
            # 预测标签
            torch_pred = torch.argmax(torch_output, dim=1)
            print(f"预测标签: {torch_pred.tolist()}")  # 使用tolist()而不是item()
        
        # ONNX Runtime推理
        ort_inputs = {'input': test_input.numpy()}
        ort_output = ort_session.run(['output'], ort_inputs)[0]
        
        # 比较结果
        torch_output_np = torch_output.numpy()
        max_diff = np.abs(torch_output_np - ort_output).max()
        print(f"PyTorch输出: {torch_output_np}")
        print(f"ONNX输出: {ort_output}")
        print(f"最大差异: {max_diff}")
        
        if np.allclose(torch_output_np, ort_output, atol=1e-4):
            print("✓ 推理结果一致")
        else:
            print("⚠️ 警告：结果不一致")

if __name__ == "__main__":
    # 初始化模型
    model = HybridModel(input_seq_len=9, num_classes=3)
    
    # 如果有预训练权重，请取消下面的注释并指定正确的权重文件路径
    model.load_state_dict(torch.load('best_model.pth'))
    
    # 导出ONNX
    export_onnx(model, "hybrid_model.onnx")
    
    # 验证导出的模型
    verify_onnx(model, "hybrid_model.onnx")
