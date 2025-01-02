use crate::ListNode;

pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (list1, list2) {
        (Some(l1), None) => return Some(l1),
        (None, Some(l2)) => return Some(l2),
        (None, None) => return None,
        (Some(l1), Some(l2)) => {
            if l1.val <= l2.val {
                return Some(Box::new(ListNode {
                    next: merge_two_lists(l1.next, Some(l2)),
                    val: l1.val,
                }));
            } else {
                return Some(Box::new(ListNode {
                    next: merge_two_lists(Some(l1), l2.next),
                    val: l2.val,
                }));
            }
        }
    }
}

// pub fn merge_two_lists(
//     list1: Option<Box<ListNode>>,
//     list2: Option<Box<ListNode>>,
// ) -> Option<Box<ListNode>> {
//     match list1 {
//         Some(l1) => {
//             let mut head = ListNode::new(l1.val);
//             while head.next.is_some() {
//                 head.next = Some(Box::new(ListNode {
//                     next: l1.next,
//                     val: l1.val,
//                 }))
//             }
//             return Some(head);
//         }
//         None => (),
//     };
// }

/*
Example 1:
Input: list1 = [1,2,4], list2 = [1,3,4]
Output: [1,1,2,3,4,4]

Example 2:
Input: list1 = [], list2 = []
Output: []

Example 3:
Input: list1 = [], list2 = [0]
Output: [0]
*/
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn ex1() {
        let list1 = ListNode::from_vec(&vec![1, 2, 4]);
        let list2 = ListNode::from_vec(&vec![1, 3, 4]);
        let mut merged = merge_two_lists(list1, list2);
        let check = vec![1, 1, 2, 3, 4, 4];
        let mut i = 0;
        while let Some(head) = merged {
            let val = head.val;
            assert_eq!(val, check[i]);
            merged = head.next;
            i += 1;
        }
    }
}
