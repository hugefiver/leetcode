use std::cmp::max;

struct MyCalendarThree {
    key: Vec<i32>,
    hei: Vec<i8>,
    ch: Vec<[usize; 2]>,
    next: Vec<usize>,
    val: Vec<i32>,
    seg: Vec<i32>,
    tag: Vec<i32>,
    root: usize,
}

impl MyCalendarThree {
    fn new_node(&mut self, k: i32, v: i32) -> usize {
        self.key.push(k);
        self.hei.push(1);
        self.ch.push([0, 0]);
        self.next.push(0);
        self.val.push(v);
        self.seg.push(v);
        self.tag.push(0);
        self.key.len() - 1
    }
    fn untag(&mut self, i: usize) {
        for j in 0..2 {
            if self.ch[i][j] > 0 {
                self.val[self.ch[i][j]] += self.tag[i];
                self.seg[self.ch[i][j]] += self.tag[i];
                self.tag[self.ch[i][j]] += self.tag[i];
            }
        }
        self.tag[i] = 0;
    }
    fn maintain(&mut self, i: usize) {
        let [lch, rch] = self.ch[i];
        self.hei[i] = max(self.hei[lch], self.hei[rch]) + 1;
        self.seg[i] = max(self.val[i], max(self.seg[lch], self.seg[rch]) + self.tag[i]);
    }
    fn rotate(&mut self, i: usize, dir: bool) -> usize {
        let j = self.ch[i][dir as usize];
        self.ch[i][dir as usize] = self.ch[j][!dir as usize];
        self.ch[j][!dir as usize] = i;
        self.maintain(i);
        self.maintain(j);
        j
    }
    fn insert_key(&mut self, x: i32, mut i: usize, prev: usize) -> usize {
        if i == 0 {
            i = self.new_node(x, self.val[prev]);
            self.next[i] = self.next[prev];
            self.next[prev] = i;
        } else if x != self.key[i] {
            self.untag(i);
            let dir = x > self.key[i];
            self.ch[i][dir as usize] =
                self.insert_key(x, self.ch[i][dir as usize], if dir { i } else { prev });
            self.maintain(i);
            if self.hei[self.ch[i][dir as usize]] > self.hei[self.ch[i][!dir as usize]] + 1 {
                let j = self.ch[i][dir as usize];
                if self.hei[self.ch[j][dir as usize]] < self.hei[self.ch[j][!dir as usize]] {
                    self.ch[i][dir as usize] = self.rotate(j, !dir);
                }
                i = self.rotate(i, dir);
            }
        }
        i
    }
    fn insert_intvl(&mut self, start: i32, end: i32, i: usize, l: usize, r: usize) {
        if i == 0 || r > 0 && self.key[r] <= start || end <= self.key[l] {
            return;
        }
        if r > 0 && start <= self.key[l] && self.key[r] <= end {
            self.val[i] += 1;
            self.seg[i] += 1;
            self.tag[i] += 1;
            return;
        }
        self.untag(i);
        self.insert_intvl(start, end, self.ch[i][0], l, i);
        self.insert_intvl(start, end, self.ch[i][1], self.next[i], r);
        if start <= self.key[i] && self.key[i] < end {
            self.val[i] += 1;
        }
        self.seg[i] = max(
            self.val[i],
            max(self.seg[self.ch[i][0]], self.seg[self.ch[i][1]]),
        );
    }
    fn new() -> Self {
        Self {
            key: vec![0],
            hei: vec![0],
            ch: vec![[0, 0]],
            next: vec![0],
            val: vec![0],
            seg: vec![0],
            tag: vec![0],
            root: 0,
        }
    }
    fn book(&mut self, start: i32, end: i32) -> i32 {
        self.root = self.insert_key(start, self.root, 0);
        self.root = self.insert_key(end, self.root, 0);
        self.insert_intvl(start, end, self.root, self.next[0], 0);
        self.seg[self.root]
    }
}
