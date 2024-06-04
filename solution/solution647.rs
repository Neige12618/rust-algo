/// 647.回文子串
/// palindromic-substrings
/// <p>给你一个字符串 <code>s</code> ，请你统计并返回这个字符串中 <strong>回文子串</strong> 的数目。</p>
///
/// <p><strong>回文字符串</strong> 是正着读和倒过来读一样的字符串。</p>
///
/// <p><strong>子字符串</strong> 是字符串中的由连续字符组成的一个序列。</p>
///
/// <p>&nbsp;</p>
///
/// <p><strong class="example">示例 1：</strong></p>
///
/// <pre>
/// <strong>输入：</strong>s = "abc"
/// <strong>输出：</strong>3
/// <strong>解释：</strong>三个回文子串: "a", "b", "c"
/// </pre>
///
/// <p><strong class="example">示例 2：</strong></p>
///
/// <pre>
/// <strong>输入：</strong>s = "aaa"
/// <strong>输出：</strong>6
/// <strong>解释：</strong>6个回文子串: "a", "a", "a", "aa", "aa", "aaa"</pre>
///
/// <p>&nbsp;</p>
///
/// <p><strong>提示：</strong></p>
///
/// <ul>
/// 	<li><code>1 &lt;= s.length &lt;= 1000</code></li>
/// 	<li><code>s</code> 由小写英文字母组成</li>
/// </ul>
/// <a href="https://leetcode.cn/problems/palindromic-substrings/">回文子串</a>
pub struct Solution;

impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let n = s.len();
        let mut ans = 0;
        let s = s.as_bytes();
        for i in 0..2 * n - 1 {
            let mut l = i as i64 / 2;
            let mut r = i / 2 + i % 2;
            while l >= 0 && r < n && s[l as usize] == s[r] {
                l -= 1;
                r += 1;
                ans += 1;
            }
        }
        return ans;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(Solution::count_substrings("abc".to_string()), 3);
    }
}
