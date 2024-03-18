// Definition for a binary tree node.
use std::rc::Rc;
use std::cell::RefCell;

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

pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
    // fn range_sum_a(node: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
    //
    //     match node {
    //         Some(n) => {
    //             let val = n.borrow_mut().val;
    //             if val < low {
    //                 range_sum_a(n.borrow().right, low, high)
    //             } else if val > high {
    //                 range_sum_a(n.borrow().left, low, high)
    //             } else {
    //                 val + range_sum_a(n.borrow().left, low, high) + range_sum_a(n.borrow().right, low, high)
    //             }
    //         }
    //         None => 0,
    //     }
    // }
    // range_sum_a(root, low, high)
    0
}
