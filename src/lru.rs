use std::collections::HashMap;

#[derive(Debug)]
struct List<T> {
    val: T,
    prev: Option<*mut List<T>>,
    next: Option<*mut List<T>>,
}

#[derive(Debug)]
struct LRUCache {
    list: Option<*mut List<(i32, i32)>>,
    last: Option<*mut List<(i32, i32)>>,
    map: HashMap<i32, Box<List<(i32, i32)>>>,
    capacity: usize,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        Self {
            list: None,
            last: None,
            map: HashMap::with_capacity(capacity as usize),
            capacity: capacity as usize,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if self.get_move(key) {
            self.list.map(|x| unsafe { (*x).val.1 }).unwrap()
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.get_move(key) {
            self.list.map(|x| unsafe { (*x).val.1 = value });
        } else {
            // remove last
            if self.map.len() == self.capacity {
                let pl = self.last.unwrap();
                let prev = unsafe { (*pl).prev };
                if let Some(x) = prev {
                    unsafe { (*x).next = None }
                } else {
                    self.list = None;
                }
                self.last = prev;
                self.map.remove(unsafe { &(*pl).val.0 });
            }
            // new node
            let n = Box::new(List {
                val: (key, value),
                prev: None,
                next: self.list,
            });
            let p = (&*n) as *const List<(i32, i32)>;
            // set prev and change front
            self.list.map(|x| unsafe {
                (*x).prev = Some(p as *mut List<(i32, i32)>);
            });
            self.list = Some(p as *mut List<(i32, i32)>);
            if self.last.is_none() {
                self.last = self.list;
            }
            self.map.insert(key, n);
        }
    }

    fn get_move(&mut self, key: i32) -> bool {
        if let Some(n) = self.map.get_mut(&key) {
            if self.list.map_or(false, |x| unsafe { (*x).val.0 == key }) {
                return true;
            }
            n.prev.map(|x| unsafe { (*x).next = n.next });
            if let Some(x) = n.next {
                unsafe {
                    (*x).prev = n.prev;
                }
            } else {
                self.last = n.prev; // reset last if is last node
            }
            n.prev = None;
            // list and n
            n.next = self.list;
            let p = (&**n) as *const List<(i32, i32)>;
            self.list.map(|x| unsafe {
                (*x).prev = Some(p as *mut List<(i32, i32)>);
            });
            self.list = Some(p as *mut List<(i32, i32)>);
            if self.last.is_none() {
                self.last = self.list;
            }
            true
        } else {
            false
        }
    }
}

pub fn lru_main() {
    let mut lru = LRUCache::new(2);
    dbg!(lru.put(2, 1));
    dbg!(lru.put(3, 2));
    dbg!(&lru);
    dbg!(lru.get(3));
    dbg!(&lru);
    dbg!(lru.get(2));
    dbg!(&lru);

    dbg!(lru.put(4, 3));
    dbg!(&lru);
    dbg!(lru.get(2));
    dbg!(lru.get(3));
    dbg!(lru.get(4));
}
