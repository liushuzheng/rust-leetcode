//! # 回溯算法框架 
//! fn backtrack(state: &mut State, choices: &Vec<Choice>, res: &mut Vec<State>) {
//!     // 判断是否为解
//!     if is_solution(state) {
//!         // 记录
//!         record_solution(state, res);
//!         // 不再继续搜索
//!         return;
//!     }
//!     // 遍历所有选择
//!     for choice in choices {
//!         // 剪枝：判断选择是否合法
//!         if is_valid(state, choice) {
//!             // 尝试：做出选择，更新状态
//!             make_choice(state, choice);
//!             backtrack(state, choices, res);
//!             // 回退：撤销选择，恢复到之前的状态
//!             undo_choice(state, choice);
//!         }
//!     }
//! }

use std::cell::RefCell;
use std::rc::Rc;

/* 二叉树节点结构体 */
pub struct TreeNode {
    val: i32,                               // 节点值
    left: Option<Rc<RefCell<TreeNode>>>,    // 左子节点引用
    right: Option<Rc<RefCell<TreeNode>>>,   // 右子节点引用
}
 impl TreeNode {
    /* 构造方法 */
    fn new(val: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            val,
            left: None,
            right: None,
        }))
    }
}

pub fn pre_order(res: &mut Vec<Rc<RefCell<TreeNode>>>, root: Option<&Rc<RefCell<TreeNode>>>) {
    if root.is_none() {
        return;
    }

    if let Some(node) = root {
        if node.borrow().val == 7 {
            res.push(node.clone());
        }
        pre_order(res, node.borrow().left.as_ref());
        pre_order(res, node.borrow().right.as_ref());
    }
}

/* 前序遍历：例题二 */
pub fn pre_order_path(
    res: &mut Vec<Vec<Rc<RefCell<TreeNode>>>>,
    path: &mut Vec<Rc<RefCell<TreeNode>>>,
    root: Option<&Rc<RefCell<TreeNode>>>,
) {
    if root.is_none() {
        return;
    }
    if let Some(node) = root {
        // 尝试
        path.push(node.clone());
        if node.borrow().val == 7 {
            // 记录解
            res.push(path.clone());
        }
        pre_order_path(res, path, node.borrow().left.as_ref());
        pre_order_path(res, path, node.borrow().right.as_ref());
        // 回退
        path.pop();
    }
}

/* 前序遍历：例题三 */
pub fn pre_order_not_3(
    res: &mut Vec<Vec<Rc<RefCell<TreeNode>>>>,
    path: &mut Vec<Rc<RefCell<TreeNode>>>,
    root: Option<&Rc<RefCell<TreeNode>>>,
) {
    // 剪枝
    if root.is_none() || root.as_ref().unwrap().borrow().val == 3 {
        return;
    }
    if let Some(node) = root {
        // 尝试
        path.push(node.clone());
        if node.borrow().val == 7 {
            // 记录解
            res.push(path.clone());
        }
        pre_order_not_3(res, path, node.borrow().left.as_ref());
        pre_order_not_3(res, path, node.borrow().right.as_ref());
        // 回退
        path.pop();
    }
}



