use crate::TreeNode;
use std::cell::RefCell;
use std::cmp;
use std::rc::Rc;
pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        Some(node) => {
            let mut node = node.borrow_mut();
            1 + cmp::max(
                super::max_depth(node.left.take()),
                super::max_depth(node.right.take()),
            )
        }
        None => 0,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        let tree = TreeNode::from_vec(&[3, 9, 20, i32::MIN, i32::MIN, 15, 7]);
        assert_eq!(max_depth(tree), 3)
    }
}
