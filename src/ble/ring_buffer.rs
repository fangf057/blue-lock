use std::fmt::Debug;

#[derive(Debug)]
pub struct RingBuffer<T> {
    buffer: Box<[T]>,
    write_idx: usize,  // 下一个写入位置
    read_idx: usize,   // 下一个读取位置
    capacity: usize,
}

impl<T> RingBuffer<T>
where
    T: Default + Copy + Debug,
{
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: vec![T::default(); capacity].into_boxed_slice(),
            write_idx: 0,
            read_idx: 0,
            capacity,
        }
    }

    #[inline]
    pub fn push(&mut self, value: T) -> Option<T> {
        let next_write = (self.write_idx + 1) % self.capacity;
        
        if next_write == self.read_idx {
            // 缓冲区已满，覆盖最旧数据
            let overwritten = self.buffer[self.read_idx];
            self.buffer[self.write_idx] = value;
            self.write_idx = next_write;
            self.read_idx = (self.read_idx + 1) % self.capacity;
            Some(overwritten)
        } else {
            // 正常写入
            self.buffer[self.write_idx] = value;
            self.write_idx = next_write;
            None
        }
    }

    #[inline]
    pub fn is_full(&self) -> bool {
        (self.write_idx + 1) % self.capacity == self.read_idx
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.write_idx == self.read_idx
    }

    pub fn window_data(&mut self) -> Vec<T> {
        if self.is_empty() {
            return Vec::new();
        }

        let mut data = Vec::with_capacity(self.capacity);
        let mut idx = self.read_idx;

        while idx != self.write_idx {
            data.push(self.buffer[idx]);
            idx = (idx + 1) % self.capacity;
        }
        data
    }

    #[inline]
    pub fn clear(&mut self) {
        self.write_idx = 0;
        self.read_idx = 0;
    }

    #[inline]
    pub fn available_space(&self) -> usize {
        if self.is_full() {
            0
        } else {
            (self.read_idx + self.capacity - self.write_idx - 1) % self.capacity
        }
    }
}