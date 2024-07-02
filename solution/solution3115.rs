/// 3115.质数的最大距离
/// maximum-prime-difference
/// <p>给你一个整数数组 <code>nums</code>。</p>
///
/// <p>返回两个（不一定不同的）质数在 <code>nums</code> 中&nbsp;<strong>下标</strong> 的 <strong>最大距离</strong>。</p>
///
/// <p>&nbsp;</p>
///
/// <p><strong class="example">示例 1：</strong></p>
///
/// <div class="example-block">
/// <p><strong>输入：</strong> <span class="example-io">nums = [4,2,9,5,3]</span></p>
///
/// <p><strong>输出：</strong> <span class="example-io">3</span></p>
///
/// <p><strong>解释：</strong> <code>nums[1]</code>、<code>nums[3]</code> 和 <code>nums[4]</code> 是质数。因此答案是 <code>|4 - 1| = 3</code>。</p>
/// </div>
///
/// <p><strong class="example">示例 2：</strong></p>
///
/// <div class="example-block">
/// <p><strong>输入：</strong> <span class="example-io">nums = [4,8,2,8]</span></p>
///
/// <p><strong>输出：</strong> <span class="example-io">0</span></p>
///
/// <p><strong>解释：</strong> <code>nums[2]</code> 是质数。因为只有一个质数，所以答案是 <code>|2 - 2| = 0</code>。</p>
/// </div>
///
/// <p>&nbsp;</p>
///
/// <p><strong>提示：</strong></p>
///
/// <ul>
/// 	<li><code>1 &lt;= nums.length &lt;= 3 * 10<sup>5</sup></code></li>
/// 	<li><code>1 &lt;= nums[i] &lt;= 100</code></li>
/// 	<li>输入保证 <code>nums</code> 中至少有一个质数。</li>
/// </ul>
/// <a href="https://leetcode.cn/problems/maximum-prime-difference/">质数的最大距离</a>
pub struct Solution;

// 打表
impl Solution {
    pub fn maximum_prime_difference(nums: Vec<i32>) -> i32 {
        const PRIME_MASK: i128 = 0x20208828828208a20a08a28ac;
        let mut i = 0;
        while PRIME_MASK >> nums[i] & 1 == 0 {
            i += 1;
        }
        let mut j = nums.len() - 1;
        while PRIME_MASK >> nums[j] & 1 == 0 {
            j -= 1;
        }
        (j - i) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(Solution::maximum_prime_difference(vec![4, 2, 9, 5, 3]), 3)
    }
}
