pub struct Solution;
use crate::TreeNode;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::iter::once;
use std::rc::Rc;

impl Solution {
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut q: VecDeque<_> = once(root).flatten().collect();
        let mut result = vec![];
        while !q.is_empty() {
            let (mut sum, n) = (0.0, q.len());
            for _ in 0..n {
                let node_rc = q.pop_front().unwrap();
                let mut node_ref = node_rc.borrow_mut();
                sum += node_ref.val as f64;
                q.extend(
                    once(node_ref.left.take())
                        .chain(once(node_ref.right.take()))
                        .flatten(),
                );
            }
            result.push(sum / (n as f64));
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        let tree = TreeNode::from_vec(&[3, 9, 20, i32::MIN, i32::MIN, 15, 7]);
        assert_eq!(Solution::average_of_levels(tree), vec![3.0, 14.5, 11.0]);
    }
}
