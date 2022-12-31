// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut l1 = l1.clone();
    let mut l2 = l2.clone();
    let mut head = Box::new(ListNode::new(0));
    let mut current = &mut head;
    let mut carry = 0;
    let mut v1 = 0;
    let mut v2 = 0;
    while l1.is_some() || l2.is_some() || carry != 0 {
        v1 = 0;
        v2 = 0;

        if let Some(node) = l1 {
            v1 = node.val;
            l1 = node.next;
        }
        if let Some(node) = l2 {
            v2 = node.val;
            l2 = node.next;
        }

        current.next = Some(Box::new(ListNode::new((v1 + v2 + carry) % 10)));
        current = current.next.as_mut().unwrap();
        carry = (v1 + v2 + carry) / 10;
    }

    head.next
}

pub fn add_two_numbers_top(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (l1, l2) {
        (None, None) => None,
        (Some(n), None) | (None, Some(n)) => Some(n),
        (Some(n1), Some(n2)) => {
            let sum = n1.val + n2.val;
            if sum < 10 {
                Some(Box::new(ListNode {
                    val: sum,
                    next: add_two_numbers_top(n1.next, n2.next),
                }))
            } else {
                let carry = Some(Box::new(ListNode::new(1)));
                Some(Box::new(ListNode {
                    val: sum - 10,
                    next: add_two_numbers_top(add_two_numbers_top(carry, n1.next), n2.next),
                }))
            }
        }
    }
}
fn main() {
    let input_l1 = vec![9, 9, 9, 9, 9, 9, 9];
    let input_l2 = vec![9, 9, 9, 9];

    fn case_1() {
        let listNode_1_1 = ListNode::new(1);
        let mut listNode_1_2 = ListNode::new(3);
        listNode_1_2.next = Some(Box::new(listNode_1_1));
        let listNode_2 = ListNode::new(8);

        let case_1_input_l1 = Some(Box::new(listNode_1_2));
        let case_1_input_l2 = Some(Box::new(listNode_2));

        let case_1_output = add_two_numbers(case_1_input_l1, case_1_input_l2);
        print!("case 1 output: {:?}", case_1_output);
    }
    case_1();
}
