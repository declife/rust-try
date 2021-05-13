#[cfg(test)]
mod tests {

    use std::char;
    use core::option;
    use std::cell::RefCell;
    use std::rc::Rc;


    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn add_binary_test() {

        let a = "100".to_string();
        let b = "110010".to_string();
        // let ans = add_binary(a, b);
        // assert_eq!(ans, "11010".to_string());
        let bb = add_binary_op(a, b);
        assert_eq!(bb, "110110".to_string());
    }

    /// 字符串过长后会有问题
    fn add_binary(a: String, b: String) -> String {

        format!("{:b}", u32::from_str_radix(&a, 2).unwrap() + u32::from_str_radix(&b, 2).unwrap())
    }

    /// 优化版本
    fn add_binary_op(a: String, b: String) -> String {
        let mut carry = 0;
        let mut ans = Vec::new();
        // let a_vec.len() = a.len();
        // let b_vec.len() = b.len();
        let a_vec = a.into_bytes();
        let b_vec = b.into_bytes();
        let (prefix, a_suffix, b_suffix) = if a_vec.len() > b_vec.len() {
            let len = a_vec.len() - b_vec.len();
            (&a_vec[..len], &a_vec[len..], &b_vec[..])
        } else {
            let len = b_vec.len() - a_vec.len();
            (&b_vec[..len], &a_vec[..], &b_vec[len..])
        };

        for index in (0..a_suffix.len()).rev() {
            let cur = a_suffix[index] as u32 - 48 + b_suffix[index] as u32 - 48 + carry;
            carry = cur >> 1;
            let out = char::from_digit(cur & 1, 2).unwrap();
            ans.push(zero_or_one(cur & 1));
        }

        for index in (0..prefix.len()).rev() {
            if carry == 0 {
                // for ch in prefix[..=index].iter().rev() {
                //     ans.push(zero_or_one(*ch as u32 - 48));
                // }
                prefix[..=index].iter().rev().for_each(|ch| ans.push(zero_or_one(*ch as u32 - 48))); //看看这个是啥
                // ans = ans + prefix[..=index].iter().rev().collect::<String>();
                // ans = format!("{}{}", &ans, prefix[..=index].iter().rev().collect::<String>());
                break;
            }
            let cur = (prefix[index] as u32 - 48) + carry;
            carry = cur >> 1;
            ans.push(zero_or_one(cur & 1));
        }

        if carry == 1 {
            ans.push('1');
        }
        ans.into_iter().rev().collect()
    }

    fn zero_or_one(n: u32) -> char {
        if n == 1 {
            '1'
        } else {
            '0'
        }
    }

    /// 平方根
    /// 测试
    #[test]
    fn my_sqrt_test() {
        assert_eq!(my_sqrt(2147483647), 46340)
    }

    fn my_sqrt(x: i32) -> i32 {
        if x < 0 {
            -1
        } else {
            let mut l = 0;
            let mut r = 46340;

            while l < r {
                let mid = (r - l) / 2 + l;
                let mid_sqr = mid * mid;
                if mid_sqr == x {
                    return mid;
                }
                if mid_sqr < x {
                    l = mid + 1;
                } else {
                    r = mid - 1;
                }
            }
            if l * l > x {
                l - 1
            } else {
                l
            }
            // for i in 0..=x {
            //     if  i * i == x {
            //         return i;
            //     } else if i * i > x {
            //         return i - 1;
            //     }
            // }
            // -1
        }
    }

    /// 牛顿迭代: xi = 0.5(x0+c/x0)...
    pub fn my_sqrt_op(x: i32) -> i32 {
        if x == 0{
            return 0i32;
        }

        let c = x as f64;
        let mut  x0 = c;
        loop{
            let xi = 0.5 * (x0 + c/x0);
            if (x0 - xi) < 1e-7{
                break;
            }
            x0 = xi;
        }

        x0 as i32
    }

    #[test]
    fn climb_stairs_test() {
        climb_stairs(2);
        assert_eq!(2, climb_stairs(2));
        assert_eq!(3, climb_stairs(3));
        assert_eq!(3, climb_stairs_op(3));
    }

    fn climb_stairs(n: i32) -> i32 {
        if n < 0 {
            0
        } else if n == 1 {
            1
        } else if n == 2 {
            2
        } else {
            climb_stairs(n - 1) + climb_stairs(n - 2)
        }
    }

    ///快速幂矩阵
    fn climb_stairs_op(n: i32) -> i32 {
        if n < 0 {
            0
        } else if n == 1 {
            1
        } else {
            let a = vec![vec![1, 1], vec![1, 0]];
            let _final = pow(a, n);
            _final[0]
        }
    }

    fn pow(mut v: Vec<Vec<i32>>, mut n: i32) -> Vec<i32> {
        let mut ans = vec![vec![1, 0],vec![0, 1]];
        while n > 0 {
            if n & 1 != 0 {
                ans = multiply(&ans, &v);
            }
            v = multiply(&v, &v);
            n >>= 1;
        }
        // ans = multiply(&ans, &v);
        vec![ans[0][0], ans[0][1]]
    }

    fn multiply(v: &Vec<Vec<i32>>, s: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut re = vec![vec![0; 2], vec![0; 2]];
        for i in 0..2 {
            for j in 0..2 {
                re[i][j] = v[i][0] * s[0][j] + v[i][1] * s[1][j];
            }
        }
        re
    }

///--------------------------------------------------------------------------------------------------///

    /// Definition for singly-linked list.
    #[derive(PartialEq, Eq, Clone, Debug)]
    pub struct ListNode {
      pub val: i32,
      pub next: Option<Box<ListNode>>
    }

    impl ListNode {
      #[inline]
      fn new(val: i32) -> Self {
        ListNode {
          next: None,
          val
        }
      }
    }

    // fn delete_duplicates_test(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    //     if head.is_none() {
    //         return head;
    //     }
    //     let mut head = head;
    //     let mut ans = Some(Box::new(ListNode::new(head.as_ref().unwrap().val)));
    //
    //     let mut cur = &head.as_ref().unwrap().next;
    //     let mut real_cur = &mut ans;
    //     loop {
    //         let cur_val = match cur {
    //             Some(val) => val,
    //             None => break
    //         };
    //
    //
    //
    //         if real_cur.as_ref().unwrap().val != cur_val.val {
    //             let next = Some(Box::new(ListNode::new(cur_val.val)));
    //             match real_cur {
    //                 None => break,
    //                 Some(ref mut rc) => rc.next = next
    //             }
    //             real_cur = if let Some(rc) = real_cur {
    //                 rc.next
    //             };
    //             // real_cur = Some(Box::new(ListNode::new(cur_val.val)));
    //             // real_cur = &mut real_cur.as_ref().unwrap().borrow_mut().next;
    //         }
    //         cur = &cur.as_ref().unwrap().next;
    //     }
    //     ans
    // }

    fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }
        let mut head = head;
        let mut ans = Some(Box::new(ListNode::new(head.as_ref().unwrap().val)));

        let mut cur = &head.as_ref().unwrap().next;
        let mut real_cur = ans.get_or_insert(Box::new(ListNode::new(0)));
        loop {
            let cur_val = match cur {
                Some(val) => val,
                None => break
            };



            if real_cur.val != cur_val.val {
                // let next = Some(Box::new(ListNode::new(cur_val.val)));
                real_cur = real_cur.next.get_or_insert(Box::new(ListNode::new(cur_val.val)));
                // real_cur = &mut match real_cur {
                //     None => None,
                //     Some(rc) => rc.next
                // }
                // real_cur = Some(Box::new(ListNode::new(cur_val.val)));
                // real_cur = &mut real_cur.as_ref().unwrap().borrow_mut().next;
            }
            cur = &cur.as_ref().unwrap().next;
        }
        ans
    }

    fn delete_duplicates_op(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut res = ListNode {val: 0, next: head }; // head不可变，但是包一层就可变了

        let mut p = &mut res.next;
        while let Some(node) = p {
            while node.next.is_some()
                && node.val == node.next.as_ref().unwrap().val {
                let next = node.next.take();
                node.next = next.unwrap().next.take();
            }
            p = &mut node.next;
        }

        res.next.take()
    }
    ///---------------------------------------------------------------------------------------------///
    /// 相同的树
    ///
    /// // Definition for a binary tree node.
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
          right: None
        }
      }
    }

    fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn is_same_tree_in(p: &Option<Rc<RefCell<TreeNode>>>, q: &Option<Rc<RefCell<TreeNode>>>) -> bool {
            if p.is_none() && q.is_none() {
                true
            } else if p.is_none() || q.is_none() || p.as_ref().unwrap().borrow().val != q.as_ref().unwrap().borrow().val {
                false
            } else {
                is_same_tree_in(&p.as_ref().unwrap().borrow().left.clone(), &q.as_ref().unwrap().borrow().left) &&
                    is_same_tree_in(&p.as_ref().unwrap().borrow().right, &q.as_ref().unwrap().borrow().right)
            }
        }
        is_same_tree_in(&p, &q)
    }

    fn is_same_tree_by_stack(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut vec_p = vec![p.clone()];
        let mut vec_q = vec![q.clone()];

        while !(vec_p.is_empty() && vec_q.is_empty()) {
            let pp = vec_p.remove(0);
            let qq = vec_q.remove(0);
            if pp.is_none() && qq.is_none() {
               continue;
            } else if pp.is_none() || qq.is_none() || pp.as_ref().unwrap().borrow().val != qq.as_ref().unwrap().borrow().val {
                return false;
            }
            vec_p.push(pp.as_ref().unwrap().borrow().left.clone());
            vec_p.push(pp.as_ref().unwrap().borrow().right.clone());
            vec_q.push(qq.as_ref().unwrap().borrow().left.clone());
            vec_q.push(qq.as_ref().unwrap().borrow().right.clone());
        }
        true
    }


    /// 使用递归很好写,深度优先搜素,
    /// 使用广度优先的话,就每循环队列存储同一行(高度)所有节点
    fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0
        }
        i32::max(1 + max_depth(root.as_ref().unwrap().borrow().left.clone()), 1 + max_depth(root.as_ref().unwrap().borrow().right.clone()))
    }

    ///
}
