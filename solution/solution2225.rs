pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = vec![vec![], vec![]];
        let mut lose_map = HashMap::new();

        for i in 0..matches.len() {
            let (a, b) = (matches[i][0], matches[i][1]);
            lose_map.entry(a).or_insert(0);
            *lose_map.entry(b).or_insert(0) += 1;
        }

        for (player, lose_count) in lose_map {
            if lose_count < 2 {
                result[lose_count as usize].push(player);
            }
        }

        result[0].sort_unstable();
        result[1].sort_unstable();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::find_winners(vec![
                vec![1, 3],
                vec![2, 3],
                vec![3, 6],
                vec![5, 6],
                vec![5, 7],
                vec![4, 5],
                vec![4, 8],
                vec![4, 9],
                vec![10, 4],
                vec![10, 9]
            ]),
            vec![vec![1, 2, 10], vec![4, 5, 7, 8]]
        );
    }
}
