pub struct RingBuffer<T> {
    buffer: Box<[T]>,   // 固定大小数组
    head: usize,        // 写入位置
    count: usize,       // 当前元素数
    window_size: usize, // 等同于buffer.len()
}

impl<T> RingBuffer<T>
where
    T: Copy + Default,
{
    pub fn new(window_size: usize) -> Self {
        Self {
            buffer: vec![Default::default(); window_size].into_boxed_slice(),
            head: 0,
            count: 0,
            window_size,
        }
    }

    #[inline]
    pub fn push(&mut self, value: T) {
        self.buffer[self.head] = value;
        self.head = (self.head + 1) % self.window_size;
        self.count = self.count.min(self.window_size - 1) + 1;
    }

    #[inline]
    pub fn is_full(&self) -> bool {
        self.count == self.window_size
    }

    /// 获取当前窗口数据 (按时间顺序从旧到新)
    pub fn window_data(&self) -> Vec<T> {
        let mut data = Vec::with_capacity(self.window_size);
        if self.count >= self.window_size {
            for i in 0..self.window_size {
                let idx = (self.head + i) % self.window_size;
                data.push(self.buffer[idx]);
            }
        } else {
            // 窗口未满时返回已有数据
            for i in 0..self.count {
                data.push(self.buffer[i]);
            }
        }
        data
    }
}
