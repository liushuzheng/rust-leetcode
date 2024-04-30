use std::cell::RefCell;
use std::rc::Rc;

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

    pub fn all_possible_fbt(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut full_binary_trees: Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::new();
        // if n % 2 == 0 {
        //     return full_binary_trees;
        // }
        // if n == 1 {
        //     full_binary_trees.push(Some(Rc::new(RefCell::new(TreeNode::new(0)))));
        //     return full_binary_trees;
        // }
        // for i in (1..n).step_by(2) {
        //     let left_subtrees = Self::all_possible_fbt(i);
        //     let right_subtrees = Self::all_possible_fbt(n - 1 - i);
        //     let rcl = Rc::new(left_subtrees);
        //     let rcr = Rc::new(right_subtrees);
        //     for left_subtree in &rcl.to_vec() {
        //         let rc = left_subtree.unwrap().clone();
        //         for right_subtree in rcr.to_vec() {
        //             let root = Rc::new(RefCell::new(TreeNode { val: 0, left:Some(rc.clone()), right: right_subtree}));
        //             full_binary_trees.push(Some(root.clone()));
        //         }
        //     }
        // }
        full_binary_trees
    }
}
#[test]
fn  test_fn(){
    let vec = TreeNode::all_possible_fbt(2);

}