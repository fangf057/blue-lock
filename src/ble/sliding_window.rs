use std::fmt::Debug;

#[derive(Debug)]
pub struct SlidingWindow<T> {
    buffer: Vec<T>,
    capacity: usize,
}

impl<T> SlidingWindow<T>
where
    T: Default + Copy + Debug,
{
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: vec![T::default(); capacity],
            capacity,
        }
    }

    /// 推入一个新值，丢弃最旧，整体前移
    pub fn push(&mut self, value: T) {
        // 删除第一个（最旧）
        self.buffer.remove(0);
        // 插入到末尾
        self.buffer.push(value);
    }

    /// 获取当前窗口所有值，顺序为old->new
    pub fn window_data(&self) -> Vec<T> {
        self.buffer.to_vec()
    }

    #[inline]
    pub fn clear(&mut self) {
        self.buffer = vec![T::default(); self.capacity];
    }
}