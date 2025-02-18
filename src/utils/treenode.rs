use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
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

    pub fn from_vec(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() {
            return None;
        }
        let mut root = Self::new(nums[0]);
        root.fill(nums, 0);
        Some(Rc::new(RefCell::new(root)))
    }

    fn fill(&mut self, nums: &[i32], index: usize) {
        let left_node = index * 2 + 1;
        if left_node < nums.len() && nums[left_node] != i32::MIN {
            self.left = Some(Rc::new(RefCell::new(Self::new(nums[left_node]))));
            self.left
                .as_ref()
                .unwrap()
                .borrow_mut()
                .fill(nums, left_node)
        }

        let right_node = left_node + 1;

        if right_node < nums.len() && nums[right_node] != i32::MIN {
            self.right = Some(Rc::new(RefCell::new(Self::new(nums[right_node]))));
            self.right
                .as_ref()
                .unwrap()
                .borrow_mut()
                .fill(nums, right_node)
        }
    }
}
