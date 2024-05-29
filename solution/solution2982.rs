/// 2982.找出出现至少三次的最长特殊子字符串 II
/// find-longest-special-substring-that-occurs-thrice-ii
/// <p>给你一个仅由小写英文字母组成的字符串 <code>s</code> 。</p>
///
/// <p>如果一个字符串仅由单一字符组成，那么它被称为 <strong>特殊 </strong>字符串。例如，字符串 <code>"abc"</code> 不是特殊字符串，而字符串 <code>"ddd"</code>、<code>"zz"</code> 和 <code>"f"</code> 是特殊字符串。</p>
///
/// <p>返回在 <code>s</code> 中出现 <strong>至少三次 </strong>的<strong> 最长特殊子字符串 </strong>的长度，如果不存在出现至少三次的特殊子字符串，则返回 <code>-1</code> 。</p>
///
/// <p><strong>子字符串 </strong>是字符串中的一个连续<strong> 非空 </strong>字符序列。</p>
///
/// <p>&nbsp;</p>
///
/// <p><strong class="example">示例 1：</strong></p>
///
/// <pre>
/// <strong>输入：</strong>s = "aaaa"
/// <strong>输出：</strong>2
/// <strong>解释：</strong>出现三次的最长特殊子字符串是 "aa" ：子字符串 "<em><strong>aa</strong></em>aa"、"a<em><strong>aa</strong></em>a" 和 "aa<em><strong>aa</strong></em>"。
/// 可以证明最大长度是 2 。
/// </pre>
///
/// <p><strong class="example">示例 2：</strong></p>
///
/// <pre>
/// <strong>输入：</strong>s = "abcdef"
/// <strong>输出：</strong>-1
/// <strong>解释：</strong>不存在出现至少三次的特殊子字符串。因此返回 -1 。
/// </pre>
///
/// <p><strong class="example">示例 3：</strong></p>
///
/// <pre>
/// <strong>输入：</strong>s = "abcaba"
/// <strong>输出：</strong>1
/// <strong>解释：</strong>出现三次的最长特殊子字符串是 "a" ：子字符串 "<em><strong>a</strong></em>bcaba"、"abc<em><strong>a</strong></em>ba" 和 "abcab<em><strong>a</strong></em>"。
/// 可以证明最大长度是 1 。
/// </pre>
///
/// <p>&nbsp;</p>
///
/// <p><strong>提示：</strong></p>
///
/// <ul>
/// 	<li><code>3 &lt;= s.length &lt;= 5 * 10<sup>5</sup></code></li>
/// 	<li><code>s</code> 仅由小写英文字母组成。</li>
/// </ul>
/// <a href="https://leetcode.cn/problems/find-longest-special-substring-that-occurs-thrice-ii/">找出出现至少三次的最长特殊子字符串 II</a>
pub struct Solution;

use std::cmp::Reverse;
use std::iter;

// 分类
impl Solution {
    pub fn maximum_length(s: String) -> i32 {
        let s = s.as_bytes();
        let mut char_map = vec![vec![]; 26];
        let mut spical_len = 0;
        for (index, &c) in s.iter().enumerate() {
            spical_len += 1;
            if index + 1 == s.len() || c != s[index + 1] {
                char_map[(c - b'a') as usize].push(Reverse(spical_len));
                spical_len = 0;
            }
        }

        let mut result = -1;

        for mut c in char_map {
            if c.is_empty() {
                continue;
            }
            c.sort_unstable();
            c.extend(iter::repeat(Reverse(0)).take(2));
            result = result
                .max(c[0].0 - 2) // 最长的 - 2
                .max(c[1].0.min(c[0].0 - 1)) // 第二长和第一长组合，以第二长为准
                .max(c[2].0); // 最短的
        }

        if result > 0 {
            result
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(Solution::maximum_length("aaaa".to_string()), 2);
    }
}
