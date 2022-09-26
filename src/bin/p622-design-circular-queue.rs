struct MyCircularQueue {
    data: Vec<i32>,
    front: usize,
    len: usize,
    cap: usize,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularQueue {

    fn new(k: i32) -> Self {
        let k = k as usize;
        Self { data: vec![0; k], front: 0, len: 0, cap: k }
    }

    fn get_next_idx(&self) -> usize {
        let idx = self.front + self.len;
        if idx >= self.cap {
            idx - self.cap
        } else {
            idx
        }
    }

    fn get_tail_idx(&self) -> usize {
        let idx = self.front + self.len - 1;
        if idx >= self.cap {
            idx - self.cap
        } else {
            idx
        }
    }
    
    fn en_queue(&mut self, value: i32) -> bool {
        if self.is_full() {
            false
        } else {
            let idx = self.get_next_idx();
            self.data[idx] = value;
            self.len += 1;

            true
        }
    }
    
    fn de_queue(&mut self) -> bool {
        if self.is_empty() {
            false
        } else {
            self.len -= 1;
            self.front += 1;
            if self.front >= self.cap {
                self.front -= self.cap;
            }

            true
        }
    }
    
    fn front(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.data[self.front]
        }
    }
    
    fn rear(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            let idx = self.get_tail_idx();
            self.data[idx]
        }
    }
    
    fn is_empty(&self) -> bool {
        self.len == 0
    }
    
    fn is_full(&self) -> bool {
        self.len == self.cap
    }
}

/**
 * Your MyCircularQueue object will be instantiated and called as such:
 * let obj = MyCircularQueue::new(k);
 * let ret_1: bool = obj.en_queue(value);
 * let ret_2: bool = obj.de_queue();
 * let ret_3: i32 = obj.front();
 * let ret_4: i32 = obj.rear();
 * let ret_5: bool = obj.is_empty();
 * let ret_6: bool = obj.is_full();
 */

fn main() {}
