use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
struct Node {
    key: i32,
    val: i32,
    freq: i32,
    stamp: u32,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.freq.cmp(&other.freq) {
            Ordering::Equal => self.stamp.cmp(&other.stamp),
            ow => ow,
        }
    }
}

#[derive(Debug)]
struct LFUCache {
    map: HashMap<i32, usize>,
    heap: Vec<Node>,
    size: usize,
    capacity: usize,
    stamp: u32, // used for least
}

impl LFUCache {
    fn new(capacity: i32) -> Self {
        Self {
            map: HashMap::with_capacity(capacity as usize),
            heap: Vec::with_capacity(capacity as usize),
            size: 0,
            capacity: capacity as usize,
            stamp: 0,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(i) = self.map.get(&key) {
            let i = *i;
            let stamp = self.get_stamp();
            let n = &mut self.heap[i];
            let val = n.val;
            n.freq += 1;
            n.stamp = stamp;
            self.sink(i);
            val
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        let stamp = self.get_stamp();
        if let Some(i) = self.map.get(&key) {
            let n = &mut self.heap[*i];
            n.val = value;
            n.freq += 1;
            n.stamp = stamp;
            self.sink(*i);
        } else {
            if self.map.len() == self.capacity {
                self.map.remove(&self.heap[0].key);
                self.map.insert(key, 0);
                self.heap[0] = Node {
                    key: key,
                    val: value,
                    freq: 1,
                    stamp: stamp,
                };
                self.sink(0);
            } else {
                let n = Node {
                    key: key,
                    val: value,
                    freq: 1,
                    stamp: stamp,
                };
                self.insert(n);
            }
        }
    }

    fn get_stamp(&mut self) -> u32 {
        self.stamp += 1;
        self.stamp
    }

    fn insert(&mut self, n: Node) {
        let i = self.size;
        self.map.insert(n.key, i);
        self.heap.push(n);
        self.size += 1;
        self.swim(i);
    }

    fn swim(&mut self, i: usize) -> usize {
        if i == 0 {
            return 0;
        }
        let p = (i - 1) / 2;
        if self.heap[p] > self.heap[i] {
            self.map.insert(self.heap[p].key, i);
            self.map.insert(self.heap[i].key, p);
            self.heap.swap(p, i);
            self.swim(p)
        } else {
            i
        }
    }

    fn sink(&mut self, i: usize) -> usize {
        let mut n = i;
        let l = 2 * i + 1;
        let r = 2 * i + 2;
        if l < self.size && self.heap[l] < self.heap[n] {
            n = l;
        }
        if r < self.size && self.heap[r] < self.heap[n] {
            n = r;
        }
        if n != i {
            self.map.insert(self.heap[n].key, i);
            self.map.insert(self.heap[i].key, n);
            self.heap.swap(n, i);
            self.sink(n)
        } else {
            i
        }
    }
}

pub fn lfu_main() {
    let mut cache = LFUCache::new(3);
    cache.put(1, 1);
    cache.put(2, 2);
    cache.put(3, 3);
    dbg!(&cache);
    dbg!(cache.get(1));
    dbg!(&cache);
    cache.put(4, 4);
    dbg!(&cache);
    dbg!(cache.get(1));
    dbg!(cache.get(2));
    dbg!(cache.get(3));
    dbg!(cache.get(3));
    cache.put(5, 5);
    dbg!(&cache);
}
