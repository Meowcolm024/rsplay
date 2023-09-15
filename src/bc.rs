use std::collections::HashMap;

struct Solution;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Cl {
    Num(i32),
    Add,
    Sub,
}

impl Solution {
    fn pop_to(op: &mut Vec<u8>, st: &mut Vec<Cl>) {
        match op.pop().unwrap() {
            1 => st.push(Cl::Sub),
            2 => st.push(Cl::Add),
            _ => {}
        }
    }

    pub fn calculate(s: String) -> i32 {
        let mut st: Vec<Cl> = vec![];
        let mut op: Vec<u8> = vec![0];
        let mut n = None;
        let mut last = 0;
        for i in 0..s.len() {
            let c = s.as_bytes()[i];
            if c >= b'0' && c <= b'9' {
                if let Some(x) = n {
                    n = Some(x * 10 + (c - b'0') as i32);
                } else {
                    n = Some((c - b'0') as i32);
                }
            } else {
                if let Some(x) = n {
                    st.push(Cl::Num(x));
                    n = None;
                }
                match c {
                    b'(' => op.push(0),
                    b')' => Solution::pop_to(&mut op, &mut st),
                    b'+' => {
                        Solution::pop_to(&mut op, &mut st);
                        op.push(2);
                    }
                    b'-' => {
                        Solution::pop_to(&mut op, &mut st);
                        if i == 0 || last == b'(' {
                            st.push(Cl::Num(0));
                        }
                        op.push(1);
                    }
                    _ => (),
                }
            }
            if c != b' ' {
                last = c;
            }
            // println!("{}\n{:?}\n{:?}\n----", c as char, &st, &op);
        }
        if let Some(x) = n {
            st.push(Cl::Num(x));
        }
        Solution::pop_to(&mut op, &mut st);
        println!("{:?}\n{:?}\n----", &st, &op);
        assert!(op.len() == 0);
        let mut cp: Vec<i32> = Vec::new();
        for s in st {
            match s {
                Cl::Num(n) => cp.push(n),
                Cl::Add => {
                    let x = cp.pop().unwrap();
                    let y = cp.pop().unwrap();
                    cp.push(x + y);
                }
                Cl::Sub => {
                    let x = cp.pop().unwrap();
                    let y = cp.pop().unwrap();
                    cp.push(y - x);
                }
            }
            // println!("{:?}", cp);
        }
        cp.pop().unwrap()
    }

    fn op_st(op: &mut Vec<u8>, st: &mut Vec<i32>) {
        match op.pop().unwrap() {
            b'+' => {
                let x = st.pop().unwrap();
                let y = st.pop().unwrap();
                st.push(x + y);
            }
            b'-' => {
                let x = st.pop().unwrap();
                let y = st.pop().unwrap();
                st.push(y - x);
            }
            _ => {}
        }
    }

    pub fn calculate2(s: String) -> i32 {
        let mut st: Vec<i32> = vec![];
        let mut op: Vec<u8> = vec![0];
        let mut n = None;
        let mut last = 0;
        for c in s.as_bytes() {
            if *c >= b'0' && *c <= b'9' {
                if let Some(x) = n {
                    n = Some(x * 10 + (c - b'0') as i32);
                } else {
                    n = Some((c - b'0') as i32);
                }
            } else {
                if let Some(x) = n {
                    st.push(x);
                    n = None;
                }
                match c {
                    b'(' => op.push(0),
                    b')' => Solution::op_st(&mut op, &mut st),
                    b'+' => {
                        Solution::op_st(&mut op, &mut st);
                        op.push(*c);
                    }
                    b'-' => {
                        Solution::op_st(&mut op, &mut st);
                        if last == 0 || last == b'(' {
                            st.push(0);
                        }
                        op.push(*c);
                    }
                    _ => (),
                }
            }
            if *c != b' ' {
                last = *c;
            }
        }
        if let Some(x) = n {
            st.push(x);
        }
        Solution::op_st(&mut op, &mut st);
        st.pop().unwrap()
    }

    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        todo!()
    }

    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut it = strs.into_iter();
        let init = it.next().unwrap();
        let i = it.fold(init.len(), |acc, x| {
            let mut i = 0;
            while i < x.len().min(acc) {
                if x.as_bytes()[i] != init.as_bytes()[i] {
                    break;
                }
                i += 1;
            }
            i
        });
        init[..i].to_string()
    }

    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut cs: [u16; 26] = [0; 26];
        for c in magazine.as_bytes() {
            cs[(c - b'a') as usize] += 1;
        }
        for c in ransom_note.as_bytes() {
            if cs[(c - b'a') as usize] == 0 {
                return false;
            }
            cs[(c - b'a') as usize] -= 1;
        }
        true
    }

    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let mut l = 0;
        let mut r = matrix.len() - 1;
        while l != r {
            let m = (l + r + 1) / 2;
            if matrix[m][0] > target {
                r = m - 1;
            } else {
                l = m;
            }
        }
        let i = l;
        l = 0;
        r = matrix[0].len() - 1;
        while l != r {
            let m = (l + r + 1) / 2;
            if matrix[i][m] > target {
                r = m - 1;
            } else {
                l = m;
            }
        }
        matrix[i][l] == target
    }
}

pub fn bc_main() {
    let s = String::from("-1+4-((-5-6)-(3+3-2)+1)-1");
    println!("{:?}", Solution::calculate(s.clone()));
    println!("{:?}", Solution::calculate2(s));

    // let mat = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
    let mat = vec![vec![1]];
    let res = Solution::search_matrix(mat, 0);
    println!("{}", res);
}
