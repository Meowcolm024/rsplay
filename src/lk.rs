struct Solution;
impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let t1 = text1.chars().collect::<Vec<char>>();
        let t2 = text2.chars().collect::<Vec<char>>();
        let l1 = text1.len();
        let l2 = text2.len();
        let mut dp = vec![vec![0; l1 + 1]; l2 + 1];
        for m in 1..l1 + 1 {
            for n in 1..l2 + 1 {
                if t1[m - 1] == t2[n - 1] {
                    dp[n][m] = 1 + dp[n - 1][m - 1];
                } else {
                    dp[n][m] = i32::max(dp[n][m - 1], dp[n - 1][m]);
                }
            }
        }
        for l in &dp {
            println!("{:?}", l);
        }
        dp[l2][l1]
    }

    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut l = 0;
        let mut r = numbers.len() - 1;
        while l < r {
            let n = numbers[l] + numbers[r];
            if n < target {
                l += 1;
            } else if n > target {
                r -= 1;
            } else {
                break;
            }
        }
        vec![(l + 1) as i32, (r + 1) as i32]
    }

    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = 0;
        let mut sum = nums[0];
        let mut min = 0;
        loop {
            println!("l {} r {} s {} m {}", l, r, sum, min);
            if sum < target {
                r += 1;
                if r == nums.len() {
                    break;
                }
                sum += nums[r];
            } else {
                min = if min == 0 {
                    r - l + 1
                } else {
                    usize::min(min, r - l + 1)
                };
                sum -= nums[l];
                l += 1;
                if l > r {
                    break;
                }
            }
        }
        min as i32
    }

    pub fn is_happy(n: i32) -> bool {
        use std::collections::HashSet;
        let mut n = n;
        let mut set: HashSet<i32> = HashSet::new();
        set.insert(n);
        loop {
            let mut r = 0;
            while n != 0 {
                r += (n % 10) * (n % 10);
                n = n / 10;
            }
            n = r;
            if n == 1 {
                return true;
            }
            if set.contains(&n) {
                return false;
            } else {
                set.insert(n);
            }
        }
    }

    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut acc: Vec<String> = vec![];
        if nums.len() == 0 {
            acc
        } else {
            let mut l = 0;
            let mut i = 1;
            while i < nums.len() {
                if nums[i] - nums[i - 1] != 1 {
                    if i - l == 1 {
                        acc.push(format!("{}", nums[l]));
                    } else {
                        acc.push(format!("{}->{}", nums[l], nums[i - 1]));
                    }
                    l = i;
                }
                i += 1;
            }
            if i - l == 1 {
                acc.push(format!("{}", nums[l]));
            } else {
                acc.push(format!("{}->{}", nums[l], nums[i - 1]));
            }
            acc
        }
    }

    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut itv = intervals.clone();
        itv.sort_by(|a, b| a[0].cmp(&b[0]));
        let mut it = itv.into_iter();
        let mut cur = it.next().unwrap();
        let mut res = vec![];
        for i in it {
            if i[0] <= cur[1] {
                cur[1] = i32::max(cur[1], i[1]);
            } else {
                res.push(cur);
                cur = i;
            }
        }
        res.push(cur);
        res
    }

    pub fn is_isomorphic(s: String, t: String) -> bool {
        use std::collections::{HashMap, HashSet};
        let mut map: HashMap<char, char> = HashMap::new();
        let mut used: HashSet<char> = HashSet::new();
        for (a, b) in s.chars().zip(t.chars()) {
            if let Some(c) = map.get(&a) {
                if b != *c {
                    return false;
                }
            } else {
                map.insert(a, b);
                if used.contains(&b) {
                    return false;
                } else {
                    used.insert(b);
                }
            }
        }
        true
    }

    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut ls = vec![1; nums.len()];
        let mut rs = vec![1; nums.len()];
        for i in 1..nums.len() {
            ls[i] = ls[i-1] * nums[i-1];
            rs[nums.len() - 1 - i] = rs[nums.len() - i] * nums[nums.len() - i];
        }
        ls.into_iter().zip(rs).map(|(x, y)| x * y).collect()
    }
}

pub fn lk_main() {
    // println!(
    //     "{}",
    //     Solution::longest_common_subsequence(String::from("ezupkr"), String::from("ubmrapg"))
    // );
    // println!(
    //     "{}",
    //     Solution::min_sub_array_len(15, vec![5, 1, 3, 5, 10, 7, 4, 9, 2, 8])
    // );
    // println!("{}", Solution::is_happy(16));
    println!("{:?}", Solution::summary_ranges(vec![0, 2, 3, 4, 6, 8, 9]));
    println!(
        "{:?}",
        Solution::merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]])
    );
    println!(
        "{}",
        Solution::is_isomorphic(String::from("badc"), String::from("baba"))
    );

    println!("{:?}", Solution::product_except_self(vec![-1,1,0,-3,3]))
}
