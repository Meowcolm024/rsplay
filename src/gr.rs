struct Solution;
impl Solution {
    fn dfs(grid: &mut Vec<Vec<char>>, i: usize, j: usize) {
        if i < grid.len() && j < grid[0].len() {
            if grid[i][j] == '1' {
                grid[i][j] = '0';
                if j > 0 {
                    Solution::dfs(grid, i, j - 1);
                }
                if i > 0 {
                    Solution::dfs(grid, i - 1, j);
                }
                Solution::dfs(grid, i + 1, j);
                Solution::dfs(grid, i, j + 1);
            }
        }
    }
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let mut count = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == '1' {
                    count += 1;
                    Solution::dfs(&mut grid, i, j);
                }
            }
        }
        count
    }
    fn srh(board: &mut Vec<Vec<char>>, i: usize, j: usize) -> u8 {
        if board[i][j] == 'O' {
            board[i][j] = 'X';
            let t1 = if j > 0 {
                Solution::srh(board, i, j - 1)
            } else {
                1
            };
            let t2 = if i > 0 {
                Solution::srh(board, i - 1, j)
            } else {
                1
            };
            let t3 = if i < board.len() - 1 {
                Solution::srh(board, i + 1, j)
            } else {
                1
            };
            let t4 = if j < board[0].len() - 1 {
                Solution::srh(board, i, j + 1)
            } else {
                1
            };
            let res = t1 | t2 | t3 | t4;
            if res == 1 {
                board[i][j] = 'O';
            }
            res
        } else {
            0
        }
    }

    pub fn solve(board: &mut Vec<Vec<char>>) {
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                Solution::srh(board, i, j);
            }
        }
    }

    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        use std::collections::vec_deque::VecDeque;
        let mut acc: VecDeque<&i32> =
            VecDeque::from(nums.iter().take(k as usize).collect::<Vec<&i32>>());
        let mut sum = acc.iter().fold(0, |x, y| x + **y);
        let mut max: i32 = sum;
        for i in (k as usize)..nums.len() {
            println!("{:?} {}", acc, max);
            let f = acc.pop_front().unwrap();
            acc.push_back(&nums[i]);
            sum = sum - f + nums[i];
            max = max.max(sum);
        }
        max as f64 / k as f64
    }

    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        gain.into_iter()
            .fold((0, 0), |(c, m), n| (c + n, m.max(c + n)))
            .1
    }

    pub fn min_flips(mut a: i32, mut b: i32, mut c: i32) -> i32 {
        let mut acc = 0;
        for _ in 0..32 {
            match (a & 1, b & 1, c & 1) {
                (0, 1, 0) => acc += 1,
                (1, 0, 0) => acc += 1,
                (1, 1, 0) => acc += 2,
                (0, 0, 1) => acc += 1,
                (_, _, _) => (),
            }
            a >>= 1;
            b >>= 1;
            c >>= 1;
        }
        acc
    }

    pub fn daily_temperatures(mut temperatures: Vec<i32>) -> Vec<i32> {
        let mut st: Vec<(i32, i32)> = Vec::new();
        for i in 0..temperatures.len() {
            while let Some((t, ti)) = st.last() {
                if *t < temperatures[i] {
                    let ti = *ti;
                    st.pop().unwrap();
                    temperatures[ti as usize] = i as i32 - ti;
                } else {
                    break;
                }
            }
            st.push((temperatures[i], i as i32));
        }
        while let Some((_, i)) = st.pop() {
            temperatures[i as usize] = 0;
        }
        temperatures
    }

    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        fn gcd<'a>(mut str1: &'a [u8], mut str2: &'a [u8]) -> String {
            // str1 > str2
            while str2.len() > 0 {
                let mut i = 0;
                while i < str1.len() && str1[i] == str2[i % str2.len()] {
                    i += 1;
                }
                let i = (i / str2.len()) * str2.len();
                if i == 0 {
                    return String::from("");
                } else {
                    let rem = &str1[i..];
                    str1 = str2;
                    str2 = rem;
                }
            }
            String::from_utf8(str1.into()).unwrap()
        }
        if str1.len() < str2.len() {
            gcd(str2.as_bytes(), str1.as_bytes())
        } else {
            gcd(str1.as_bytes(), str2.as_bytes())
        }
    }

    pub fn reverse_vowels(s: String) -> String {
        fn is_vowel(c: char) -> bool {
            matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U')
        }
        let mut s: Vec<char> = s.chars().collect();
        let mut i = 0;
        let mut j = s.len() - 1;
        while i < j {
            match (is_vowel(s[i]), is_vowel(s[j])) {
                (true, true) => {
                    let t = s[i];
                    s[i] = s[j];
                    s[j] = t;
                    i += 1;
                    j -= 1;
                }
                (true, false) => {
                    j -= 1;
                }
                (false, true) => {
                    i += 1;
                }
                (false, false) => {
                    i += 1;
                    j -= 1;
                }
            }
        }
        s.into_iter().collect()
    }

    pub fn reverse_words(s: String) -> String {
        s.split(' ')
            .rev()
            .filter(|s| s.len() > 0)
            .fold(String::new(), |acc, s| acc + s + " ")
            .trim()
            .to_owned()
    }

    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_unstable_by(|x, y| x[0].cmp(&y[0]));
        let mut c = 0;
        let mut last = &intervals[0];
        for i in 1..intervals.len() {
            if intervals[i][0] < last[1] {
                c += 1;
                if intervals[i][1] < last[1] {
                    last = &intervals[i]
                }
            } else {
                last = &intervals[i];
            }
        }
        c
    }

    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let mut res = vec![chars[0]];
        let mut c = chars[0];
        let mut n = 1;
        for i in 1..chars.len() {
            if chars[i] == c {
                n += 1;
            } else {
                if n > 1 {
                    res.append(&mut n.to_string().chars().collect());
                }
                n = 1;
                c = chars[i];
                res.push(c);
            }
        }
        if n > 1 {
            res.append(&mut n.to_string().chars().collect());
        }
        std::mem::swap(chars, &mut res);
        chars.len() as i32
    }

    pub fn word_pattern(pattern: String, s: String) -> bool {
        let s = s.split_whitespace().into_iter().collect::<Vec<&str>>();
        if pattern.len() != s.len() {
            return false;
        }
        use std::collections::HashMap;
        let mut r: HashMap<char, &str> = HashMap::new();
        let mut l: HashMap<&str, char> = HashMap::new();
        for (p, n) in pattern.chars().zip(s) {
            if let Some(q) = l.get(&n) {
                if *q != p {
                    return false;
                }
            } else if let Some(m) = r.get(&p) {
                if *n != **m {
                    return false;
                }
            } else {
                r.insert(p, n);
                l.insert(n, p);
            }
        }
        true
    }

    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut map: HashMap<i32, i32> = HashMap::new();
        for i in 0..nums.len() {
            if let Some(j) = map.get(&(target - nums[i])) {
                return vec![i as i32, *j];
            } else {
                map.insert(nums[i], i as i32);
            }
        }
        vec![]
    }

    pub fn max_vowels(s: String, k: i32) -> i32 {
        use std::collections::VecDeque;
        fn is_vowel(c: u8) -> bool {
            matches!(c as char, 'a' | 'e' | 'i' | 'o' | 'u')
        }
        let mut q: VecDeque<u8> = VecDeque::with_capacity(k as usize);
        for i in 0..k as usize {
            q.push_back(s.as_bytes()[i]);
        }
        let mut n = q.iter().filter(|s| is_vowel(**s)).count() as i32;
        let mut max = n;
        for i in k as usize..s.len() {
            let h = q.pop_front().unwrap();
            q.push_back(s.as_bytes()[i]);
            match (is_vowel(h), is_vowel(s.as_bytes()[i])) {
                (true, false) => n -= 1,
                (false, true) => {
                    n += 1;
                    max = max.max(n)
                }
                _ => (),
            }
        }
        max
    }

    // pub fn generate_parenthesis(n: i32) -> Vec<String> {
    //     match n {
    //         0 => vec![],
    //         1 => vec!["()".to_string()],
    //         n => {
    //             let res = Solution::generate_parenthesis(n - 1);
    //             let mut out: Vec<String> = res.clone().into_iter().map(|s| s + "()").collect();
    //             let mut res: Vec<String> = res.into_iter().map(|s| format!("({})", s)).collect();
    //             res.append(&mut out);
    //             res
    //         }
    //     }
    // }

    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut st: Vec<i32> = Vec::new();
        for t in tokens {
            match t.as_str() {
                "+" => {
                    let a = st.pop().unwrap();
                    *st.last_mut().unwrap() += a;
                }
                "-" => {
                    let a = st.pop().unwrap();
                    *st.last_mut().unwrap() -= a;
                }
                "*" => {
                    let a = st.pop().unwrap();
                    *st.last_mut().unwrap() *= a;
                }
                "/" => {
                    let a = st.pop().unwrap();
                    *st.last_mut().unwrap() /= a;
                }
                t => st.push(t.parse::<i32>().unwrap()),
            }
        }
        *st.last().unwrap()
    }
}

pub fn gr_main() {
    let nums = vec![0, 4, 0, 3, 2];
    Solution::find_max_average(nums, 1);
    println!(
        "{}",
        Solution::gcd_of_strings(String::from("LEET"), String::from("CODE"))
    );
    let s = String::from("ibpbhixfiouhdljnjfflpapptrxgcomvnb");
    Solution::max_vowels(s, 33);
}
