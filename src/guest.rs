use std::borrow::Cow;

use lazy_static::lazy_static;
use regex::Regex;

pub fn guest_output<'a>(s: &'a str) -> impl Iterator<Item = Cow<'a, str>> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"输出(:|：)\s?(\S+)").unwrap();
    }

    let mut iter = s.split("\n");
    (0..).map_while(move |_| loop {
        match iter.next() {
            Some(line) if line.contains("输出") => {
                let mut captures_iter = RE.captures_iter(line);
                return match captures_iter.next() {
                    Some(caps) => {
                        let st = caps.get(2).unwrap().as_str();
                        Some(pure_output(st))
                    }
                    None => iter.next().map(|line| Cow::Borrowed(line)),
                };
            }
            Some(_) => {}
            None => return None,
        }
    })
}

fn pure_output<'a>(s: &'a str) -> Cow<'a, str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"<[^>]+>").unwrap();
    }

    RE.replace_all(s, "")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guest_output() {
        let content = "<p><strong>示例 1：</strong></p>\n\n<pre>\n<strong>输入: </strong>costs = [[17,2,17],[16,16,5],[14,3,19]]\n<strong>输出: </strong>10\n<strong>解释: </strong>将 0 号房子粉刷成蓝色，1 号房子粉刷成绿色，2 号房子粉刷成蓝色<strong>。</strong>\n&nbsp;    最少花费: 2 + 5 + 3 = 10。\n</pre>\n\n<p><strong>示例 2：</strong></p>\n\n<pre>\n<strong>输入: </strong>costs = [[7,6,2]]\n<strong>输出: 2</strong>\n</pre>\n\n<p>&nbsp;</p>\n\n<p><strong>提示:</strong></p>\n\n<ul>\n\t<li><code>costs.length == n</code></li>\n\t<li><code>costs[i].length == 3</code></li>\n\t<li><code>1 &lt;= n &lt;= 100</code></li>\n\t<li><code>1 &lt;= costs[i][j] &lt;= 20</code></li>\n</ul>\n\n<p>&nbsp;</p>\n\n<p><meta charset=\"UTF-8\" />注意：本题与主站 256&nbsp;题相同：<a href=\"https://leetcode-cn.com/problems/paint-house/\">https://leetcode-cn.com/problems/paint-house/</a></p>\n";
        let iter = guest_output(content);
        assert_eq!(iter.collect::<Vec<_>>(), vec!["10", "2"]);
    }
    #[test]
    fn test_guest_output2() {
        let content = "<p><strong>示例 1：</strong></p>\n\n<pre>\n<strong>输入</strong>\n[\"Solution\", \"pick\", \"pick\", \"pick\", \"pick\", \"pick\", \"pick\", \"pick\"]\n[[7, [2, 3, 5]], [], [], [], [], [], [], []]\n<strong>输出</strong>\n[null, 0, 4, 1, 6, 1, 0, 4]\n\n<b>解释\n</b>Solution solution = new Solution(7, [2, 3, 5]);\nsolution.pick(); // 返回0，任何[0,1,4,6]的整数都可以。注意，对于每一个pick的调用，\n                 // 0、1、4和6的返回概率必须相等(即概率为1/4)。\nsolution.pick(); // 返回 4\nsolution.pick(); // 返回 1\nsolution.pick(); // 返回 6\nsolution.pick(); // 返回 1\nsolution.pick(); // 返回 0\nsolution.pick(); // 返回 4\n</pre>\n\n<p>&nbsp;</p>\n\n<p><strong>提示:</strong></p>\n\n<ul>\n\t<li><code>1 &lt;= n &lt;= 10<sup>9</sup></code></li>\n\t<li><code>0 &lt;= blacklist.length &lt;= min(10<sup>5</sup>, n - 1)</code></li>\n\t<li><code>0 &lt;= blacklist[i] &lt; n</code></li>\n\t<li><code>blacklist</code>&nbsp;中所有值都 <strong>不同</strong></li>\n\t<li>&nbsp;<code>pick</code>&nbsp;最多被调用&nbsp;<code>2 * 10<sup>4</sup></code>&nbsp;次</li>\n</ul>\n";
        let iter = guest_output(content);
        assert_eq!(
            iter.collect::<Vec<_>>(),
            vec!["[null, 0, 4, 1, 6, 1, 0, 4]"]
        );
    }
}
