use anyhow::{bail, Ok, Result};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClassTestCase {
    name: String,
    params: Vec<String>,
}

pub fn parse_test_cases<'a>(s: &'a str) -> Result<impl Iterator<Item = serde_json::Value> + 'a> {
    let sp = s.split('\n');
    // Ok(sp.map(|b |b.to_owned()))
    Ok(sp.map(|v| serde_json::from_str(v).unwrap()))
}

#[deprecated]
pub fn parse_class_test_cases(s: &str) -> Result<impl Iterator<Item = ClassTestCase>> {
    let mut sp = s.split('\n');

    let n1 = sp.next();
    let n2 = sp.next();
    if let (Some(line1), Some(line2)) = (n1, n2) {
        let (mut name, mut params) = {
            let name: Vec<String> = serde_json::from_str(line1)?;
            let params: Vec<Vec<String>> = serde_json::from_str(line2)?;
            (name.into_iter(), params.into_iter())
        };

        let iter = (0..).map_while(move |_| match (name.next(), params.next()) {
            (Some(name), Some(params)) => Some(ClassTestCase { name, params }),
            _ => None,
        });

        Ok(iter)
    } else {
        bail!("parse test cases error")
    }
}

pub fn parse_class_test_cases2<'a>(s: &'a str) -> Option<(&'a str, &'a str)> {
    let mut sp = s.split('\n');

    let n1 = sp.next();
    let n2 = sp.next();
    if let (Some(line1), Some(line2)) = (n1, n2) {
        Some((line1, line2))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let a = parse_class_test_cases("[\"AllOne\",\"inc\",\"inc\",\"getMaxKey\",\"getMinKey\",\"inc\",\"getMaxKey\",\"getMinKey\"]\n[[],[\"hello\"],[\"hello\"],[],[],[\"leet\"],[],[]]").unwrap();
        println!("--> testcases: {:?}", a.collect::<Vec<_>>())
    }
    #[test]
    fn test_parse2() {
        let a = parse_test_cases("\"ab\"\n\"ba\"\n\"ab\"\n\"ab\"\n\"aa\"\n\"aa\"").unwrap();
        println!("--> testcases: {:?}", a.collect::<Vec<_>>())
    }

    #[test]
    fn test_parse3() {
        let a = parse_test_cases("[1,2,3]\n[4,2,9,3,5,null,7]\n[21,7,14,1,1,2,2,3,3]").unwrap();
        println!("--> testcases: {:?}", a.collect::<Vec<_>>())
    }
    #[test]
    fn test_parse4() {
        let a = parse_test_cases("[[17,2,17],[16,16,5],[14,3,19]]\n[[7,6,2]]").unwrap();
        for i in a {
            println!("--> testcases: {:?}", i)
        }
    }
}
