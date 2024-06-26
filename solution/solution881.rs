/// 881.救生艇
/// boats-to-save-people
/// <p>给定数组<meta charset="UTF-8" />&nbsp;<code>people</code>&nbsp;。<code>people[i]</code>表示第 <code>i</code><sup>&nbsp;</sup>个人的体重&nbsp;，<strong>船的数量不限</strong>，每艘船可以承载的最大重量为&nbsp;<code>limit</code>。</p>
///
/// <p>每艘船最多可同时载两人，但条件是这些人的重量之和最多为&nbsp;<code>limit</code>。</p>
///
/// <p>返回 <em>承载所有人所需的最小船数</em>&nbsp;。</p>
///
/// <p>&nbsp;</p>
///
/// <p><strong>示例 1：</strong></p>
///
/// <pre>
/// <strong>输入：</strong>people = [1,2], limit = 3
/// <strong>输出：</strong>1
/// <strong>解释：</strong>1 艘船载 (1, 2)
/// </pre>
///
/// <p><strong>示例 2：</strong></p>
///
/// <pre>
/// <strong>输入：</strong>people = [3,2,2,1], limit = 3
/// <strong>输出：</strong>3
/// <strong>解释：</strong>3 艘船分别载 (1, 2), (2) 和 (3)
/// </pre>
///
/// <p><strong>示例 3：</strong></p>
///
/// <pre>
/// <strong>输入：</strong>people = [3,5,3,4], limit = 5
/// <strong>输出：</strong>4
/// <strong>解释：</strong>4 艘船分别载 (3), (3), (4), (5)</pre>
///
/// <p>&nbsp;</p>
///
/// <p><strong>提示：</strong></p>
///
/// <ul>
/// 	<li><code>1 &lt;= people.length &lt;= 5 * 10<sup>4</sup></code></li>
/// 	<li><code>1 &lt;= people[i] &lt;= limit &lt;= 3 * 10<sup>4</sup></code></li>
/// </ul>
/// <a href="https://leetcode.cn/problems/boats-to-save-people/">救生艇</a>
pub struct Solution;

// 双指针
impl Solution {
    pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
        people.sort_unstable();
        let mut left = 0;
        let mut right = people.len() as i32 - 1;
        let mut ans = 0;

        while left <= right {
            if people[left as usize] + people[right as usize] <= limit {
                left += 1;
            }
            right -= 1;
            ans += 1;
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(Solution::num_rescue_boats(vec![3, 5, 3, 4], 5), 4)
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::num_rescue_boats(vec![5, 1, 4, 2], 6), 2)
    }
}
