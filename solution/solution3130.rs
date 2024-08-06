/// 3130.找出所有稳定的二进制数组 II
/// find-all-possible-stable-binary-arrays-ii
/// <p>给你 3 个正整数&nbsp;<code>zero</code>&nbsp;，<code>one</code>&nbsp;和&nbsp;<code>limit</code>&nbsp;。</p>
///
/// <p>一个 <span data-keyword="binary-array">二进制数组</span> <code>arr</code> 如果满足以下条件，那么我们称它是 <strong>稳定的</strong> ：</p>
///
/// <ul>
/// 	<li>0 在&nbsp;<code>arr</code>&nbsp;中出现次数 <strong>恰好</strong>&nbsp;为<strong>&nbsp;</strong><code>zero</code>&nbsp;。</li>
/// 	<li>1 在&nbsp;<code>arr</code>&nbsp;中出现次数 <strong>恰好</strong>&nbsp;为&nbsp;<code>one</code>&nbsp;。</li>
/// 	<li><code>arr</code> 中每个长度超过 <code>limit</code>&nbsp;的 <span data-keyword="subarray-nonempty">子数组</span> 都 <strong>同时</strong> 包含 0 和 1 。</li>
/// </ul>
///
/// <p>请你返回 <strong>稳定</strong>&nbsp;二进制数组的 <em>总</em> 数目。</p>
///
/// <p>由于答案可能很大，将它对&nbsp;<code>10<sup>9</sup> + 7</code>&nbsp;<b>取余</b>&nbsp;后返回。</p>
///
/// <p>&nbsp;</p>
///
/// <p><strong class="example">示例 1：</strong></p>
///
/// <div class="example-block">
/// <p><span class="example-io"><b>输入：</b>zero = 1, one = 1, limit = 2</span></p>
///
/// <p><span class="example-io"><b>输出：</b>2</span></p>
///
/// <p><strong>解释：</strong></p>
///
/// <p>两个稳定的二进制数组为&nbsp;<code>[1,0]</code> 和&nbsp;<code>[0,1]</code>&nbsp;，两个数组都有一个 0 和一个 1 ，且没有子数组长度大于 2 。</p>
/// </div>
///
/// <p><strong class="example">示例 2：</strong></p>
///
/// <div class="example-block">
/// <p><strong>输入：</strong><span class="example-io">zero = 1, one = 2, limit = 1</span></p>
///
/// <p><span class="example-io"><b>输出：</b>1</span></p>
///
/// <p><strong>解释：</strong></p>
///
/// <p>唯一稳定的二进制数组是&nbsp;<code>[1,0,1]</code>&nbsp;。</p>
///
/// <p>二进制数组&nbsp;<code>[1,1,0]</code> 和&nbsp;<code>[0,1,1]</code>&nbsp;都有长度为 2 且元素全都相同的子数组，所以它们不稳定。</p>
/// </div>
///
/// <p><strong class="example">示例 3：</strong></p>
///
/// <div class="example-block">
/// <p><span class="example-io"><b>输入：</b>zero = 3, one = 3, limit = 2</span></p>
///
/// <p><span class="example-io"><b>输出：</b>14</span></p>
///
/// <p><strong>解释：</strong></p>
///
/// <p>所有稳定的二进制数组包括&nbsp;<code>[0,0,1,0,1,1]</code>&nbsp;，<code>[0,0,1,1,0,1]</code>&nbsp;，<code>[0,1,0,0,1,1]</code>&nbsp;，<code>[0,1,0,1,0,1]</code>&nbsp;，<code>[0,1,0,1,1,0]</code>&nbsp;，<code>[0,1,1,0,0,1]</code>&nbsp;，<code>[0,1,1,0,1,0]</code>&nbsp;，<code>[1,0,0,1,0,1]</code>&nbsp;，<code>[1,0,0,1,1,0]</code>&nbsp;，<code>[1,0,1,0,0,1]</code>&nbsp;，<code>[1,0,1,0,1,0]</code>&nbsp;，<code>[1,0,1,1,0,0]</code>&nbsp;，<code>[1,1,0,0,1,0]</code>&nbsp;和&nbsp;<code>[1,1,0,1,0,0]</code>&nbsp;。</p>
/// </div>
///
/// <p>&nbsp;</p>
///
/// <p><strong>提示：</strong></p>
///
/// <ul>
/// 	<li><code>1 &lt;= zero, one, limit &lt;= 1000</code></li>
/// </ul>
/// <a href="https://leetcode.cn/problems/find-all-possible-stable-binary-arrays-ii/">找出所有稳定的二进制数组 II</a>
pub struct Solution;

use std::mem::swap;

const MOD: i32 = 1e9 as i32 + 7;

struct Comb<const N: usize = 1001> {
    f: [i64; N],
    inv_f: [i64; N],
}

fn pow(mut x: i64, mut n: i32) -> i64 {
    let mut res = 1;
    while n > 0 {
        if n % 2 > 0 {
            res = res * x % MOD as i64;
        }
        n >>= 1;
        x = x * x % MOD as i64;
    }
    res
}

impl<const N: usize> Comb<N> {
    fn new() -> Self {
        let mut c = Comb {
            f: [0; N],
            inv_f: [0; N],
        };
        c.f[0] = 1;
        for i in 1..N {
            c.f[i] = c.f[i - 1] * i as i64 % MOD as i64;
        }

        c.inv_f[N - 1] = pow(c.f[N - 1], MOD - 2);

        for i in (1..N).rev() {
            c.inv_f[i - 1] = c.inv_f[i] * i as i64 % MOD as i64;
        }

        c
    }

    fn comb(&self, n: usize, m: usize) -> i64 {
        self.f[n] * self.inv_f[m] % MOD as i64 * self.inv_f[n - m] % MOD as i64
    }
}

impl Solution {
    pub fn number_of_stable_arrays(zero: i32, one: i32, limit: i32) -> i32 {
        let c: Comb = Comb::new();
        let (mut zero, mut one, limit) = (zero as usize, one as usize, limit as usize);
        if zero > one {
            swap(&mut zero, &mut one);
        }

        let mut f0 = vec![0; zero + 3];
        for i in (zero - 1) / limit + 1..=zero {
            f0[i] = c.comb(zero - 1, i - 1);
            for j in 1..=(zero - i) / limit {
                f0[i] = (f0[i]
                    + (1 - j as i64 % 2 * 2) * c.comb(i, j) * c.comb(zero - j * limit - 1, i - 1))
                    % MOD as i64;
            }
        }

        let mut ans = 0;
        for i in (one - 1) / limit + 1..=one.min(zero + 1) {
            let mut f1 = c.comb(one - 1, i - 1);
            for j in 1..=(one - i) / limit {
                f1 = (f1
                    + (1 - j as i64 % 2 * 2) * c.comb(i, j) * c.comb(one - j * limit - 1, i - 1))
                    % MOD as i64;
            }
            ans = (ans + (f0[i - 1] + f0[i] * 2 + f0[i + 1]) * f1) % MOD as i64;
        }

        (ans as i32 + MOD) % MOD
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(Solution::number_of_stable_arrays(1, 1, 2), 2)
    }
}
