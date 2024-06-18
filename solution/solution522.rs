/// 522.最长特殊序列 II
/// longest-uncommon-subsequence-ii
/// <p>给定字符串列表&nbsp;<code>strs</code> ，返回其中 <strong>最长的特殊序列</strong>&nbsp;的长度。如果最长特殊序列不存在，返回 <code>-1</code> 。</p>
///
/// <p><strong>特殊序列</strong> 定义如下：该序列为某字符串 <strong>独有的子序列（即不能是其他字符串的子序列）</strong>。</p>
///
/// <p>&nbsp;<code>s</code>&nbsp;的&nbsp;<strong>子序列</strong>可以通过删去字符串&nbsp;<code>s</code>&nbsp;中的某些字符实现。</p>
///
/// <ul>
/// 	<li>例如，<code>"abc"</code>&nbsp;是 <code>"aebdc"</code>&nbsp;的子序列，因为您可以删除<code>"a<u>e</u>b<u>d</u>c"</code>中的下划线字符来得到 <code>"abc"</code>&nbsp;。<code>"aebdc"</code>的子序列还包括<code>"aebdc"</code>、 <code>"aeb"</code>&nbsp;和 <font color="#c7254e" face="Menlo, Monaco, Consolas, Courier New, monospace"><span style="font-size: 12.6px; background-color: rgb(249, 242, 244);">""</span></font>&nbsp;(空字符串)。</li>
/// </ul>
///
/// <p>&nbsp;</p>
///
/// <p><strong>示例 1：</strong></p>
///
/// <pre>
/// <strong>输入:</strong> strs = ["aba","cdc","eae"]
/// <strong>输出:</strong> 3
/// </pre>
///
/// <p><strong>示例 2:</strong></p>
///
/// <pre>
/// <strong>输入:</strong> strs = ["aaa","aaa","aa"]
/// <strong>输出:</strong> -1
/// </pre>
///
/// <p>&nbsp;</p>
///
/// <p><strong>提示:</strong></p>
///
/// <ul>
/// 	<li><code>2 &lt;= strs.length &lt;= 50</code></li>
/// 	<li><code>1 &lt;= strs[i].length &lt;= 10</code></li>
/// 	<li><code>strs[i]</code>&nbsp;只包含小写英文字母</li>
/// </ul>
/// <a href="https://leetcode.cn/problems/longest-uncommon-subsequence-ii/">最长特殊序列 II</a>
pub struct Solution;

impl Solution {
    // 返回 s 是否为 t 的子序列
    fn is_subseq(s: &[u8], t: &str) -> bool {
        let mut i = 0;
        for c in t.bytes() {
            if s[i] == c {
                i += 1;
                if i == s.len() {
                    // 所有字符匹配完毕
                    return true; // s 是 t 的子序列
                }
            }
        }
        false
    }

    pub fn find_lu_slength(strs: Vec<String>) -> i32 {
        let mut ans = -1;
        'next: for (i, s) in strs.iter().enumerate() {
            if s.len() as i32 <= ans {
                // 不会让 ans 变大
                continue;
            }
            let s = s.as_bytes();
            for (j, t) in strs.iter().enumerate() {
                if j != i && Self::is_subseq(s, t) {
                    continue 'next;
                }
            }
            ans = s.len() as i32;
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(
            Solution::find_lu_slength(vec![
                "aba".to_string(),
                "cdc".to_string(),
                "eae".to_string()
            ]),
            3
        )
    }
}
