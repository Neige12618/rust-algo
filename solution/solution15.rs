/// 15.三数之和
/// 3sum
/// <p>给你一个整数数组 <code>nums</code> ，判断是否存在三元组 <code>[nums[i], nums[j], nums[k]]</code> 满足 <code>i != j</code>、<code>i != k</code> 且 <code>j != k</code> ，同时还满足 <code>nums[i] + nums[j] + nums[k] == 0</code> 。请</p>
///
/// <p>你返回所有和为 <code>0</code> 且不重复的三元组。</p>
///
/// <p><strong>注意：</strong>答案中不可以包含重复的三元组。</p>
///
/// <p>&nbsp;</p>
///
/// <p>&nbsp;</p>
///
/// <p><strong>示例 1：</strong></p>
///
/// <pre>
/// <strong>输入：</strong>nums = [-1,0,1,2,-1,-4]
/// <strong>输出：</strong>[[-1,-1,2],[-1,0,1]]
/// <strong>解释：</strong>
/// nums[0] + nums[1] + nums[2] = (-1) + 0 + 1 = 0 。
/// nums[1] + nums[2] + nums[4] = 0 + 1 + (-1) = 0 。
/// nums[0] + nums[3] + nums[4] = (-1) + 2 + (-1) = 0 。
/// 不同的三元组是 [-1,0,1] 和 [-1,-1,2] 。
/// 注意，输出的顺序和三元组的顺序并不重要。
/// </pre>
///
/// <p><strong>示例 2：</strong></p>
///
/// <pre>
/// <strong>输入：</strong>nums = [0,1,1]
/// <strong>输出：</strong>[]
/// <strong>解释：</strong>唯一可能的三元组和不为 0 。
/// </pre>
///
/// <p><strong>示例 3：</strong></p>
///
/// <pre>
/// <strong>输入：</strong>nums = [0,0,0]
/// <strong>输出：</strong>[[0,0,0]]
/// <strong>解释：</strong>唯一可能的三元组和为 0 。
/// </pre>
///
/// <p>&nbsp;</p>
///
/// <p><strong>提示：</strong></p>
///
/// <ul>
/// 	<li><code>3 &lt;= nums.length &lt;= 3000</code></li>
/// 	<li><code>-10<sup>5</sup> &lt;= nums[i] &lt;= 10<sup>5</sup></code></li>
/// </ul>
/// <a href="https://leetcode.cn/problems/3sum/">三数之和</a>
pub struct Solution;

/// 双指针
impl Solution {
    fn two_sum(nums: &[i32], target: i32) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        let mut left = 0;
        let mut right = nums.len() - 1;
        while left < right {
            let sum = nums[left] + nums[right];
            if sum > target {
                right -= 1;
            } else if sum < target {
                left += 1;
            } else {
                // 若果找到了，则将结果加入答案中，并且同时移动两端指针
                ans.push(vec![nums[left], nums[right], -target]);
                left += 1;
                right -= 1;
                while left < right && nums[left] == nums[left - 1] {
                    left += 1;
                }
                while left < right && nums[right] == nums[right + 1] {
                    right -= 1;
                }
            }
        }
        ans
    }

    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();

        let mut ans = vec![];

        for (index, &value) in nums.iter().take(nums.len() - 2).enumerate() {
            if index > 0 && value == nums[index - 1] {
                continue;
            }
            if value + nums[index + 1] + nums[index + 2] > 0 {
                break;
            }
            if value + nums[nums.len() - 2] + nums[nums.len() - 1] < 0 {
                continue;
            }
            let target = -value;
            ans.extend(Self::two_sum(&nums[index + 1..], target));
        }

        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(Solution::three_sum(vec![0, 0, 0]), vec![vec![0, 0, 0]]);
        assert_eq!(
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
            vec![vec![-1, 2, -1], vec![0, 1, -1]]
        );
    }
}
