/// 1673.找出最具竞争力的子序列
///
/// <p>给你一个整数数组 <code>nums</code> 和一个正整数 <code>k</code> ，返回长度为 <code>k</code> 且最具 <strong>竞争力</strong> 的<em> </em><code>nums</code> 子序列。</p>
///
/// <p>数组的子序列是从数组中删除一些元素（可能不删除元素）得到的序列。</p>
///
/// <p>在子序列 <code>a</code> 和子序列 <code>b</code> 第一个不相同的位置上，如果 <code>a</code> 中的数字小于 <code>b</code> 中对应的数字，那么我们称子序列 <code>a</code> 比子序列 <code>b</code>（相同长度下）更具 <strong>竞争力</strong> 。 例如，<code>[1,3,4]</code> 比 <code>[1,3,5]</code> 更具竞争力，在第一个不相同的位置，也就是最后一个位置上， <code>4</code> 小于 <code>5</code> 。</p>
///
/// <p> </p>
///
/// <p><strong>示例 1：</strong></p>
///
/// <pre>
/// <strong>输入：</strong>nums = [3,5,2,6], k = 2
/// <strong>输出：</strong>[2,6]
/// <strong>解释：</strong>在所有可能的子序列集合 {[3,5], [3,2], [3,6], [5,2], [5,6], [2,6]} 中，[2,6] 最具竞争力。
/// </pre>
///
/// <p><strong>示例 2：</strong></p>
///
/// <pre>
/// <strong>输入：</strong>nums = [2,4,3,3,5,4,9,6], k = 4
/// <strong>输出：</strong>[2,3,3,4]
/// </pre>
///
/// <p> </p>
///
/// <p><strong>提示：</strong></p>
///
/// <ul>
/// 	<li><code>1 <= nums.length <= 10<sup>5</sup></code></li>
/// 	<li><code>0 <= nums[i] <= 10<sup>9</sup></code></li>
/// 	<li><code>1 <= k <= nums.length</code></li>
/// </ul>
/// <a href="https://leetcode.cn/problems/find-the-most-competitive-subsequence/">找出最具竞争力的子序列</a>
pub struct Solution;

/// 单调栈
impl Solution {
    pub fn most_competitive(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut result = vec![0; k];
        let mut curr_index = 0;

        for (index, &value) in nums.iter().enumerate() {
            let mut res_remain = k - curr_index;
            let nums_remain = nums.len() - index;
            while curr_index > 0 && result[curr_index - 1] > value && nums_remain > res_remain {
                curr_index -= 1;
                res_remain = k - curr_index;
            }
            if curr_index < k {
                result[curr_index] = value;
                curr_index += 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(
            Solution::most_competitive(vec![2, 4, 3, 3, 5, 4, 9, 6], 4),
            vec![2, 3, 3, 4]
        );
        assert_eq!(
            Solution::most_competitive(vec![71, 18, 52, 29, 55, 73, 24, 42, 66, 8, 80, 2], 3),
            vec![8, 80, 2]
        );
        assert_eq!(
            Solution::most_competitive(
                vec![
                    84, 10, 71, 23, 66, 61, 62, 64, 34, 41, 80, 25, 91, 43, 4, 75, 65, 13, 37, 41,
                    46, 90, 55, 8, 85, 61, 95, 71
                ],
                24
            ),
            vec![
                10, 23, 61, 62, 34, 41, 80, 25, 91, 43, 4, 75, 65, 13, 37, 41, 46, 90, 55, 8, 85,
                61, 95, 71
            ]
        )
    }
}
