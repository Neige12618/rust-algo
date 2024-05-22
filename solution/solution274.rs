pub struct Solution;

impl Solution {
    pub fn h_index(mut citations: Vec<i32>) -> i32 {
        citations.sort_unstable();
        let n = citations.len();
        let mut ans = 0;
        for i in (0..n).rev() {
            if citations[i] > ans {
                ans += 1;
            } else {
                break;
            }
        }
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::h_index(vec![3, 0, 6, 1, 5]), 3);
    }
}
