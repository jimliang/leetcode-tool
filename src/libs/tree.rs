use std::{cell::RefCell, collections::VecDeque, rc::Rc};

#[derive(PartialEq, Eq)]
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

    pub fn from_jsonstr(s: &str) -> Option<Rc<RefCell<TreeNode>>> {
        let value = serde_json::from_str::<serde_json::Value>(s).ok()?;
        if let serde_json::Value::Array(array) = value {
            build_tree(
                array
                    .into_iter()
                    .map(|item| item.as_i64().map(|v| v as i32)),
            )
        } else {
            None
        }
    }
}

impl std::fmt::Debug for TreeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            // "[ val: {}, left: {}, right: {} ]",
            "{} ({}, {})",
            self.val,
            match self.left {
                Some(ref l) => format!("{:?}", l.borrow()),
                None => String::from("null"),
            },
            match self.right {
                Some(ref l) => format!("{:?}", l.borrow()),
                None => String::from("null"),
            }
        )
    }
}

pub type TreeNodeW = Rc<RefCell<TreeNode>>;

fn map_val(val: Option<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    val.and_then(|item| item.map(|val| Rc::new(RefCell::new(TreeNode::new(val)))))
}

pub struct RawTree(Vec<Option<i32>>);

impl RawTree {
    pub fn from_jsonstr(s: &str) -> Self {
        Self(serde_json::from_str(s).unwrap())
    }
    pub fn to_jsonstr(&self) -> String {
        serde_json::to_string(&self.0).unwrap()
    }
}

pub fn build_tree<I: IntoIterator<Item = Option<i32>>>(list: I) -> Option<TreeNodeW> {
    let mut iter = list.into_iter();
    let root = match iter.next() {
        Some(Some(val)) => Rc::new(RefCell::new(TreeNode::new(val))),
        _ => return None,
    };
    let mut queue: VecDeque<TreeNodeW> = VecDeque::new();
    queue.push_back(root.clone());
    while let Some(node) = queue.pop_front() {
        let left = map_val(iter.next());
        let right = map_val(iter.next());
        if let Some(ref l) = left {
            queue.push_back(l.clone());
        }
        if let Some(ref r) = right {
            queue.push_back(r.clone());
        }
        let mut node_mut = node.borrow_mut();
        node_mut.left = left;
        node_mut.right = right;
    }
    Some(root)
}

pub fn format_tree(tree: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
    let mut list = vec![];
    if let Some(root) = tree {
        let mut queue = VecDeque::new();
        list.push(Some(root.borrow().val));
        queue.push_back(root);
        while let Some(node) = queue.pop_front() {
            let mut node_borrow = node.borrow_mut();
            match node_borrow.left.take() {
                Some(left) => {
                    list.push(Some(left.borrow().val));
                    queue.push_back(left);
                }
                None => {
                    list.push(None);
                }
            }
            match node_borrow.right.take() {
                Some(right) => {
                    list.push(Some(right.borrow().val));
                    queue.push_back(right);
                }
                None => {
                    list.push(None);
                }
            }
        }
        while let Some(None) = list.last() {
            let _ = list.pop();
        }
    }
    list
}

#[test]
fn serde_tree() {
    let s = "[5,4,2,3,3,7]";
    let tree = TreeNode::from_jsonstr(s);
    let list = format_tree(tree);
    assert_eq!(s, serde_json::to_string(&list).unwrap());
}

#[test]
fn serde_tree2() {
    let s = "[3,9,20,null,null,15,7]";
    let tree = TreeNode::from_jsonstr(s);
    // println!("{:?}", tree);
    let list = format_tree(tree);
    assert_eq!(s, serde_json::to_string(&list).unwrap());
}
