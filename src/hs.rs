use std::{cell::RefCell, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution;
impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let (mut m, mut n) = (matrix.len(), matrix[0].len());
        let (mut x, mut y) = (0, 0);
        let mut res = vec![0; m * n];
        for i in 0..res.len() {
            res[i] = matrix[y][x];
            if i == res.len() - 1 {
                break;
            }
            // println!("{} {:?} {:?}", res[i], (y, x), (n, m));
            if x == matrix[0].len() - n && x != n - 1 {
                if y == matrix.len() - m {
                    x += 1;
                } else if y == matrix.len() - m + 1 {
                    m -= 1;
                    n -= 1;
                    // println!("!!! {} ({:?}, {:?})", res[i], (y, x), (n, m));
                    x = matrix[0].len() - n;
                    y = matrix.len() - m;
                } else {
                    y -= 1;
                }
            } else if x == n - 1 {
                if y == m - 1 {
                    x -= 1;
                } else {
                    y += 1;
                }
            } else {
                if y == matrix.len() - m {
                    x += 1;
                } else {
                    x -= 1;
                }
            }
        }
        res
    }

    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let mut rz: [u64; 4] = [0; 4];
        let mut cz: [u64; 4] = [0; 4];
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if matrix[i][j] == 0 {
                    rz[i / 64] |= 1 << (i % 64);
                    cz[j / 64] |= 1 << (j % 64);
                }
            }
        }
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if ((rz[i / 64] >> (i % 64)) & 1 == 1) || ((cz[j / 64] >> (j % 64)) & 1 == 1) {
                    matrix[i][j] = 0;
                }
            }
        }
    }

    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(root) = root {
            1 + i32::max(
                Solution::max_depth(root.borrow().left.clone()),
                Solution::max_depth(root.borrow().right.clone()),
            )
        } else {
            0
        }
    }
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        root.clone().map(|t| {
            let r = Solution::invert_tree(t.borrow().left.clone());
            let l = Solution::invert_tree(t.borrow().right.clone());
            t.borrow_mut().left = l;
            t.borrow_mut().right = r;
        });
        root
    }

    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(root) = root {
            Solution::invert_tree(root.borrow().left.clone()) == root.borrow().right.clone()
        } else {
            true
        }
    }

    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        if let Some(root) = root {
            let val = root.borrow().val;
            if target_sum == val && root.borrow().left.is_none() && root.borrow().right.is_none() {
                true
            } else {
                Solution::has_path_sum(root.borrow().left.clone(), target_sum - val)
                    || Solution::has_path_sum(root.borrow().right.clone(), target_sum - val)
            }
        } else {
            false
        }
    }

    fn rn(root: &Rc<RefCell<TreeNode>>, acc: i32) -> i32 {
        let val = acc * 10 + root.borrow().val;
        match (&root.borrow().left, &root.borrow().right) {
            (None, None) => val,
            (None, Some(r)) => Solution::rn(r, val),
            (Some(l), None) => Solution::rn(l, val),
            (Some(l), Some(r)) => Solution::rn(l, val) + Solution::rn(r, val),
        }
    }

    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(root) = root {
            Solution::rn(&root, root.borrow().val)
        } else {
            0
        }
    }

    pub fn simplify_path(path: String) -> String {
        let mut st: Vec<&str> = vec![];
        for s in path.split_inclusive('/') {
            match s {
                "../" | ".." => {
                    if st.len() > 0 {
                        st.pop();
                    }
                }
                "./" | "." => (),
                "/" => (),
                s => st.push(s),
            }
        }
        (String::from("/") + st.into_iter().collect::<String>().trim_end_matches('/')).to_owned()
    }

    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        const MIN: i32 = -1001;
        fn mps(root: &Rc<RefCell<TreeNode>>) -> (i32, i32) {
            let val = root.borrow().val;
            let (l, lx) = root.borrow().left.as_ref().map_or((MIN, MIN), |t| mps(&t));
            let (r, rx) = root.borrow().right.as_ref().map_or((MIN, MIN), |t| mps(&t));
            let z = vec![val, val + l, val + r].into_iter().max().unwrap();
            let zx = vec![z, val + l + r, lx, rx].into_iter().max().unwrap();
            (z, zx)
        }
        root.map_or(MIN, |t| {
            let (l, r) = mps(&t);
            i32::max(l, r)
        })
    }

    pub fn search_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn srt(root: &Rc<RefCell<TreeNode>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
            match root.borrow().val.cmp(&val) {
                std::cmp::Ordering::Equal => Some(root.clone()),
                std::cmp::Ordering::Less => {
                    root.borrow().right.as_ref().map_or(None, |t| srt(t, val))
                }
                std::cmp::Ordering::Greater => {
                    root.borrow().left.as_ref().map_or(None, |t| srt(t, val))
                }
            }
        }
        root.map_or(None, |r| srt(&r, val))
    }

    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn rec(root: &Rc<RefCell<TreeNode>>, max: i32) -> i32 {
            let val = root.borrow().val;
            let r = if val >= max { 1 } else { 0 };
            let rl = root
                .borrow()
                .left
                .as_ref()
                .map_or(0, |r| rec(r, val.max(max)));
            let rr = root
                .borrow()
                .right
                .as_ref()
                .map_or(0, |r| rec(r, val.max(max)));
            r + rl + rr
        }
        match root {
            Some(r) => rec(&r, i32::MIN),
            None => 0,
        }
    }

    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        use std::collections::VecDeque;
        let mut acc = 0;
        let mut q = VecDeque::from([root]);
        while let Some(r) = q.pop_front() {
            if let Some(n) = r {
                acc += 1;
                q.push_back(n.borrow().left.clone());
                q.push_back(n.borrow().right.clone());
            }
        }
        acc
    }

    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        fn rec(root: Rc<RefCell<TreeNode>>) -> Rc<RefCell<TreeNode>> {
            let rl = root.borrow().left.clone();
            let rr = root.borrow().right.clone();
            match (rl, rr) {
                (None, None) => root,
                (None, Some(r)) => rec(r),
                (Some(l), None) => {
                    root.borrow_mut().left = None;
                    root.borrow_mut().right = Some(l.clone());
                    rec(l)
                }
                (Some(l), Some(r)) => {
                    root.borrow_mut().right = Some(l.clone());
                    let ln = rec(l);
                    root.borrow_mut().left = None;
                    ln.borrow_mut().right = Some(r.clone());
                    rec(r)
                }
            }
        }
        if let Some(r) = root {
            rec(r.clone());
        }
    }

    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        use std::collections::VecDeque;
        let mut p: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        let mut q: VecDeque<Rc<RefCell<TreeNode>>> = root.into_iter().collect();
        let mut res = Vec::new();

        while !(q.is_empty() && p.is_empty()) {
            let mut l = Vec::new();
            while let Some(tn) = q.pop_front() {
                tn.borrow().left.as_ref().map(|x| p.push_back(x.clone()));
                tn.borrow().right.as_ref().map(|x| p.push_back(x.clone()));
                l.push(tn.borrow().val);
            }
            res.push(l);
            q.append(&mut p);
        }
        res
    }

    pub fn right_side_view(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        use std::collections::VecDeque;
        let mut p: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        let mut q: VecDeque<Rc<RefCell<TreeNode>>> = root.into_iter().collect();
        let mut res = Vec::new();

        while !(q.is_empty() && p.is_empty()) {
            let mut r = None;
            while let Some(tn) = q.pop_front() {
                tn.borrow().left.as_ref().map(|x| p.push_back(x.clone()));
                tn.borrow().right.as_ref().map(|x| p.push_back(x.clone()));
                r = Some(tn.borrow().val);
            }
            q.append(&mut p);
            r.map(|x| res.push(x));
        }
        res
    }

    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn rec(root: &Rc<RefCell<TreeNode>>) -> Option<(i32, i32)> {
            let val = root.borrow().val;
            match (root.borrow().left.as_ref(), root.borrow().right.as_ref()) {
                (None, None) => Some((val, val)),
                (None, Some(r)) => {
                    let (kl, kr) = rec(r)?;
                    if kl <= val || kr <= val {
                        None
                    } else {
                        Some((val, kr))
                    }
                }
                (Some(l), None) => {
                    let (kl, kr) = rec(l)?;
                    if kl >= val || kr >= val {
                        None
                    } else {
                        Some((kl, val))
                    }
                }
                (Some(l), Some(r)) => {
                    let (kl, kr) = rec(l)?;
                    let (tl, tr) = rec(r)?;
                    if kl >= val || kr >= val || tl <= val || tr <= val {
                        None
                    } else {
                        Some((kl, tr))
                    }
                }
            }
        }
        root.as_ref().map_or(true, |x| rec(x).is_some())
    }

    pub fn leaf_similar(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        fn rec1(root: &Rc<RefCell<TreeNode>>, acc: &mut Vec<i32>) {
            match (root.borrow().left.as_ref(), root.borrow().right.as_ref()) {
                (None, None) => acc.push(root.borrow().val),
                (None, Some(r)) => rec1(r, acc),
                (Some(l), None) => rec1(l, acc),
                (Some(l), Some(r)) => {
                    rec1(l, acc);
                    rec1(r, acc);
                }
            }
        }
        fn rec2(root: &Rc<RefCell<TreeNode>>, acc: &mut Vec<i32>) -> bool {
            match (root.borrow().left.as_ref(), root.borrow().right.as_ref()) {
                (None, None) => acc.pop().map_or(false, |v| v == root.borrow().val),
                (None, Some(r)) => rec2(r, acc),
                (Some(l), None) => rec2(l, acc),
                (Some(l), Some(r)) => rec2(l, acc) && rec2(r, acc),
            }
        }
        match (root1, root2) {
            (None, None) => true,
            (Some(l), Some(r)) => {
                let mut acc = Vec::new();
                rec1(&l, &mut acc);
                acc.reverse();
                rec2(&r, &mut acc) && acc.is_empty()
            }
            _ => false,
        }
    }

    pub fn delete_node(
        root: Option<Rc<RefCell<TreeNode>>>,
        key: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        todo!()
    }
}

struct BSTIterator {
    tree: Vec<Rc<RefCell<TreeNode>>>,
}

impl BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut tree: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
        if let Some(r) = root {
            Self::rec(r, &mut tree);
        }
        Self { tree }
    }

    fn next(&mut self) -> i32 {
        let n = self.tree.pop().unwrap();
        let res = n.borrow().val;
        n.borrow()
            .right
            .as_ref()
            .map(|x| Self::rec(x.clone(), &mut self.tree));
        res
    }

    fn has_next(&self) -> bool {
        !self.tree.is_empty()
    }

    fn rec(root: Rc<RefCell<TreeNode>>, tree: &mut Vec<Rc<RefCell<TreeNode>>>) {
        let l = root.borrow().left.clone();
        root.borrow_mut().left = None;
        tree.push(root);
        l.map(|x| Self::rec(x, tree));
    }
}

pub fn hs_main() {
    let mat = vec![vec![1], vec![2]];
    println!("{:?}", Solution::spiral_order(mat));
    let mut mat = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
    Solution::set_zeroes(&mut mat);
    println!("{:?}", mat);
}
