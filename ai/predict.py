import torch
from torch import nn
from model import HybridModel # 假设 HybridModel 在这里或已导入

# 假设 device 在全局定义
device = "mps"
MODEL_WEIGHTS_PATH = './best_model.pth' # <--- !!! 仔细检查并修改为正确的路径 !!!
INPUT_SEQ_LEN = 9
NUM_CLASSES = 3
CLASS_LABELS = {0: "静止状态", 1: "远离状态",2:"靠近状态"} # 假设 0 是静止, 1 是远离

def load_model_for_inference(model_path, model_class, **model_params):
    """加载模型权重并设置为评估模式"""
    print(f"Using device: {device}")

    # --- 1. 实例化模型 (在 try 之前) ---
    try:
        print("Instantiating model...")
        # 确保这里的参数与你的 HybridModel __init__ 匹配
        model = model_class(input_seq_len=model_params.get('input_seq_len', 9),
                            num_classes=model_params.get('num_classes', 2)
                            # 如果 HybridModel 需要其他参数，在这里添加:
                            # cnn_channels=model_params.get('cnn_channels'), # 等等
                           )
        print("Model instantiated.")
    except Exception as e:
        print(f"Error during model instantiation: {e}")
        return None, None # 实例化失败，直接返回 None

    # --- 2. 加载权重 ---
    try:
        print(f"Loading weights from {model_path}...")
        # 使用 weights_only=True 更安全
        model.load_state_dict(torch.load(model_path, map_location=device, weights_only=True))
        print("Weights loaded successfully.")

    except FileNotFoundError:
        print(f"Error: Model weights file not found at {model_path}")
        return None, None
    except Exception as e:
        print(f"Error loading model weights: {e}")
        # 即使加载权重失败，model 变量已存在，但应返回 None 表示加载不完整/失败
        return None, None

    # --- 3. 设置模式并返回 ---
    model.to(device)
    model.eval()
    print("Model loaded and set to evaluation mode.")
    return model, device

if __name__ == "__main__":
    # 定义模型参数 (必须与训练时保存的模型一致)
    model_params = {
        'input_seq_len': INPUT_SEQ_LEN,
        'num_classes': NUM_CLASSES,
        # 以下参数需要匹配你实际训练的 HybridModel
        # 例如，如果你的 HybridModel __init__ 不需要这些参数，可以移除
        # 'cnn_channels': [16, 32],
        # 'lstm_hidden_size': 32,
        # 'dropout_rate': 0.3
    }

    # 加载模型
    model, device = load_model_for_inference(MODEL_WEIGHTS_PATH, HybridModel, **model_params)

    # 查看模型结构
    print(model)
    total_params = sum(p.numel() for p in model.parameters())
    print(f"Total number of parameters: {total_params}")

    # !!! 添加检查 !!!
    if model is None:
        print("Model loading failed. Exiting.")
        exit()

    # 准备样本数据
    input_sample  = [
        [-46.0, -47.0, -47.0, -49.0, -49.0, -48.0, -51.0, -51.0, -51.0],# static
        [-50.0, -62.0, -71.0, -68.0, -65.0, -73.0, -72.0, -79.0, -75.0], # move
        [-75,-70,-60,-50,-40,-30,-20,-10,-10], # near
    ]


    # --- 4. Preprocessing (重要提醒) ---
    # !!! 如果训练时有标准化/归一化，这里必须应用相同的转换 !!!
    # scaler = load_scaler()
    # static_tensor_scaled = scaler.transform(static_tensor.cpu().numpy()) # 示例
    # static_tensor = torch.tensor(static_tensor_scaled, dtype=torch.float32).to(device)
    # (对 move_tensor 做同样处理)

    # --- 5. Prediction ---
    print("\n--- Predicting ---")
    try:
        with torch.no_grad(): # 在评估时不需要计算梯度
            for sample  in input_sample:
                r  = torch.tensor(sample, dtype=torch.float32).to(device).unsqueeze(0) # 添加 batch 维度
                model_output = model(r)
                # predict 
                pred_idx = model_output.argmax(dim=1).item()
                print(f"Input: {sample}")
                print(f"Prediction: {CLASS_LABELS.get(pred_idx, 'Unknown')} (Index: {pred_idx})")
                print(f"Raw Logits: {model_output.cpu().numpy()}")
    except Exception as e:
        print(f"An error occurred during prediction: {e}")
