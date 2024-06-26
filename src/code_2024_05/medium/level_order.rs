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

pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let v = vec![];
    
    v
}

pub fn cur(root: Option<Rc<RefCell<TreeNode>>>, v: &Vec<Vec<i32>>)  {
   
    if root.is_some(){
        let rf = root.as_ref().unwrap().borrow();
       
        
    }

    
}
