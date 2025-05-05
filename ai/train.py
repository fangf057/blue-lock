import torch
import torch.nn as nn
import torch.optim as optim
from torch.utils.data import DataLoader, random_split
from sklearn.metrics import accuracy_score, confusion_matrix
import matplotlib.pyplot as plt
import numpy as np
from model import HybridModel
from gen import EmpiricalBluetoothDataset

# 超参数设置
config = {
    'batch_size': 64,
    'lr': 1e-5,
    'num_epochs': 100,
    'train_ratio': 0.8,
    'device': 'mps'
}

# 初始化模型和数据集
model = HybridModel(dropout_rate=0.5).to(config['device'])
dataset = EmpiricalBluetoothDataset(num_samples=10000)

# 划分训练集和测试集
train_size = int(config['train_ratio'] * len(dataset))
test_size = len(dataset) - train_size
train_dataset, test_dataset = random_split(dataset, [train_size, test_size])

train_loader = DataLoader(train_dataset, batch_size=config['batch_size'], shuffle=True)
test_loader = DataLoader(test_dataset, batch_size=config['batch_size'])

# 损失函数和优化器
criterion = nn.CrossEntropyLoss()
optimizer = optim.Adam(model.parameters(), lr=config['lr'])

# 训练和验证函数
def train_epoch(model, dataloader):
    model.train()
    total_loss = 0
    all_preds, all_labels = [], []
    
    for inputs, labels in dataloader:
        inputs = inputs.to(config['device'])
        labels = labels.long().to(config['device'])
        
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

def validate(model, dataloader):
    model.eval()
    total_loss = 0
    all_preds, all_labels = [], []
    
    with torch.no_grad():
        for inputs, labels in dataloader:
            inputs = inputs.to(config['device'])
            labels = labels.long().to(config['device'])
            
            outputs = model(inputs)
            loss = criterion(outputs, labels)
            
            total_loss += loss.item()
            _, preds = torch.max(outputs, 1)
            all_preds.extend(preds.cpu().numpy())
            all_labels.extend(labels.cpu().numpy())
    
    acc = accuracy_score(all_labels, all_preds)
    avg_loss = total_loss / len(dataloader)
    return avg_loss, acc, all_labels, all_preds

# 训练循环
train_losses, val_losses = [], []
train_accs, val_accs = [], []

best_val_acc = 0.0

for epoch in range(config['num_epochs']):
    train_loss, train_acc = train_epoch(model, train_loader)
    val_loss, val_acc, _, _ = validate(model, test_loader)
    
    train_losses.append(train_loss)
    val_losses.append(val_loss)
    train_accs.append(train_acc)
    val_accs.append(val_acc)
    
    # 保存最佳模型
    if val_acc > best_val_acc:
        best_val_acc = val_acc
        torch.save(model.state_dict(), 'best_model.pth')
    
    print(f'Epoch {epoch+1}/{config["num_epochs"]} - '
          f'Train Loss: {train_loss:.4f}, Train Acc: {train_acc:.4f} - '
          f'Val Loss: {val_loss:.4f}, Val Acc: {val_acc:.4f}')

# 加载最佳模型
model.load_state_dict(torch.load('best_model.pth'))
model = model.to(config['device'])

# 最终测试
_, test_acc, y_true, y_pred = validate(model, test_loader)
conf_mat = confusion_matrix(y_true, y_pred)

print(f'\nFinal Test Accuracy: {test_acc:.4f}')
print('Confusion Matrix:')
print(conf_mat)

# 可视化训练过程
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

# 样本预测可视化
def plot_predictions(model, dataloader, num_samples=6):
    model.eval()
    samples = []
    
    with torch.no_grad():
        for inputs, labels in dataloader:
            inputs = inputs.to(config['device'])
            outputs = model(inputs)
            _, preds = torch.max(outputs, 1)
            
            for i in range(min(num_samples, len(inputs))):
                sample = {
                    'input': inputs[i].cpu().numpy(),
                    'label': labels[i].item(),
                    'pred': preds[i].item()
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
        
        pred_color = 'darkred' if sample['pred'] == 1 else 'darkblue'
        true_color = 'red' if sample['label'] == 1 else 'blue'
        
        plt.title(f"Pred: {'Moving' if sample['pred'] == 1 else 'Static'}\n"
                 f"True: {'Moving' if sample['label'] == 1 else 'Static'}")
        plt.ylim(-90, -40)
        plt.grid(True)
    
    plt.tight_layout()
    plt.show()

plot_predictions(model, test_loader)
