/// 3040.相同分数的最大操作数目 II
/// maximum-number-of-operations-with-the-same-score-ii
/// <p>给你一个整数数组&nbsp;<code>nums</code>&nbsp;，如果&nbsp;<code>nums</code>&nbsp;<strong>至少</strong>&nbsp;包含 <code>2</code>&nbsp;个元素，你可以执行以下操作中的&nbsp;<strong>任意</strong>&nbsp;一个：</p>
///
/// <ul>
/// 	<li>选择 <code>nums</code>&nbsp;中最前面两个元素并且删除它们。</li>
/// 	<li>选择 <code>nums</code>&nbsp;中最后两个元素并且删除它们。</li>
/// 	<li>选择 <code>nums</code>&nbsp;中第一个和最后一个元素并且删除它们。</li>
/// </ul>
///
/// <p>一次操作的&nbsp;<strong>分数</strong>&nbsp;是被删除元素的和。</p>
///
/// <p>在确保<strong>&nbsp;所有操作分数相同</strong>&nbsp;的前提下，请你求出&nbsp;<strong>最多</strong>&nbsp;能进行多少次操作。</p>
///
/// <p>请你返回按照上述要求&nbsp;<strong>最多</strong>&nbsp;可以进行的操作次数。</p>
///
/// <p>&nbsp;</p>
///
/// <p><strong class="example">示例 1：</strong></p>
///
/// <pre>
/// <b>输入：</b>nums = [3,2,1,2,3,4]
/// <b>输出：</b>3
/// <b>解释：</b>我们执行以下操作：
/// - 删除前两个元素，分数为 3 + 2 = 5 ，nums = [1,2,3,4] 。
/// - 删除第一个元素和最后一个元素，分数为 1 + 4 = 5 ，nums = [2,3] 。
/// - 删除第一个元素和最后一个元素，分数为 2 + 3 = 5 ，nums = [] 。
/// 由于 nums 为空，我们无法继续进行任何操作。
/// </pre>
///
/// <p><strong class="example">示例 2：</strong></p>
///
/// <pre>
/// <b>输入：</b>nums = [3,2,6,1,4]
/// <b>输出：</b>2
/// <b>解释：</b>我们执行以下操作：
/// - 删除前两个元素，分数为 3 + 2 = 5 ，nums = [6,1,4] 。
/// - 删除最后两个元素，分数为 1 + 4 = 5 ，nums = [6] 。
/// 至多进行 2 次操作。
/// </pre>
///
/// <p>&nbsp;</p>
///
/// <p><strong>提示：</strong></p>
///
/// <ul>
/// 	<li><code>2 &lt;= nums.length &lt;= 2000</code></li>
/// 	<li><code>1 &lt;= nums[i] &lt;= 1000</code></li>
/// </ul>
/// <a href="https://leetcode.cn/problems/maximum-number-of-operations-with-the-same-score-ii/">相同分数的最大操作数目 II</a>
pub struct Solution;

// 记忆化搜索
impl Solution {
    pub fn max_operations(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n < 2 {
            return 0;
        }

        let mut s = vec![vec![0; n]; n];
        let count1 = Self::dfs(2, n - 1, nums[0] + nums[1], &nums, &mut s);
        s.iter_mut().for_each(|v| v.fill(0));
        let count2 = Self::dfs(0, n.max(3) - 3, nums[n - 1] + nums[n - 2], &nums, &mut s);
        s.iter_mut().for_each(|v| v.fill(0));
        let count3 = Self::dfs(1, n - 2, nums[0] + nums[n - 1], &nums, &mut s);

        count1.max(count2.max(count3)) + 1
    }

    fn dfs(
        left: usize,
        right: usize,
        target: i32,
        nums: &Vec<i32>,
        memo: &mut Vec<Vec<i32>>,
    ) -> i32 {
        if left >= right {
            return 0;
        }
        if memo[left][right] != 0 {
            return memo[left][right];
        }
        let mut res = 0;
        if nums[left] + nums[left + 1] == target {
            res = res.max(Self::dfs(left + 2, right, target, nums, memo) + 1);
        }
        if nums[left] + nums[right] == target {
            res = res.max(Self::dfs(left + 1, right - 1, target, nums, memo) + 1);
        }
        if nums[right - 1] + nums[right] == target {
            res = res.max(Self::dfs(left, right.max(2) - 2, target, nums, memo) + 1);
        }
        memo[left][right] = res;
        res
    }

    pub fn max_operations_no_recursion(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let no_recursion = |left: usize, right: usize, target: i32| {
            let mut dp = vec![vec![0; n + 1]; n + 1];
            for i in (left..right).rev() {
                for j in i + 1..=right {
                    if nums[i] + nums[i + 1] == target {
                        // 删除前两个数
                        dp[i][j + 1] = dp[i][j + 1].max(dp[i + 2][j + 1] + 1);
                    }
                    if nums[j - 1] + nums[j] == target {
                        // 删除后两个数
                        dp[i][j + 1] = dp[i][j + 1].max(dp[i][j - 1] + 1);
                    }
                    if nums[i] + nums[j] == target {
                        // 删除第一个和最后一个数
                        dp[i][j + 1] = dp[i][j + 1].max(dp[i + 1][j] + 1);
                    }
                }
            }
            return dp[left][right + 1];
        };
        let res1 = no_recursion(2, n - 1, nums[0] + nums[1]); // 删除前两个数
        let res2 = no_recursion(0, n.max(3) - 3, nums[n - 2] + nums[n - 1]); // 删除后两个数
        let res3 = no_recursion(1, n - 2, nums[0] + nums[n - 1]); // 删除第一个和最后一个数
        return res1.max(res2.max(res3)) + 1; // 加上第一次操作
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(Solution::max_operations(vec![3, 2, 1, 2, 3, 4]), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::max_operations(vec![3, 2]), 1);
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::max_operations(vec![
                298, 258, 242, 18, 538, 363, 193, 535, 21, 357, 473, 470, 227, 465, 91, 374, 182,
                457, 99, 414, 448, 422, 134, 218, 338, 473, 83, 23, 533, 150, 304, 252, 479, 77,
                273, 283, 196, 525, 31, 99, 457, 398, 158, 357, 199, 256, 300, 402, 154, 326, 230,
                15, 364, 313, 458, 500, 56, 522, 34, 98, 492, 490, 66, 392, 431, 479, 77, 88, 263,
                293, 354, 202, 338, 456, 100, 254, 490, 107, 449, 309, 247, 896, 976, 320, 471,
                436, 411, 267, 614, 576, 233, 669, 243, 444, 112, 533, 23, 496, 60, 66, 302, 420,
                136, 524, 32, 251, 305, 215, 341, 280, 276, 218, 468, 429, 127, 125, 164, 64, 464,
                92, 458, 98, 243, 192, 541, 86, 470, 450, 106, 132, 424, 541, 15, 41, 515, 494, 62,
                538, 18, 132, 424, 109, 447, 406, 150, 360, 484, 72, 83, 473, 541, 15, 297, 259,
                232, 324, 406, 395, 161, 453, 103, 280, 276, 55, 501, 108, 142, 374, 182, 295, 261,
                329, 377, 179, 263, 293, 86, 83, 219, 337, 42, 514, 199, 347, 209, 314, 38, 518
            ]),
            88
        );
    }
}
