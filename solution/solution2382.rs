/// 2382.删除操作后的最大子段和
/// maximum-segment-sum-after-removals
/// <p>给你两个下标从 <strong>0</strong>&nbsp;开始的整数数组&nbsp;<code>nums</code> 和&nbsp;<code>removeQueries</code>&nbsp;，两者长度都为&nbsp;<code>n</code>&nbsp;。对于第&nbsp;<code>i</code>&nbsp;个查询，<code>nums</code>&nbsp;中位于下标&nbsp;<code>removeQueries[i]</code>&nbsp;处的元素被删除，将 <code>nums</code>&nbsp;分割成更小的子段。</p>
///
/// <p>一个 <strong>子段</strong>&nbsp;是 <code>nums</code>&nbsp;中连续 <strong>正</strong>&nbsp;整数形成的序列。<strong>子段和</strong>&nbsp;是子段中所有元素的和。</p>
///
/// <p>请你返回一个长度为 <code>n</code>&nbsp;的整数数组<em>&nbsp;</em><code>answer</code>&nbsp;，其中<em>&nbsp;</em><code>answer[i]</code>是第&nbsp;<code>i</code>&nbsp;次删除操作以后的&nbsp;<strong>最大</strong>&nbsp;子段和。</p>
///
/// <p><strong>注意：</strong>一个下标至多只会被删除一次。</p>
///
/// <p>&nbsp;</p>
///
/// <p><strong>示例 1：</strong></p>
///
/// <pre><b>输入：</b>nums = [1,2,5,6,1], removeQueries = [0,3,2,4,1]
/// <b>输出：</b>[14,7,2,2,0]
/// <b>解释：</b>用 0 表示被删除的元素，答案如下所示：
/// 查询 1 ：删除第 0 个元素，nums 变成 [0,2,5,6,1] ，最大子段和为子段 [2,5,6,1] 的和 14 。
/// 查询 2 ：删除第 3 个元素，nums 变成 [0,2,5,0,1] ，最大子段和为子段 [2,5] 的和 7 。
/// 查询 3 ：删除第 2 个元素，nums 变成 [0,2,0,0,1] ，最大子段和为子段 [2] 的和 2 。
/// 查询 4 ：删除第 4 个元素，nums 变成 [0,2,0,0,0] ，最大子段和为子段 [2] 的和 2 。
/// 查询 5 ：删除第 1 个元素，nums 变成 [0,0,0,0,0] ，最大子段和为 0 ，因为没有任何子段存在。
/// 所以，我们返回 [14,7,2,2,0] 。</pre>
///
/// <p><strong>示例 2：</strong></p>
///
/// <pre><b>输入：</b>nums = [3,2,11,1], removeQueries = [3,2,1,0]
/// <b>输出：</b>[16,5,3,0]
/// <b>解释：</b>用 0 表示被删除的元素，答案如下所示：
/// 查询 1 ：删除第 3 个元素，nums 变成 [3,2,11,0] ，最大子段和为子段 [3,2,11] 的和 16 。
/// 查询 2 ：删除第 2 个元素，nums 变成 [3,2,0,0] ，最大子段和为子段 [3,2] 的和 5 。
/// 查询 3 ：删除第 1 个元素，nums 变成 [3,0,0,0] ，最大子段和为子段 [3] 的和 3 。
/// 查询 5 ：删除第 0 个元素，nums 变成 [0,0,0,0] ，最大子段和为 0 ，因为没有任何子段存在。
/// 所以，我们返回 [16,5,3,0] 。
/// </pre>
///
/// <p>&nbsp;</p>
///
/// <p><strong>提示：</strong></p>
///
/// <ul>
/// 	<li><code>n == nums.length == removeQueries.length</code></li>
/// 	<li><code>1 &lt;= n &lt;= 10<sup>5</sup></code></li>
/// 	<li><code>1 &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
/// 	<li><code>0 &lt;= removeQueries[i] &lt; n</code></li>
/// 	<li><code>removeQueries</code>&nbsp;中所有数字 <strong>互不相同</strong>&nbsp;。</li>
/// </ul>
/// <a href="https://leetcode.cn/problems/maximum-segment-sum-after-removals/">删除操作后的最大子段和</a>
pub struct Solution;

#[derive(Debug)]
struct UnionFind {
    fa: Vec<usize>,
}

impl UnionFind {
    // 创建一个新的并查集，有 n 个元素 (0 到 n-1)
    fn new(n: usize) -> Self {
        Self {
            fa: (0..n).collect(),
        }
    }

    // 查找 x 所在的集合，并进行路径压缩
    fn find(&mut self, mut x: usize) -> usize {
        while x != self.fa[x] {
            x = self.fa[x];
        }
        self.fa[x]
    }

    // 将 x 和 y 所在的集合合并。
    fn union(&mut self, x: usize, y: usize) {
        self.fa[x] = self.find(y);
    }
}

// 并查集
impl Solution {
    pub fn maximum_segment_sum(nums: Vec<i32>, remove_queries: Vec<i32>) -> Vec<i64> {
        let n = nums.len();
        let mut union_find = UnionFind::new(n + 1);
        let mut sum = vec![0usize; n + 1];

        let mut ans = vec![0i64; n];

        for i in (1..=n - 1).rev() {
            let index = remove_queries[i] as usize;
            let to = union_find.find(index + 1);
            // 就近合并
            union_find.union(index, index + 1);
            sum[to] += sum[index] + nums[index] as usize;
            ans[i - 1] = ans[i].max(sum[to] as i64);
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
            Solution::maximum_segment_sum(vec![1, 2, 5, 6, 1], vec![0, 3, 2, 4, 1]),
            vec![14, 7, 2, 2, 0]
        );
    }
}
