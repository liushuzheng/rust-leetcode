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
}

// pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
//     if p.is_none() && q.is_none() {
//         return true;
//     }
//     if p.is_none() || q.is_none() {
//         return false;
//     }
// 
//     let rc1 = p.as_ref().unwrap().borrow();
//     let rc2 = q.as_ref().unwrap().borrow();
//     if rc1.val != rc2.val {
//         return false;
//     }
//     
//     is_same_tree(rc1.left.clone(),rc2.left.clone()) && is_same_tree(rc1.right.clone(),rc2.right.clone())
// }

#[test]
fn test_func(){
    
}
