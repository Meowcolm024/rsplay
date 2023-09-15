struct Solution;

#[derive(Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

type List = Option<Box<ListNode>>;

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    fn from_list(xs: Vec<i32>) -> Option<Box<ListNode>> {
        let mut res = None;
        for x in xs.into_iter().rev() {
            res = Some(Box::new(ListNode { val: x, next: res }));
        }
        res
    }
}

impl Solution {
    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut res = None;
        while head.is_some() {
            std::mem::swap(&mut head.as_mut().unwrap().next, &mut res);
            std::mem::swap(&mut head, &mut res);
        }
        res
    }

    pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            head
        } else {
            let mut odd = head;
            let mut even = None;
            let mut cur = None;
            std::mem::swap(&mut odd.as_mut().unwrap().next, &mut even);
            std::mem::swap(&mut even.as_mut().unwrap().next, &mut cur);
            let mut ot = &mut odd;
            let mut et = &mut even;
            let mut oe = true;
            while cur.is_some() {
                if oe {
                    std::mem::swap(
                        &mut ot.as_mut().unwrap().next,
                        &mut cur.as_mut().unwrap().next,
                    );
                    std::mem::swap(&mut ot.as_mut().unwrap().next, &mut cur);
                    ot = &mut ot.as_mut().unwrap().next;
                } else {
                    std::mem::swap(
                        &mut et.as_mut().unwrap().next,
                        &mut cur.as_mut().unwrap().next,
                    );
                    std::mem::swap(&mut et.as_mut().unwrap().next, &mut cur);
                    et = &mut et.as_mut().unwrap().next;
                }
                oe = !oe;
            }
            ot.as_mut().unwrap().next = even;
            odd
        }
    }

    pub fn reverse_between(
        head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        let mut head = Some(Box::new(ListNode { val: 0, next: head }));
        let mut tail = &mut head;
        for _ in 0..left - 1 {
            tail = &mut tail.as_mut().unwrap().next;
        }
        let mut hd = None;
        std::mem::swap(&mut hd, &mut tail.as_mut().unwrap().next);
        let mut rest = &mut hd;
        for _ in left..right + 1 {
            rest = &mut rest.as_mut().unwrap().next;
        }
        let mut res = None;
        std::mem::swap(&mut res, &mut rest);
        while hd.is_some() {
            std::mem::swap(&mut hd.as_mut().unwrap().next, &mut res);
            std::mem::swap(&mut hd, &mut res);
        }
        if tail.is_none() {
            res
        } else {
            tail.as_mut().unwrap().next = res;
            head.unwrap().next
        }
    }

    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut res = String::with_capacity(word1.len() + word2.len());
        let l = std::cmp::min(word1.len(), word2.len());
        for i in 0..l {
            res.push(word1.as_bytes()[i] as char);
            res.push(word2.as_bytes()[i] as char);
        }
        if word1.len() > word2.len() {
            res.push_str(&word1[l..]);
        } else {
            res.push_str(&word2[l..]);
        }
        res
    }

    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut res = head;
        let mut len = 0;
        let mut cur = &res;
        while let Some(r) = cur.as_ref() {
            len += 1;
            cur = &r.next;
        }
        let mut i = 0;
        while i + k <= len {
            res = Solution::reverse_between(res, i + 1, i + k);
            i += k;
        }
        res
    }

    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        use std::collections::VecDeque;
        let mut lists: Vec<Option<Box<ListNode>>> =
            lists.into_iter().filter(|x| x.is_some()).collect();
        lists.sort_unstable_by_key(|k| k.as_ref().unwrap().val);
        let mut lists: VecDeque<Option<Box<ListNode>>> = lists.into_iter().collect();
        let mut res = None;
        while !lists.is_empty() {
            let mut hd = lists.pop_front().unwrap();
            let mut tl = None;
            if hd.as_ref().unwrap().next.is_some() {
                std::mem::swap(&mut hd.as_mut().unwrap().next, &mut tl);
            }
            std::mem::swap(&mut hd.as_mut().unwrap().next, &mut res);
            res = hd;
            if tl.is_some() {
                let mut i = 0;
                while i < lists.len() && lists[i].as_ref().unwrap().val <= tl.as_ref().unwrap().val
                {
                    i += 1;
                }
                lists.insert(i, tl);
            }
        }
        Solution::reverse_list(res)
    }

    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let dummy = Some(Box::new(ListNode { val: 0, next: head }));
        let mut fast = &dummy;
        let mut slow = &dummy;
        for _ in 0..(n + 1) {
            if let Some(f) = fast {
                fast = &f.next;
            } else {
                break;
            }
        }
        while fast.is_some() {
            fast = &fast.as_ref().unwrap().next;
            slow = &slow.as_ref().unwrap().next;
        }
        // delete slow.next
        drop(fast);
        unsafe {
            let sp = slow as *const Option<Box<ListNode>>;
            let sp = sp as *mut Option<Box<ListNode>>;
            let mut rest: Option<Box<ListNode>> = None;
            let snn = &mut sp
                .as_mut()
                .unwrap()
                .as_mut()
                .unwrap()
                .next
                .as_mut()
                .unwrap()
                .next;
            std::mem::swap(snn, &mut rest);
            sp.as_mut().unwrap().as_mut().unwrap().next = rest;
        }
        dummy.unwrap().next
    }

    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }
        let mut prev: Option<i32> = None;
        let mut i = 0;
        let mut res: Option<Box<ListNode>> = Some(Box::new(ListNode::new(0)));
        let mut tl = &mut res;
        while let Some(h) = head {
            let ListNode { val, next } = *h;
            match prev {
                Some(n) => {
                    if n == val {
                        i += 1;
                    } else {
                        if i == 1 {
                            tl.as_mut().unwrap().next = Some(Box::new(ListNode::new(n)));
                            tl = &mut tl.as_mut().unwrap().next;
                        }
                        prev = Some(val);
                        i = 1;
                    }
                }
                None => {
                    prev = Some(val);
                    i = 1;
                }
            }
            head = next
        }
        if let Some(n) = prev {
            if i == 1 {
                tl.as_mut().unwrap().next = Some(Box::new(ListNode::new(n)));
            }
        }
        res.unwrap().next
    }

    pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        fn len(mut head: &Option<Box<ListNode>>) -> i32 {
            let mut i = 0;
            while let Some(h) = head {
                head = &h.next;
                i += 1;
            }
            i
        }
        let l = len(&head);
        if l == 0 {
            return head;
        }
        let k = l - k % l;
        if k == 0 {
            return head;
        }
        let mut tl = &mut head;
        for _ in 0..k - 1 {
            tl = &mut tl.as_mut().unwrap().next;
        }
        let mut ret = Some(Box::new(ListNode::new(0)));
        std::mem::swap(
            &mut ret.as_mut().unwrap().next,
            &mut tl.as_mut().unwrap().next,
        );
        let mut rtl = &mut ret;
        while rtl.as_mut().unwrap().next.is_some() {
            rtl = &mut rtl.as_mut().unwrap().next;
        }
        rtl.as_mut().unwrap().next = head;
        ret.unwrap().next
    }

    fn split(mut head: List) -> (List, List) {
        if head.is_some() {
            let mut i = 0;
            let mut p = &head;
            while let Some(n) = p {
                p = &n.next;
                i += 1;
            }
            let mut ret = &mut head;
            for _ in 1..i / 2 {
                ret = &mut ret.as_mut().unwrap().next;
            }
            let mut res = None;
            std::mem::swap(&mut res, &mut ret.as_mut().unwrap().next);
            (head, res)
        } else {
            (None, None)
        }
    }

    fn merge(l: List, r: List) -> List {
        match (l, r) {
            (None, None) => None,
            (None, Some(r)) => Some(r),
            (Some(l), None) => Some(l),
            (Some(l), Some(r)) => {
                let x = l.val;
                let y = r.val;
                if x < y {
                    let ListNode { val: _, next: xs } = *l;
                    Some(Box::new(ListNode {
                        val: x,
                        next: Solution::merge(xs, Some(r)),
                    }))
                } else {
                    let ListNode { val: _, next: ys } = *r;
                    Some(Box::new(ListNode {
                        val: y,
                        next: Solution::merge(Some(l), ys),
                    }))
                }
            }
        }
    }

    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_some() {
            if head.as_ref().unwrap().next.is_none() {
                head
            } else {
                let (l, r) = Solution::split(head);
                Solution::merge(Solution::sort_list(l), Solution::sort_list(r))
            }
        } else {
            None
        }
    }

    pub fn partition(mut head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        if head.is_none() {
            head
        } else {
            let mut d1 = Some(Box::new(ListNode::new(0)));
            let mut ld1 = &mut d1;
            let mut d2 = Some(Box::new(ListNode::new(0)));
            let mut ld2 = &mut d2;
            let h = &mut head;
            while h.is_some() {
                let v = h.as_ref().unwrap().val;
                if v < x {
                    std::mem::swap(&mut ld1.as_mut().unwrap().next, h);
                    ld1 = &mut ld1.as_mut().unwrap().next;
                    std::mem::swap(&mut ld1.as_mut().unwrap().next, h);
                } else {
                    std::mem::swap(&mut ld2.as_mut().unwrap().next, h);
                    ld2 = &mut ld2.as_mut().unwrap().next;
                    std::mem::swap(&mut ld2.as_mut().unwrap().next, h);
                }
            }
            std::mem::swap(&mut ld1.as_mut().unwrap().next, &mut d2.unwrap().next);
            d1.unwrap().next
        }
    }
}

pub fn re_main() {
    // let ls = ListNode::from_list(vec![4, 3, 5, 1, 2]);
    // println!("{:?}", Solution::reverse_k_group(ls, 2));
    // // let ls = ListNode::from_list(vec![1, 2]);
    // // println!("{:?}", Solution::odd_even_list(ls));
    // // let ls = ListNode::from_list(vec![1, 2]);
    // // println!("{:?}", Solution::reverse_between(ls, 1, 2));
    // let ls = ListNode::from_list(vec![4, 3, 1, 2, 5]);
    // let rs = Solution::sort_list(ls);
    // println!("{:?}", rs);

    let ls = ListNode::from_list(vec![1, 4, 3, 2, 5, 2]);
    let rs = Solution::partition(ls, 3);
    println!("{:?}", rs);
}
