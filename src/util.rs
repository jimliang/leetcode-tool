use std::borrow::Cow;

use lazy_static::lazy_static;
use regex::Regex;

pub fn parse_struct_name(s: &str) -> Option<&str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"struct ([\S]+)").unwrap();
    }
    let mut iter = RE.captures_iter(s);
    if let Some(caps) = iter.next() {
        return caps.get(1).map(|v| v.as_str());
    }

    None
}

pub fn get_title_slug<'a>(s: &'a str) -> Cow<'a, str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"leetcode-cn.com/problems/(\S+)").unwrap();
    }
    let mut iter = RE.captures_iter(s);
    if let Some(caps) = iter.next() {
        let v = caps.get(1).unwrap();
        Cow::Borrowed(v.as_str())
    } else {
        Cow::Borrowed(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_struct_name() {
        let name = parse_struct_name("pub struct ListNode {\n//   pub val: i32,\n//   pub next:");
        println!("{:?}", name);
    }
    #[test]
    fn test_get_title_slug() {
        let name = get_title_slug("leetcode-cn.com/problems/abc-def aaa");
        assert_eq!(name, "abc-def");
    }

    #[test]
    fn test_get_title_slug2() {
        let name = get_title_slug("abc-def");
        assert_eq!(name, "abc-def");
    }
}
