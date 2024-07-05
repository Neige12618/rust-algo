/// 3086.拾起 K 个 1 需要的最少行动次数
/// minimum-moves-to-pick-k-ones
/// <p>给你一个下标从 <strong>0</strong> 开始的二进制数组 <code>nums</code>，其长度为 <code>n</code> ；另给你一个 <strong>正整数 </strong><code>k</code> 以及一个 <strong>非负整数 </strong><code>maxChanges</code> 。</p>
///
/// <p>Alice 在玩一个游戏，游戏的目标是让&nbsp;Alice 使用 <strong>最少 </strong>数量的 <strong>行动 </strong>次数从 <code>nums</code> 中拾起 <code>k</code> 个 1 。游戏开始时，Alice 可以选择数组 <code>[0, n - 1]</code> 范围内的任何索引&nbsp;<code>aliceIndex</code> 站立。如果 <code>nums[aliceIndex] == 1</code> ，Alice 会拾起一个 1 ，并且 <code>nums[aliceIndex]</code> 变成<code>0</code>（这<strong> 不算 </strong>作一次行动）。之后，Alice 可以执行 <strong>任意数量</strong> 的 <strong>行动</strong>（<strong>包括</strong><strong>零次</strong>），在每次行动中&nbsp;Alice 必须 <strong>恰好 </strong>执行以下动作之一：</p>
///
/// <ul>
/// 	<li>选择任意一个下标 <code>j != aliceIndex</code> 且满足 <code>nums[j] == 0</code> ，然后将 <code>nums[j]</code> 设置为 <code>1</code> 。这个动作最多可以执行 <code>maxChanges</code> 次。</li>
/// 	<li>选择任意两个相邻的下标 <code>x</code> 和 <code>y</code>（<code>|x - y| == 1</code>）且满足 <code>nums[x] == 1</code>, <code>nums[y] == 0</code> ，然后交换它们的值（将 <code>nums[y] = 1</code> 和 <code>nums[x] = 0</code>）。如果 <code>y == aliceIndex</code>，在这次行动后&nbsp;Alice 拾起一个 1 ，并且 <code>nums[y]</code> 变成 <code>0</code> 。</li>
/// </ul>
///
/// <p>返回&nbsp;Alice 拾起 <strong>恰好 </strong><code>k</code> 个 1 所需的 <strong>最少 </strong>行动次数。</p>
///
/// <p>&nbsp;</p>
///
/// <p><strong class="example">示例 1：</strong></p>
///
/// <div class="example-block" style="border-color: var(--border-tertiary); border-left-width: 2px; color: var(--text-secondary); font-size: .875rem; margin-bottom: 1rem; margin-top: 1rem; overflow: visible; padding-left: 1rem;">
/// <p><strong>输入：</strong><span class="example-io" style="font-family: Menlo,sans-serif; font-size: 0.85rem;">nums = [1,1,0,0,0,1,1,0,0,1], k = 3, maxChanges = 1</span></p>
///
/// <p><strong>输出：</strong><span class="example-io" style="font-family: Menlo,sans-serif; font-size: 0.85rem;">3</span></p>
///
/// <p><strong>解释：</strong>如果游戏开始时&nbsp;Alice 在 <code>aliceIndex == 1</code> 的位置上，按照以下步骤执行每个动作，他可以利用 <code>3</code> 次行动拾取 <code>3</code> 个 1 ：</p>
///
/// <ul>
/// 	<li>游戏开始时&nbsp;Alice 拾取了一个 1 ，<code>nums[1]</code> 变成了 <code>0</code>。此时 <code>nums</code> 变为 <code>[1,<strong><u>1</u></strong>,1,0,0,1,1,0,0,1]</code> 。</li>
/// 	<li>选择 <code>j == 2</code> 并执行第一种类型的动作。<code>nums</code> 变为 <code>[1,<strong><u>0</u></strong>,1,0,0,1,1,0,0,1]</code></li>
/// 	<li>选择 <code>x == 2</code> 和 <code>y == 1</code> ，并执行第二种类型的动作。<code>nums</code> 变为 <code>[1,<strong><u>1</u></strong>,0,0,0,1,1,0,0,1]</code> 。由于 <code>y == aliceIndex</code>，Alice 拾取了一个 1 ，<code>nums</code> 变为&nbsp; <code>[1,<strong><u>0</u></strong>,0,0,0,1,1,0,0,1]</code> 。</li>
/// 	<li>选择 <code>x == 0</code> 和 <code>y == 1</code> ，并执行第二种类型的动作。<code>nums</code> 变为 <code>[0,<strong><u>1</u></strong>,0,0,0,1,1,0,0,1]</code> 。由于 <code>y == aliceIndex</code>，Alice 拾取了一个 1 ，<code>nums</code> 变为&nbsp; <code>[0,<strong><u>0</u></strong>,0,0,0,1,1,0,0,1]</code> 。</li>
/// </ul>
///
/// <p>请注意，Alice 也可能执行其他的 <code>3</code> 次行动序列达成拾取 <code>3</code> 个 1 。</p>
/// </div>
///
/// <p><strong class="example">示例 2：</strong></p>
///
/// <div class="example-block" style="border-color: var(--border-tertiary); border-left-width: 2px; color: var(--text-secondary); font-size: .875rem; margin-bottom: 1rem; margin-top: 1rem; overflow: visible; padding-left: 1rem;"><!-- 以下是示例内容的中文翻译，同时保留了原有的HTML格式和注释 -->
/// <p><strong>输入：</strong><span class="example-io" style="font-family: Menlo,sans-serif; font-size: 0.85rem;">nums = [0,0,0,0], k = 2, maxChanges = 3</span></p>
///
/// <p><strong>输出：</strong><span class="example-io" style="font-family: Menlo,sans-serif; font-size: 0.85rem;">4</span></p>
///
/// <p><strong>解释：</strong>如果游戏开始时&nbsp;Alice 在 <code>aliceIndex == 0</code> 的位置上，按照以下步骤执行每个动作，他可以利用 <code>4</code> 次行动拾取 <code>2</code> 个 1 ：</p>
///
/// <ul>
/// 	<li>选择 <code>j == 1</code> 并执行第一种类型的动作。<code>nums</code> 变为 <code>[<strong><u>0</u></strong>,1,0,0]</code> 。</li>
/// 	<li>选择 <code>x == 1</code> 和 <code>y == 0</code> ，并执行第二种类型的动作。<code>nums</code> 变为 <code>[<strong><u>1</u></strong>,0,0,0]</code> 。由于 <code>y == aliceIndex</code>，Alice 拾起了一个 1 ，<code>nums</code> 变为 <code>[<strong><u>0</u></strong>,0,0,0]</code> 。</li>
/// 	<li>再次选择 <code>j == 1</code> 并执行第一种类型的动作。<code>nums</code> 变为 <code>[<strong><u>0</u></strong>,1,0,0]</code> 。</li>
/// 	<li>再次选择 <code>x == 1</code> 和 <code>y == 0</code> ，并执行第二种类型的动作。<code>nums</code> 变为 <code>[<strong><u>1</u></strong>,0,0,0]</code> 。由于<code>y == aliceIndex</code>，Alice 拾起了一个 1 ，<code>nums</code> 变为 <code>[<strong><u>0</u></strong>,0,0,0]</code> 。</li>
/// </ul>
/// </div>
///
/// <p>&nbsp;</p>
///
/// <p><strong>提示：</strong></p>
///
/// <ul>
/// 	<li><code>2 &lt;= n &lt;= 10<sup>5</sup></code></li>
/// 	<li><code>0 &lt;= nums[i] &lt;= 1</code></li>
/// 	<li><code>1 &lt;= k &lt;= 10<sup>5</sup></code></li>
/// 	<li><code>0 &lt;= maxChanges &lt;= 10<sup>5</sup></code></li>
/// 	<li><code>maxChanges + sum(nums) &gt;= k</code></li>
/// </ul>
/// <a href="https://leetcode.cn/problems/minimum-moves-to-pick-k-ones/">拾起 K 个 1 需要的最少行动次数</a>
pub struct Solution;

// 贪心
impl Solution {
    pub fn minimum_moves(nums: Vec<i32>, k: i32, max_changes: i32) -> i64 {
        let k = k as i64;
        let max_changes = max_changes as i64;
        let mut pos = vec![];
        let mut c = 0i64;
        for (i, &x) in nums.iter().enumerate() {
            if x == 0 {
                continue;
            }
            pos.push(i);
            c = c.max(1);
            if i > 0 && nums[i - 1] == 1 {
                if i > 1 && nums[i - 2] == 1 {
                    c = 3;
                } else {
                    c = c.max(2);
                }
            }
        }

        c = c.min(k);
        if max_changes >= k - c {
            return 0.max(c - 1) + (k - c) * 2;
        }

        let n = pos.len();
        let mut pre_sum = vec![0; n + 1];
        for i in 0..n {
            pre_sum[i + 1] = pre_sum[i] + pos[i] as i64;
        }

        let mut ans = i64::MAX;
        let size = (k - max_changes) as usize;
        for right in size..n + 1 {
            let left = right - size;
            let i = left + size / 2;
            let s1 = (pos[i] * (i - left)) as i64 - (pre_sum[i] - pre_sum[left]);
            let s2 = pre_sum[right] - pre_sum[i] - (pos[i] * (right - i)) as i64;
            ans = ans.min(s1 + s2);
        }

        return ans + max_changes * 2;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(
            Solution::minimum_moves(vec![1, 1, 0, 0, 0, 1, 1, 0, 0, 1], 3, 1),
            3
        )
    }
}
