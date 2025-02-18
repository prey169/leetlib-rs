#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn from_vec(vec: &[i32]) -> Option<Box<ListNode>> {
        let mut result = None;
        for entry in vec.iter().rev() {
            let mut node = Self::new(*entry);
            node.next = result;
            result = Some(Box::new(node));
        }
        result
    }
}
