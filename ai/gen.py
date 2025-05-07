import torch
import numpy as np
import matplotlib.pyplot as plt
from torch.utils.data import Dataset, DataLoader
from torch import nn
import torch.nn.functional as F

import torch
import numpy as np
from torch.utils.data import Dataset

# 真实数据样本
real_static_samples = [
    np.array([-46.0, -47.0, -47.0, -49.0, -49.0, -48.0, -51.0, -51.0, -51.0]),
    np.array([-49.0, -49.0, -53.0, -53.0, -61.0, -53.0, -61.0, -49.0, -51.0]),
    np.array([-49.0, -44.0, -46.0, -47.0, -48.0, -45.0, -45.0, -48.0, -48.0]),
]
    
real_moving_samples = [
    np.array([-50.0, -62.0, -71.0, -68.0, -65.0, -73.0, -72.0, -79.0, -75.0]),
    np.array([-48.0, -49.0, -49.0, -54.0, -60.0, -61.0, -54.0, -61.0, -72.0]),
    np.array([-56.0, -49.0, -49.0, -67.0, -75.0, -75.0, -73.0, -76.0, -82.0]),
    np.array([-58.0, -57.0, -61.0, -61.0, -58.0, -60.0, -57.0, -57.0, -84.0]),
    np.array([-61.0, -61.0, -58.0, -60.0, -57.0, -57.0, -84.0, -84.0, -73.0]),
    np.array([-58.0, -60.0, -57.0, -57.0, -84.0, -84.0, -73.0, -75.0, -75.0])
]

real_approaching_samples = [
    np.array([-75.0, -72.0, -68.0, -65.0, -62.0, -58.0, -55.0, -52.0, -50.0]),  # 标准渐进靠近
    np.array([-82.0, -76.0, -73.0, -70.0, -65.0, -65.0, -60.0, -58.0, -56.0]),  # 快速靠近后稳定
    np.array([-72.0, -72.0, -68.0, -65.0, -60.0, -55.0, -52.0, -50.0, -48.0]),   # 开始稳定后靠近
    np.array([-84.0, -84.0, -73.0, -75.0, -75.0, -77.0, -53.0, -56.0, -57.0]),
    np.array([-84.0, -73.0, -75.0, -75.0, -77.0, -53.0, -56.0, -57.0, -70.0])
]

class EmpiricalBluetoothDataset(Dataset):
    def __init__(self, num_samples=1000, real_static_samples=real_static_samples, 
                 real_moving_samples=real_moving_samples,
                 real_approaching_samples=real_approaching_samples):
        np.random.seed(42)
        num_samples = (num_samples // 3) * 3  # 确保能被3整除
        samples_per_class = num_samples // 3
        
        self.stable_patterns = real_static_samples
        self.moving_references = real_moving_samples
        self.approaching_references = real_approaching_samples
        
        # ===== 静止状态生成 =====
        static_data = []
        for _ in range(samples_per_class):
            # 1. 随机选择信号强度区间
            signal_range = np.random.choice(['strong', 'medium', 'weak'])
            if signal_range == 'strong':
                base_mean = np.random.uniform(-45, -55)
            elif signal_range == 'medium':
                base_mean = np.random.uniform(-55, -65)
            else:
                base_mean = np.random.uniform(-65, -75)
            
            # 2. 生成新样本
            new_sample = np.zeros(9)
            new_sample[0] = base_mean + np.random.normal(0, 2)
            
            # 3. 模拟不同类型的静止波动
            wave_type = np.random.choice(['stable', 'fluctuating', 'step'])
            
            if wave_type == 'stable':
                # 稳定波动
                for i in range(1, 9):
                    new_sample[i] = new_sample[i-1] + np.random.normal(0, 1.5)
            
            elif wave_type == 'fluctuating':
                # 较大波动但维持在一定范围
                for i in range(1, 9):
                    if np.random.rand() < 0.7:  # 70%概率小波动
                        new_sample[i] = new_sample[i-1] + np.random.normal(0, 2)
                    else:  # 30%概率大波动
                        new_sample[i] = base_mean + np.random.normal(0, 4)
            
            else:  # step
                # 阶跃式变化
                step_point = np.random.randint(3, 6)
                step_size = np.random.uniform(5, 10) * np.random.choice([-1, 1])
                for i in range(1, 9):
                    if i < step_point:
                        new_sample[i] = new_sample[i-1] + np.random.normal(0, 1.5)
                    else:
                        new_sample[i] = new_sample[i-1] + step_size + np.random.normal(0, 1.5)
                        step_size = 0  # 只在步进点发生跳变
            
            # 4. 确保信号强度在合理范围内
            max_allowed = -40
            min_allowed = -80
            if np.any(new_sample > max_allowed) or np.any(new_sample < min_allowed):
                # 如果超出范围，进行整体平移
                if np.max(new_sample) > max_allowed:
                    new_sample = new_sample - (np.max(new_sample) - max_allowed) - 2
                if np.min(new_sample) < min_allowed:
                    new_sample = new_sample - (np.min(new_sample) - min_allowed) + 2
            
            static_data.append(new_sample)

        # ===== 移动状态生成 =====
        moving_data = []
        for _ in range(samples_per_class):
            start_range = np.random.choice(['strong', 'medium', 'weak'])
            if start_range == 'strong':
                start_rssi = np.random.uniform(-45, -55)
            elif start_range == 'medium':
                start_rssi = np.random.uniform(-55, -65)
            else:
                start_rssi = np.random.uniform(-65, -75)
            
            # 生成递减序列
            new_sample = np.zeros(9)
            new_sample[0] = start_rssi
            
            # 累积衰减
            total_decay = np.random.uniform(15, 25)  # 总衰减幅度
            decay_pattern = np.random.choice(['gradual', 'sudden'])
            
            if decay_pattern == 'gradual':
                # 渐进式衰减
                steps = np.linspace(0, total_decay, 9)
                for i in range(9):
                    noise = np.random.normal(0, 1.0)  # 减小噪声
                    new_sample[i] = start_rssi - steps[i] + noise
            else:
                # 突变式衰减
                change_point = np.random.randint(3, 6)
                for i in range(9):
                    noise = np.random.normal(0, 1.0)
                    if i < change_point:
                        new_sample[i] = start_rssi + noise
                    else:
                        new_sample[i] = start_rssi - total_decay + noise
            
            # 确保不超出合理范围
            new_sample = np.clip(new_sample, -80, -40)
            moving_data.append(new_sample)

        # ===== 靠近状态生成 =====
        approaching_data = []
        for _ in range(samples_per_class):
            # 从弱信号开始
            start_range = np.random.choice(['weak', 'very_weak'])
            if start_range == 'weak':
                start_rssi = np.random.uniform(-70, -75)
            else:
                start_rssi = np.random.uniform(-75, -80)
            
            # 生成递增序列
            new_sample = np.zeros(9)
            new_sample[0] = start_rssi
            
            # 信号增强
            total_increase = np.random.uniform(15, 25)  # 总增强幅度
            approach_pattern = np.random.choice(['gradual', 'sudden'])
            
            if approach_pattern == 'gradual':
                # 渐进式靠近
                steps = np.linspace(0, total_increase, 9)
                for i in range(9):
                    noise = np.random.normal(0, 1.0)
                    new_sample[i] = start_rssi + steps[i] + noise
            else:
                # 突变式靠近
                change_point = np.random.randint(3, 6)
                for i in range(9):
                    noise = np.random.normal(0, 1.0)
                    if i < change_point:
                        new_sample[i] = start_rssi + noise
                    else:
                        new_sample[i] = start_rssi + total_increase + noise
            
            # 确保不超出合理范围
            new_sample = np.clip(new_sample, -80, -40)
            approaching_data.append(new_sample)
        
        # 合并数据集
        self.X = np.vstack([static_data, moving_data, approaching_data]).astype(np.float32)
        self.y = np.hstack([np.zeros(samples_per_class),  # 静止: 0
                           np.ones(samples_per_class),    # 远离: 1
                           2 * np.ones(samples_per_class)])  # 靠近: 2
        self.X = torch.tensor(self.X)
        self.y = torch.tensor(self.y)

    def __len__(self):
        return len(self.X)
    
    def __getitem__(self, idx):
        return self.X[idx], self.y[idx]

# 在工具函数部分更新analyze_real_data函数
def analyze_real_data(real_static_samples, real_moving_samples, real_approaching_samples):
    """分析真实数据特征"""
    print("\n=== 真实数据分析结果 ===")
    
    # 分析静止样本
    print("静止样本分析:")
    for i, sample in enumerate(real_static_samples):
        avg = np.mean(sample)
        std = np.std(sample)
        peak_to_peak = np.ptp(sample)
        print(f"样本{i+1}: 均值={avg:.1f}dBm, 标准差={std:.1f}, 峰峰值={peak_to_peak:.1f}")
        
        diffs = np.diff(sample)
        if np.any(diffs < -5):
            drop_pos = np.where(diffs < -5)[0] + 1
            print(f"  检测到突降点：位置={drop_pos}, 突降幅度={sample[drop_pos[0]-1]-sample[drop_pos[0]]:.1f}dBm")
    
    # 分析远离样本
    print("\n远离样本分析:")
    for i, sample in enumerate(real_moving_samples):
        avg = np.mean(sample)
        std = np.std(sample)
        total_drop = sample[0] - sample[-1]
        print(f"样本{i+1}: 总衰减={total_drop:.1f}dBm, 均值={avg:.1f}dBm, 标准差={std:.1f}")
        print("  各点衰减:", [f"{sample[i]}-{sample[i+1]}={sample[i]-sample[i+1]:.1f}dBm" 
                          for i in range(len(sample)-1)])
    
    # 分析靠近样本
    print("\n靠近样本分析:")
    for i, sample in enumerate(real_approaching_samples):
        avg = np.mean(sample)
        std = np.std(sample)
        total_increase = sample[-1] - sample[0]
        print(f"样本{i+1}: 总增强={total_increase:.1f}dBm, 均值={avg:.1f}dBm, 标准差={std:.1f}")
        print("  各点增强:", [f"{sample[i+1]}-{sample[i]}={sample[i+1]-sample[i]:.1f}dBm" 
                          for i in range(len(sample)-1)])

def plot_comparison(dataset, real_static_samples, real_moving_samples, real_approaching_samples):
    plt.figure(figsize=(18, 20))
    
    # 真实静止数据
    for i in range(3):
        ax = plt.subplot(5, 3, i+1)
        ax.plot(real_static_samples[i], 'o-', color='blue')
        ax.set_title(f"真实静止样本{i+1}\n(峰峰值={np.ptp(real_static_samples[i]):.1f}dBm)")
        ax.set_ylim(-85, -40)
        ax.grid(True)
    
    # 真实远离数据
    for i in range(3):
        ax = plt.subplot(5, 3, i+4)
        ax.plot(real_moving_samples[i], 'o-', color='red')
        total_drop = real_moving_samples[i][0] - real_moving_samples[i][-1]
        ax.set_title(f"真实远离样本{i+1}\n(总衰减={total_drop:.1f}dBm)")
        ax.set_ylim(-85, -40)
        ax.grid(True)
    
    # 真实靠近数据
    for i in range(3):
        ax = plt.subplot(5, 3, i+7)
        ax.plot(real_approaching_samples[i], 'o-', color='green')
        total_increase = real_approaching_samples[i][-1] - real_approaching_samples[i][0]
        ax.set_title(f"真实靠近样本{i+1}\n(总增强={total_increase:.1f}dBm)")
        ax.set_ylim(-85, -40)
        ax.grid(True)
    
    # 生成静止数据
    static_data = dataset.X[dataset.y == 0].numpy()
    ax10 = plt.subplot(5, 3, 10)
    for i in range(5):
        ax10.plot(static_data[i], 'o-', alpha=0.7)
    ax10.set_title("生成静止样本")
    ax10.set_ylim(-85, -40)
    ax10.grid(True)
    
    # 生成远离数据
    moving_data = dataset.X[dataset.y == 1].numpy()
    ax11 = plt.subplot(5, 3, 11)
    for i in range(5):
        ax11.plot(moving_data[i], 'o-', alpha=0.7)
    ax11.set_title("生成远离样本")
    ax11.set_ylim(-85, -40)
    ax11.grid(True)
    
    # 生成靠近数据
    approaching_data = dataset.X[dataset.y == 2].numpy()
    ax12 = plt.subplot(5, 3, 12)
    for i in range(5):
        ax12.plot(approaching_data[i], 'o-', alpha=0.7)
    ax12.set_title("生成靠近样本")
    ax12.set_ylim(-85, -40)
    ax12.grid(True)
    
    plt.tight_layout()
    plt.show()

# 主程序部分更新
if __name__ == "__main__":
    # 设置随机种子
    torch.manual_seed(42)
    np.random.seed(42)
    plt.rcParams['font.sans-serif'] = ['Arial Unicode MS']
    plt.rcParams['axes.unicode_minus'] = False
    
    # 生成数据集
    dataset = EmpiricalBluetoothDataset(num_samples=1000)
    
    # 数据分析
    analyze_real_data(real_static_samples, real_moving_samples, real_approaching_samples)
    
    # 可视化对比
    plot_comparison(dataset, real_static_samples, real_moving_samples, real_approaching_samples)
    
    # 创建数据加载器
    train_loader = DataLoader(dataset, batch_size=32, shuffle=True)
    
    # 打印统计信息
    static_data = dataset.X[dataset.y == 0].numpy()
    moving_data = dataset.X[dataset.y == 1].numpy()
    approaching_data = dataset.X[dataset.y == 2].numpy()
    
    print("\n=== 生成数据统计 ===")
    print("静止数据统计:")
    print(f"平均波动范围: {np.mean(np.ptp(static_data, axis=1)):.1f}dBm")
    print(f"最大波动范围: {np.max(np.ptp(static_data, axis=1)):.1f}dBm")
    
    print("\n远离数据统计:")
    print(f"平均总衰减: {np.mean([s[0]-s[-1] for s in moving_data]):.1f}dBm")
    print(f"最大总衰减: {np.max([s[0]-s[-1] for s in moving_data]):.1f}dBm")
    
    print("\n靠近数据统计:")
    print(f"平均总增强: {np.mean([s[-1]-s[0] for s in approaching_data]):.1f}dBm")
    print(f"最大总增强: {np.max([s[-1]-s[0] for s in approaching_data]):.1f}dBm")
