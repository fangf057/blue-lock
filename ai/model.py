import torch
import torch.nn as nn
import torch.nn.functional as F

class HybridModel(nn.Module):
    def __init__(self, input_seq_len=9, num_classes=3, cnn_channels=[16, 32], lstm_hidden_size=32, dropout_rate=0.4):
        super().__init__()
        self.input_seq_len = input_seq_len

        # 1. 趋势特征提取
        self.trend_net = nn.Sequential(
            nn.Conv1d(1, 16, kernel_size=3, padding=1),
            nn.BatchNorm1d(16),
            nn.ReLU(),
            nn.Conv1d(16, 16, kernel_size=3, padding=1),
            nn.BatchNorm1d(16),
            nn.ReLU(),
        )

        # 2. 斜率特征提取
        self.slope_net = nn.Sequential(
            nn.Conv1d(1, 16, kernel_size=3, padding=1),
            nn.BatchNorm1d(16),
            nn.ReLU(),
        )

        # 3. 局部变化特征
        self.local_net = nn.Sequential(
            nn.Conv1d(1, 16, kernel_size=2, stride=1),
            nn.BatchNorm1d(16),
            nn.ReLU(),
        )

        # 4. LSTM层
        lstm_input_size = 48  # 16 * 3 (三个特征分支)
        self.lstm = nn.LSTM(
            input_size=lstm_input_size,
            hidden_size=lstm_hidden_size,
            num_layers=2,
            batch_first=True,
            bidirectional=True,
            dropout=dropout_rate
        )

        # 5. 自注意力
        self.attention = nn.Sequential(
            nn.Linear(lstm_hidden_size*2, 1),
            nn.Softmax(dim=1)
        )

        # 6. 趋势分析层
        self.trend_analysis = nn.Sequential(
            nn.Linear(input_seq_len-1, 32),
            nn.ReLU(),
            nn.Linear(32, 16),
            nn.ReLU()
        )

        # 7. 分类器
        classifier_input_size = lstm_hidden_size * 2 + 16  # LSTM特征 + 趋势特征
        self.classifier = nn.Sequential(
            nn.Linear(classifier_input_size, classifier_input_size // 2),
            nn.ReLU(),
            nn.Dropout(dropout_rate),
            nn.Linear(classifier_input_size // 2, num_classes)
        )

    def compute_slopes(self, x):
        return x[:, 1:] - x[:, :-1]

    def compute_trend_features(self, slopes):
        trend_lengths = torch.zeros_like(slopes)
        current_trend = torch.zeros_like(slopes[:, 0])
        for i in range(slopes.shape[1]):
            mask_down = slopes[:, i] < -2.0  # 显著下降
            mask_up = slopes[:, i] > 2.0    # 显著上升
            current_trend[mask_down] += 1
            current_trend[mask_up] -= 1
            trend_lengths[:, i] = current_trend
        return trend_lengths

    def forward(self, x):
        batch_size = x.size(0)
        
        # 1. 计算斜率
        slopes = self.compute_slopes(x)  # [batch, seq_len-1]
        
        # 2. 趋势特征
        x_trend = x.unsqueeze(1)  # [batch, 1, seq_len]
        trend_features = self.trend_net(x_trend)  # [batch, 16, seq_len]
        
        # 3. 斜率特征
        slopes = slopes.unsqueeze(1)  # [batch, 1, seq_len-1]
        slope_features = self.slope_net(slopes)  # [batch, 16, seq_len-1]
        slope_features = F.pad(slope_features, (0, 1))  # 补齐长度
        
        # 4. 局部变化特征
        local_features = self.local_net(x.unsqueeze(1))  # [batch, 16, seq_len-1]
        local_features = F.pad(local_features, (0, 1))  # 补齐长度
        
        # 5. 特征融合
        combined_features = torch.cat([
            trend_features, 
            slope_features, 
            local_features
        ], dim=1)  # [batch, 48, seq_len]
        
        combined_features = combined_features.permute(0, 2, 1)  # [batch, seq_len, 48]
        
        # 6. LSTM处理
        lstm_out, _ = self.lstm(combined_features)
        
        # 7. 注意力加权
        attention_weights = self.attention(lstm_out)
        attended_out = torch.sum(attention_weights * lstm_out, dim=1)
        
        # 8. 趋势分析
        trend_features = self.trend_analysis(slopes.squeeze(1))
        
        # 9. 最终分类
        final_features = torch.cat([attended_out, trend_features], dim=1)
        logits = self.classifier(final_features)
        
        return logits
