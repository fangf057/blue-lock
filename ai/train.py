import os
import random
import torch
import torch.nn as nn
import torch.optim as optim
from torch.utils.data import DataLoader, random_split
from sklearn.metrics import accuracy_score, confusion_matrix
import matplotlib.pyplot as plt
import numpy as np
from model import HybridModel
from gen import EmpiricalBluetoothDataset
from tqdm import tqdm

def set_seed(seed=2024):
    """确保结果可复现"""
    random.seed(seed)
    np.random.seed(seed)
    torch.manual_seed(seed)
    torch.cuda.manual_seed_all(seed)
    torch.backends.cudnn.deterministic = True
    torch.backends.cudnn.benchmark = False

def get_device():
    """自动选择设备"""
    if torch.backends.mps.is_available():
        device = torch.device('mps')
    elif torch.cuda.is_available():
        device = torch.device('cuda')
    else:
        device = torch.device('cpu')
    print(f'Using device: {device}')
    return device

# ============================== 配置 ==============================
class Config:
    batch_size = 128
    lr = 1e-4
    num_epochs = 100
    train_ratio = 0.8
    num_samples = 10000
    dropout_rate = 0.5
    best_model_path = 'best_model.pth'
    seed = 2024

# ============================== 相关函数 ==============================

def ensure_float_tensor(x, device):
    """输入tensor转float并放到device"""
    if not torch.is_floating_point(x):
        x = x.float()
    return x.to(device)

def ensure_int_tensor(x, device):
    """标签tensor转long并放到device"""
    return x.long().to(device)

def train_epoch(model, dataloader, optimizer, criterion, device):
    model.train()
    total_loss, all_preds, all_labels = 0, [], []
    for inputs, labels in dataloader:
        inputs = ensure_float_tensor(inputs, device)
        labels = ensure_int_tensor(labels, device)
        optimizer.zero_grad()
        outputs = model(inputs)
        loss = criterion(outputs, labels)
        loss.backward()
        optimizer.step()
        total_loss += loss.item()
        _, preds = torch.max(outputs, 1)
        all_preds.extend(preds.cpu().numpy())
        all_labels.extend(labels.cpu().numpy())
    acc = accuracy_score(all_labels, all_preds)
    avg_loss = total_loss / len(dataloader)
    return avg_loss, acc

def validate(model, dataloader, criterion, device):
    model.eval()
    total_loss, all_preds, all_labels = 0, [], []
    with torch.no_grad():
        for inputs, labels in dataloader:
            inputs = ensure_float_tensor(inputs, device)
            labels = ensure_int_tensor(labels, device)
            outputs = model(inputs)
            loss = criterion(outputs, labels)
            total_loss += loss.item()
            _, preds = torch.max(outputs, 1)
            all_preds.extend(preds.cpu().numpy())
            all_labels.extend(labels.cpu().numpy())
    acc = accuracy_score(all_labels, all_preds)
    avg_loss = total_loss / len(dataloader)
    return avg_loss, acc, all_labels, all_preds

def save_model(model, path):
    torch.save(model.state_dict(), path)
    print(f'Model saved to {path}')

def load_best_model(model, path, device):
    if os.path.exists(path):
        model.load_state_dict(torch.load(path, map_location=device))
        model.to(device)
        print(f'Loaded best model from {path}')
    else:
        raise FileNotFoundError(f"Model file {path} not found!")
    return model

def plot_metrics(train_losses, val_losses, train_accs, val_accs):
    plt.figure(figsize=(12, 5))
    plt.subplot(1, 2, 1)
    plt.plot(train_losses, label='Train Loss')
    plt.plot(val_losses, label='Val Loss')
    plt.xlabel('Epoch')
    plt.ylabel('Loss')
    plt.legend()
    plt.subplot(1, 2, 2)
    plt.plot(train_accs, label='Train Acc')
    plt.plot(val_accs, label='Val Acc')
    plt.xlabel('Epoch')
    plt.ylabel('Accuracy')
    plt.legend()
    plt.tight_layout()
    plt.show()

def plot_predictions(model, dataloader, device, num_samples=6):
    model.eval()
    samples = []
    with torch.no_grad():
        for inputs, labels in dataloader:
            inputs = ensure_float_tensor(inputs, device)
            labels = labels.cpu()
            outputs = model(inputs)
            _, preds = torch.max(outputs, 1)
            for i in range(min(num_samples, len(inputs))):
                sample = {
                    'input': inputs[i].cpu().numpy(),
                    'label': labels[i].item(),
                    'pred': preds[i].cpu().item()
                }
                samples.append(sample)
                if len(samples) >= num_samples:
                    break
            if len(samples) >= num_samples:
                break
    plt.figure(figsize=(12, 8))
    for i, sample in enumerate(samples):
        plt.subplot(2, 3, i+1)
        color = 'red' if sample['label'] == 1 else 'blue'
        plt.plot(sample['input'], 'o-', color=color)
        plt.title(f"Pred: {'Moving' if sample['pred'] == 1 else 'Static'}\n"
                  f"True: {'Moving' if sample['label'] == 1 else 'Static'}")
        plt.ylim(-90, -40)
        plt.grid(True)
    plt.tight_layout()
    plt.show()

# ============================== 主流程 ==============================
def main():
    set_seed(Config.seed)
    device = get_device()
    # dataset & dataloader
    dataset = EmpiricalBluetoothDataset(num_samples=Config.num_samples)
    train_size = int(Config.train_ratio * len(dataset))
    test_size = len(dataset) - train_size
    train_dataset, test_dataset = random_split(dataset, [train_size, test_size])
    train_loader = DataLoader(train_dataset, batch_size=Config.batch_size, shuffle=True, num_workers=0)
    test_loader = DataLoader(test_dataset, batch_size=Config.batch_size)
    # 模型、优化器、损失
    model = HybridModel(dropout_rate=Config.dropout_rate).to(device)
    criterion = nn.CrossEntropyLoss()
    optimizer = optim.Adam(model.parameters(), lr=Config.lr)
    train_losses, val_losses, train_accs, val_accs = [], [], [], []
    best_val_acc = 0.0

    for epoch in tqdm(range(Config.num_epochs)):
        train_loss, train_acc = train_epoch(model, train_loader, optimizer, criterion, device)
        val_loss, val_acc, _, _ = validate(model, test_loader, criterion, device)
        train_losses.append(train_loss)
        val_losses.append(val_loss)
        train_accs.append(train_acc)
        val_accs.append(val_acc)
        # 保存最优模型
        if val_acc > best_val_acc:
            best_val_acc = val_acc
            save_model(model, Config.best_model_path)
        print(f'Epoch {epoch+1}/{Config.num_epochs} - '
              f'Train Loss: {train_loss:.4f}, Train Acc: {train_acc:.4f} - '
              f'Val Loss: {val_loss:.4f}, Val Acc: {val_acc:.4f}')
        # MPS可定期清缓存
        if device.type == 'mps':
            torch.mps.empty_cache()

    # 载入最佳模型
    model = load_best_model(model, Config.best_model_path, device)
    # 测试准确率
    _, test_acc, y_true, y_pred = validate(model, test_loader, criterion, device)
    conf_mat = confusion_matrix(y_true, y_pred)
    print(f'\nFinal Test Accuracy: {test_acc:.4f}')
    print('Confusion Matrix:')
    print(conf_mat)
    # 可视化
    plot_metrics(train_losses, val_losses, train_accs, val_accs)
    plot_predictions(model, test_loader, device)

if __name__ == '__main__':
    main()