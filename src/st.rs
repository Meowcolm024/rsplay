struct MinStack {
    st: Vec<(i32, i32)>,
}

impl MinStack {
    fn new() -> Self {
        Self { st: Vec::new() }
    }

    fn push(&mut self, val: i32) {
        if let Some((h, m)) = self.st.last() {
            self.st.push((val, i32::min(*m, val)));
        } else {
            self.st.push((val, val));
        }
    }

    fn pop(&mut self) {
        self.st.pop();
    }

    fn top(&self) -> i32 {
        self.st.last().unwrap().0
    }

    fn get_min(&self) -> i32 {
        self.st.last().unwrap().1
    }
}

struct Solution;
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut r = nums.len() - 1;
        while l < r {
            let m = l + (r - 1) / 2;
            if target < nums[m] {
                r = m - 1;
            } else if target > nums[m] {
                l = m + 1;
            } else {
                break;
            }
        }
        l as i32
    }

    pub fn is_palindrome(s: String) -> bool {
        let s: Vec<char> = s
            .chars()
            .filter(|x| char::is_ascii_alphanumeric(x))
            .map(|x| char::to_ascii_lowercase(&x))
            .collect();
        for i in 0..s.len() / 2 {
            if s[i] != s[s.len() - i - 1] {
                return false;
            }
        }
        true
    }

    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        use std::collections::HashSet;
        let mut vt: HashSet<char> = HashSet::new();
        let mut ht: HashSet<char> = HashSet::new();
        for i in 0..9 {
            vt.clear();
            ht.clear();
            for j in 0..9 {
                let vij = board[i][j];
                if vij != '.' {
                    if vt.contains(&vij) {
                        return false;
                    } else {
                        vt.insert(vij);
                    }
                }
                let hij = board[j][i];
                if hij != '.' {
                    if ht.contains(&hij) {
                        return false;
                    } else {
                        ht.insert(hij);
                    }
                }
            }
        }
        for i in 0..9 {
            vt.clear();
            for j in 0..3 {
                for k in 0..3 {
                    let vij = board[(i % 3) * 3 + j][(i / 3) * 3 + k];
                    if vij != '.' {
                        if vt.contains(&vij) {
                            return false;
                        } else {
                            vt.insert(vij);
                        }
                    }
                }
            }
        }
        true
    }

    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut map = [0; 26];
        for c in s.chars() {
            map[(c as u8 - 'a' as u8) as usize] += 1;
        }
        for c in t.chars() {
            map[(c as u8 - 'a' as u8) as usize] -= 1;
            if map[(c as u8 - 'a' as u8) as usize] < 0 {
                return false;
            }
        }
        true
    }

    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        fn feat(s: &String) -> Vec<u8> {
            let mut res = vec![0; 26];
            for b in s.as_bytes() {
                res[(b - b'a') as usize] += 1;
            }
            res
        }
        use std::collections::HashMap;
        let mut map: HashMap<Vec<u8>, Vec<String>> = HashMap::new();
        for s in strs {
            let f = feat(&s);
            if let Some(ss) = map.get_mut(&f) {
                ss.push(s);
            } else {
                map.insert(f, vec![s]);
            }
        }
        map.into_values().collect()
    }
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut dp: Vec<i32> = vec![i32::MAX; amount as usize + 1];
        dp[0] = 0;
        for i in 1..=amount as usize {
            for c in &coins {
                if i >= *c as usize && dp[i - *c as usize] != i32::MAX {
                    dp[i] = i32::min(dp[i], 1 + dp[i - *c as usize]);
                }
            }
        }
        if dp[amount as usize] == i32::MAX {
            -1
        } else {
            dp[amount as usize]
        }
    }

    pub fn minimum_total(mut triangle: Vec<Vec<i32>>) -> i32 {
        if triangle.len() == 1 {
            return triangle[0][0];
        }
        for i in (0..triangle.len() - 1).rev() {
            for j in 0..i + 1 {
                triangle[i][j] += i32::min(triangle[i + 1][j], triangle[i + 1][j + 1]);
            }
        }
        triangle[0][0]
    }

    pub fn min_path_sum(mut grid: Vec<Vec<i32>>) -> i32 {
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if i > 0 && j > 0 {
                    grid[i][j] += i32::min(grid[i - 1][j], grid[i][j - 1]);
                } else if i > 0 {
                    grid[i][j] += grid[i - 1][j];
                } else if j > 0 {
                    grid[i][j] += grid[i][j - 1];
                }
            }
        }
        *grid.last().unwrap().last().unwrap()
    }

    pub fn roman_to_int(s: String) -> i32 {
        fn go(c: u8) -> i32 {
            match c {
                b'I' => 1,
                b'V' => 5,
                b'X' => 10,
                b'L' => 50,
                b'C' => 100,
                b'D' => 500,
                b'M' => 1000,
                _ => 0,
            }
        }
        let mut it = s.as_bytes().into_iter().rev();
        let mut last = *it.next().unwrap();
        let mut acc = go(last);
        for i in it {
            match (*i, last) {
                (b'I', b'V') => acc -= 1,
                (b'I', b'X') => acc -= 1,
                (b'X', b'L') => acc -= 10,
                (b'X', b'C') => acc -= 10,
                (b'C', b'D') => acc -= 100,
                (b'C', b'M') => acc -= 100,
                (i, _) => {
                    last = i;
                    acc += go(i);
                }
            }
        }
        acc
    }

    pub fn int_to_roman(mut num: i32) -> String {
        [
            (1000, "M"),
            (900, "CM"),
            (500, "D"),
            (400, "CD"),
            (100, "C"),
            (90, "XC"),
            (50, "L"),
            (40, "XL"),
            (10, "X"),
            (9, "IX"),
            (5, "V"),
            (4, "IV"),
            (1, "I"),
        ]
        .into_iter()
        .map(|(i, n)| {
            let res = num / i;
            num = num % i;
            n.repeat(res as usize)
        })
        .collect()
    }

}

pub fn st_main() {
    // println!("ins {}", Solution::search_insert(vec![1, 3, 5, 6], 2));
    // println!(
    //     "{}",
    //     Solution::is_palindrome(String::from("A man, a plan, a canal: Panama"))
    // );
    // println!(
    //     "{}",
    //     Solution::is_anagram("anagram".to_string(), "nagaram".to_string())
    // );
    fn sqrt(n: i32) -> i32 {
        let mut x = n as f64;
        for _ in 0..(x.log2().ceil() as usize) {
            x = (x + n as f64 / x) / 2.0;
        }
        x.floor() as i32
    }
    println!("{}", sqrt(8));
    println!("{}", Solution::int_to_roman(1994))
}
