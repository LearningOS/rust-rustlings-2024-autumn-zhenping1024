/*
    heap
    This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;

/// 二叉堆结构体
pub struct Heap<T>
where
    T: Default,
{
    count: usize,                              // 当前堆中元素的数量
    items: Vec<T>,                             // 堆元素，索引从1开始
    comparator: fn(&T, &T) -> bool,           // 比较函数，用于维护堆的性质
}

impl<T> Heap<T>
where
    T: Default,
{
    /// 创建一个新的堆，指定比较函数
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()], // 索引0不使用
            comparator,
        }
    }

    /// 返回堆的大小
    pub fn len(&self) -> usize {
        self.count
    }

    /// 检查堆是否为空
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// 向堆中添加一个新元素
    pub fn add(&mut self, value: T) {
        self.items.push(value);
        self.count += 1;
        let mut idx = self.count;

        // 上浮操作，维护堆的性质
        while idx > 1 {
            let parent = self.parent_idx(idx);
            // 如果当前元素与父节点满足比较器的条件，则交换
            if (self.comparator)(&self.items[idx], &self.items[parent]) {
                self.items.swap(idx, parent);
                idx = parent;
            } else {
                break;
            }
        }
    }

    /// 获取父节点的索引
    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    /// 检查当前节点是否有子节点
    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    /// 获取左子节点的索引
    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    /// 获取右子节点的索引
    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    /// 获取当前节点中“最小”子节点的索引，根据比较器
    fn smallest_child_idx(&self, idx: usize) -> usize {
        let left = self.left_child_idx(idx);
        let right = self.right_child_idx(idx);

        if right > self.count {
            // 只有左子节点存在
            left
        } else {
            // 比较左子节点和右子节点，选择满足比较器条件的子节点
            if (self.comparator)(&self.items[left], &self.items[right]) {
                left
            } else {
                right
            }
        }
    }

    /// 下沉操作，维护堆的性质
    fn heapify_down(&mut self, idx: usize) {
        let mut idx = idx;
        while self.left_child_idx(idx) <= self.count {
            let child_idx = self.smallest_child_idx(idx);
            if (self.comparator)(&self.items[child_idx], &self.items[idx]) {
                self.items.swap(idx, child_idx);
                idx = child_idx;
            } else {
                break;
            }
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// 创建一个新的最小堆
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// 创建一个新的最大堆
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default + Clone, // 需要 Clone 来复制元素
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_empty() {
            return None;
        }

        // 取出堆顶元素
        let top = self.items[1].clone();

        if self.count == 1 {
            // 如果堆中只有一个元素，直接移除
            self.items.pop();
            self.count -= 1;
        } else {
            // 将最后一个元素移动到堆顶
            let last = self.items.pop().unwrap();
            self.items[1] = last;
            self.count -= 1;
            // 下沉操作，维护堆的性质
            self.heapify_down(1);
        }

        Some(top)
    }
}

/// 最小堆结构体，用于创建最小堆实例
pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new_min()
    }
}

/// 最大堆结构体，用于创建最大堆实例
pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new_max()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}