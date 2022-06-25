use std::borrow::Cow;

use lazy_static::lazy_static;
use regex::Regex;

pub fn guest_output<'a>(s: &'a str) -> impl Iterator<Item = Cow<'a, str>> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"输出(:|：)\s?(\S+)").unwrap();
    }

    let iter = RE.captures_iter(s);

    iter.map(|caps| {
        let st = caps.get(2).unwrap().as_str();
        pure_output(st)
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
}
