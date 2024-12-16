// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        Self { next: None, val }
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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn create_list() {
        let node_vec = vec![1, 2, 4, 5, 7, 9];
        let mut list = ListNode::from_vec(&node_vec);
        let mut i = 0;
        while let Some(head) = list {
            let val = head.val;
            assert_eq!(val, node_vec[i]);
            list = head.next;
            i += 1;
        }
    }
}
