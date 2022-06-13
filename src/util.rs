use lazy_static::lazy_static;
use regex::Regex;

pub fn parse_struct_name(s: &str) -> Option<&str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"struct ([\S]+)").unwrap();
    }
    for caps in RE.captures_iter(s) {
        return caps.get(1).map(|v|v.as_str())
    }

    None
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_struct_name() {
        let name = parse_struct_name("pub struct ListNode {\n//   pub val: i32,\n//   pub next:");
        println!("{:?}", name);
    }


}