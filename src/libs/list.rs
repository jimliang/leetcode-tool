#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn from_jsonstr(s: &str) -> Option<Box<ListNode>> {
        let value = serde_json::from_str::<serde_json::Value>(s).ok()?;
        if let serde_json::Value::Array(array) = value {
            let list = array.into_iter().map(|item| match item.as_i64() {
                Some(v) => v as i32,
                None => 0,
            });
            // .collect();
            // build_node(list)
            Self::from_iter(list)
        } else {
            None
        }
    }

    pub fn into_iter(n: Option<Box<ListNode>>) -> impl Iterator<Item = i32> {
        let mut node = n;
        (0..).map_while(move |_| {
            if let Some(mut n) = node.take() {
                node = n.next.take();
                Some(n.val)
            } else {
                None
            }
        })
    }

    pub fn from_iter<I: IntoIterator<Item = i32>>(list: I) -> Option<Box<ListNode>> {
        let mut iter = list.into_iter();
        match iter.next() {
            Some(val) => {
                let mut node = Box::new(ListNode::new(val));
                node.next = Self::from_iter(iter);
                Some(node)
            }
            None => None,
        }
    }
    pub fn from_iter_rev<I: IntoIterator<Item = i32>>(list: I) -> Option<Box<ListNode>> {
        let mut iter = list.into_iter();
        let mut root: Option<Box<ListNode>> = None;
        for val in iter {
            root = Some(Box::new(ListNode { val, next: root }));
        }
        root
    }
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl std::fmt::Display for ListNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", build_list(Some(Box::new(self.clone()))))
    }
}

pub fn build_list(node: Option<Box<ListNode>>) -> Vec<i32> {
    let mut list = vec![];
    let mut cur = &node;
    while let Some(c) = cur {
        list.push(c.val);
        cur = &c.next;
    }
    list
}
