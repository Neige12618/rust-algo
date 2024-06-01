/// 2929.给小朋友们分糖果 II
/// distribute-candies-among-children-ii
/// <p>给你两个正整数&nbsp;<code>n</code> 和&nbsp;<code>limit</code>&nbsp;。</p>
///
/// <p>请你将 <code>n</code>&nbsp;颗糖果分给 <code>3</code>&nbsp;位小朋友，确保没有任何小朋友得到超过 <code>limit</code>&nbsp;颗糖果，请你返回满足此条件下的&nbsp;<strong>总方案数</strong>&nbsp;。</p>
///
/// <p>&nbsp;</p>
///
/// <p><strong class="example">示例 1：</strong></p>
///
/// <pre>
/// <b>输入：</b>n = 5, limit = 2
/// <b>输出：</b>3
/// <b>解释：</b>总共有 3 种方法分配 5 颗糖果，且每位小朋友的糖果数不超过 2 ：(1, 2, 2) ，(2, 1, 2) 和 (2, 2, 1) 。
/// </pre>
///
/// <p><strong class="example">示例 2：</strong></p>
///
/// <pre>
/// <b>输入：</b>n = 3, limit = 3
/// <b>输出：</b>10
/// <b>解释：</b>总共有 10 种方法分配 3 颗糖果，且每位小朋友的糖果数不超过 3 ：(0, 0, 3) ，(0, 1, 2) ，(0, 2, 1) ，(0, 3, 0) ，(1, 0, 2) ，(1, 1, 1) ，(1, 2, 0) ，(2, 0, 1) ，(2, 1, 0) 和 (3, 0, 0) 。
/// </pre>
///
/// <p>&nbsp;</p>
///
/// <p><strong>提示：</strong></p>
///
/// <ul>
/// 	<li><code>1 &lt;= n &lt;= 10<sup>6</sup></code></li>
/// 	<li><code>1 &lt;= limit &lt;= 10<sup>6</sup></code></li>
/// </ul>
/// <a href="https://leetcode.cn/problems/distribute-candies-among-children-ii/">给小朋友们分糖果 II</a>
pub struct Solution;

// 同2928，数据范围扩大
impl Solution {
    pub fn distribute_candies(n: i32, limit: i32) -> i64 {
        let mut ans = 0i64;
        for i in (0..=n.min(limit)).rev() {
            let remain = n - i;
            if remain > 2 * limit {
                break;
            }
            // 如果剩下的还可以被分配
            // 至少 max(0, remain−limit)，至多 min(limit, remain)
            // 所以使用至多的情况去除至少的情况，即是在中间的情况。
            ans += (limit.min(remain) - 0.max(remain - limit)) as i64 + 1;
        }
        ans
    }

    pub fn distribute_candies_math(n: i32, limit: i32) -> i64 {
        let c_2 = |n| {
            let n = n as i64;
            if n <= 0 {
                0
            } else {
                n * (n - 1) / 2
            }
        };
        // 无限制任意选中去除有一个超限，有两个超限，三个都超限的情况
        c_2(n + 2) - 3 * c_2(n - limit + 1) + 3 * c_2(n - 2 * limit) - c_2(n - 3 * limit - 1)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(Solution::distribute_candies(5, 2), 3);
    }
}
