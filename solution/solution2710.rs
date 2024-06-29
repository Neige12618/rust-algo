/// 2710.移除字符串中的尾随零
/// remove-trailing-zeros-from-a-string
/// <p>给你一个用字符串表示的正整数 <code>num</code> ，请你以字符串形式返回不含尾随零的整数<em> </em><code>num</code><em> </em>。</p>
///
/// <p>&nbsp;</p>
///
/// <p><strong>示例 1：</strong></p>
///
/// <pre><strong>输入：</strong>num = "51230100"
/// <strong>输出：</strong>"512301"
/// <strong>解释：</strong>整数 "51230100" 有 2 个尾随零，移除并返回整数 "512301" 。
/// </pre>
///
/// <p><strong>示例 2：</strong></p>
///
/// <pre><strong>输入：</strong>num = "123"
/// <strong>输出：</strong>"123"
/// <strong>解释：</strong>整数 "123" 不含尾随零，返回整数 "123" 。
/// </pre>
///
/// <p>&nbsp;</p>
///
/// <p><strong>提示：</strong></p>
///
/// <ul>
/// 	<li><code>1 &lt;= num.length &lt;= 1000</code></li>
/// 	<li><code>num</code> 仅由数字 <code>0</code> 到 <code>9</code> 组成</li>
/// 	<li><code>num</code> 不含前导零</li>
/// </ul>
/// <a href="https://leetcode.cn/problems/remove-trailing-zeros-from-a-string/">移除字符串中的尾随零</a>
pub struct Solution;

// 调库
impl Solution {
    pub fn remove_trailing_zeros(num: String) -> String {
        num.trim_end_matches('0').to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(
            Solution::remove_trailing_zeros("124790914000".to_string()),
            "124790914".to_string()
        )
    }
}
