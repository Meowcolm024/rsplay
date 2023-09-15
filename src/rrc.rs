use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

struct RecentCounter {
    q: VecDeque<i32>,
}

impl RecentCounter {
    fn new() -> Self {
        Self { q: VecDeque::new() }
    }

    fn ping(&mut self, t: i32) -> i32 {
        while let Some(t0) = self.q.pop_front() {
            if t0 + 3000 >= t {
                self.q.push_front(t0);
                break;
            }
        }
        self.q.push_back(t);
        self.q.len() as i32
    }
}

struct Solution;
impl Solution {
    fn guessNumber(n: i32) -> i32 {
        fn guess(n: i32) -> i32 {
            if n < 100 {
                -1
            } else if n > 100 {
                1
            } else {
                0
            }
        }
        let mut l = 1;
        let mut r = n;
        loop {
            let m = (l + r) / 2;
            println!("m {}", m);
            match guess(m) {
                -1 => l = m - 1,
                1 => r = m + 1,
                _ => return m,
            }
        }
    }

    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut hs = HashMap::new();
        for a in arr {
            match hs.get(&a) {
                Some(i) => hs.insert(a, i + 1),
                None => hs.insert(a, 1),
            };
        }
        let l = hs.len();
        let s: HashSet<i32> = hs.into_values().collect();
        s.len() == l
    }

    pub fn find_kth_largest_qs(mut nums: Vec<i32>, k: i32) -> i32 {
        fn partition(nums: &mut Vec<i32>, l: i32, r: i32) -> i32 {
            if r - l != 0 {
                let t = std::time::SystemTime::now()
                    .duration_since(std::time::SystemTime::UNIX_EPOCH)
                    .map_or(0, |x| x.as_secs() as i32 % (r - l) + l);
                nums.swap(t as usize, r as usize);
            }
            let x = nums[r as usize];
            let mut i = l;
            for j in l..r {
                if nums[j as usize] <= x {
                    nums.swap(i as usize, j as usize);
                    i += 1;
                }
            }
            nums.swap(i as usize, r as usize);
            i
        }
        fn rec(nums: &mut Vec<i32>, l: i32, r: i32, k: i32) -> i32 {
            let i = partition(nums, l, r);
            if i - l == k - 1 {
                nums[i as usize]
            } else if i - l > k - 1 {
                rec(nums, l, i - 1, k)
            } else {
                rec(nums, i + 1, r, k + l - i - 1)
            }
        }
        let r = nums.len() as i32 - 1;
        rec(&mut nums, 0, r, r - k + 2)
    }

    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut hp = BinaryHeap::new();
        for n in nums {
            hp.push(n);
        }
        for _ in 0..k - 1 {
            hp.pop().unwrap();
        }
        *hp.peek().unwrap()
    }

    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        match nums.len() {
            0 => vec![],
            1 => vec![nums],
            _ => {
                let h = nums.pop().unwrap();
                let res = Solution::permute(nums);
                res.into_iter()
                    .flat_map(|n: Vec<i32>| {
                        let mut ret = Vec::with_capacity(n.len() + 1);
                        for i in 0..=n.len() {
                            let mut nn = n.clone();
                            nn.insert(i, h);
                            ret.push(nn);
                        }
                        ret
                    })
                    .collect()
            }
        }
    }

    pub fn can_visit_all_rooms(mut rooms: Vec<Vec<i32>>) -> bool {
        let mut visit = vec![false; rooms.len()];
        let mut count = 1;
        visit[0] = true;
        let mut queue = rooms[0].clone();
        while let Some(k) = queue.pop() {
            if !visit[k as usize] {
                visit[k as usize] = true;
                count += 1;
                queue.append(&mut rooms[k as usize]);
            }
        }
        count == visit.len()
    }

    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        for i in 0..n {
            for j in 0..i {
                let tmp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = tmp;
            }
        }
        for i in 0..n {
            matrix[i].reverse();
        }
    }

    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        // state
        // 1 -> 1 (2-3n) | 3 (<2,>3 n)
        // 0 -> 0 dead | 2 (=3 n)
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                let c: i32 = [
                    (i + 1, j),
                    (i - 1, j), // ! underflow
                    (i, j + 1),
                    (i, j - 1),
                    (i + 1, j + 1),
                    (i - 1, j + 1),
                    (i + 1, j + 1),
                    (i - 1, j - 1),
                ]
                .iter()
                .map(|(i, j)| {
                    board
                        .get(*i)
                        .and_then(|b| b.get(*j))
                        .map(|x| x % 2)
                        .unwrap_or(0)
                })
                .sum();
                if board[i][j] % 2 == 0 && c == 3 {
                    board[i][j] = 2;
                } else if board[i][j] % 2 == 1 && (c > 3 || c < 2) {
                    board[i][j] = 3;
                }
                dbg!(&board);
            }
        }
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                match board[i][j] {
                    2 => board[i][j] = 1,
                    3 => board[i][j] = 0,
                    _ => (),
                }
            }
        }
    }

    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, nums.len() - 1);
        while l < r {
            let m = (l + r) / 2;
            dbg!(l, r, m);
            if m + 1 <= r {
                if nums[m] > nums[m + 1] {
                    r = m;
                } else {
                    l = m + 1;
                }
            } else {
                r = m;
            }
        }
        l as i32
    }

    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut res = vec![0; n as usize + 1];
        for i in 1..=n {
            dbg!(i, res[i as usize >> 1], i & 1);
            res[i as usize] = res[i as usize >> 1] + (i & 1);
        }
        res
    }

    pub fn add_binary(a: String, b: String) -> String {
        if a.len() > b.len() {
            Solution::add_binary(b, a)
        } else {
            // a <= b
            let a: String = a.chars().rev().collect();
            let mut b: String = b.chars().rev().collect();
            unsafe {
                let mut c = false;
                for i in 0..a.len() {
                    match (a.as_bytes()[i], b.as_bytes()[i]) {
                        (b'0', b'0') if c => {
                            b.as_bytes_mut()[i] = b'1';
                            c = false;
                        }
                        (b'0', b'1') if c => {
                            b.as_bytes_mut()[i] = b'0';
                        }
                        (b'1', b'0') => {
                            b.as_bytes_mut()[i] = b'1';
                        }
                        (b'1', b'1') if !c => {
                            b.as_bytes_mut()[i] = b'0';
                            c = true;
                        }
                        _ => {}
                    }
                }
                for i in a.len()..b.len() {
                    match b.as_bytes()[i] {
                        b'0' if c => {
                            b.as_bytes_mut()[i] = b'1';
                            c = false;
                            break;
                        }
                        b'1' if c => {
                            b.as_bytes_mut()[i] = b'0';
                        }
                        _ => {}
                    }
                }
                if c {
                    b.push('1');
                }
                b.chars().rev().collect()
            }
        }
    }

    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut s = 0;
        let mut g = 0;
        let mut i = 0;
        while i < gas.len() {
            if s >= gas.len() { return -1 }
            g = g + gas[(i + s) % gas.len()] - cost[(i + s) % gas.len()];
            if g < 0 {
                g = 0;
                s += i + 1;
                if s >= gas.len() { return -1 }
                i = 0;
                continue;
            } else {
                i += 1;
            }
        }
        s as i32
    }
}

use std::collections::BTreeSet;

struct SmallestInfiniteSet {
    cache: BTreeSet<i32>,
    n: i32,
}

impl SmallestInfiniteSet {
    fn new() -> Self {
        Self {
            cache: BTreeSet::new(),
            n: 1,
        }
    }

    fn pop_smallest(&mut self) -> i32 {
        if let Some(n) = self.cache.pop_first() {
            if n < self.n {
                return n;
            } else if n > self.n {
                self.cache.insert(n);
            }
        }
        let res = self.n;
        self.n += 1;
        res
    }

    fn add_back(&mut self, num: i32) {
        self.cache.insert(num);
    }
}

use rand::{rngs, Rng};

#[derive(Debug)]
struct RandomizedSet {
    map: HashMap<i32, usize>,
    list: Vec<i32>,
    rng: rngs::ThreadRng,
}

impl RandomizedSet {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            list: Vec::new(),
            rng: rngs::ThreadRng::default(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        if self.map.contains_key(&val) {
            false
        } else {
            self.map.insert(val, self.list.len());
            self.list.push(val);
            true
        }
    }

    fn remove(&mut self, val: i32) -> bool {
        if let Some(i) = self.map.remove(&val) {
            let last = self.list.len() - 1;
            if i < last {
                self.map.insert(self.list[last], i);
                self.list.swap(last, i);
            }
            self.list.pop().unwrap();
            true
        } else {
            false
        }
    }

    fn get_random(&mut self) -> i32 {
        let rnd = self.rng.gen::<usize>() % self.list.len();
        self.list[rnd]
    }
}

pub fn rrc_main() {
    // Solution::guessNumber(120);
    // let res = Solution::find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4);
    // println!("{res}");
    // let mut sis = SmallestInfiniteSet::new();
    // sis.add_back(2);
    // println!("{}", sis.pop_smallest());
    // println!("{}", sis.pop_smallest());
    // println!("{}", sis.pop_smallest());
    // sis.add_back(1);
    // println!("{}", sis.pop_smallest());

    // let xs = vec![1, 2, 3, 4];
    // let res = Solution::permute(xs);
    // println!("{}\n{:?}", res.len(), &res);
    // // let rs = vec![vec![1,3],vec![3,0,1],vec![2],vec![0]];
    // let rs = vec![vec![1], vec![2], vec![3], vec![]];
    // let rt = Solution::can_visit_all_rooms(rs);
    // println!("{rt}");
    // // let res = Solution::combine(4, 2);
    // // println!("{:?}", res);

    // let mut rs = RandomizedSet::new();
    // rs.remove(0);
    // rs.remove(0);
    // rs.insert(0);
    // let x = rs.get_random();
    // dbg!("{} {}", &rs, x);
    // rs.remove(0);
    // let x = rs.insert(0);
    // dbg!("{} {}", rs, x);
    // let xs = vec![1];
    // let res = Solution::find_peak_element(xs);
    // println!("{}", res);

    // let mut board = vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1], vec![0, 0, 0]];
    // Solution::game_of_life(&mut board);
    // dbg!(board);

    let res = Solution::count_bits(5);
    dbg!(res);

    let res = Solution::can_complete_circuit(vec![1,2,3,4,5], vec![3,4,5,1,2]);
    dbg!(res);
}
