struct Solution {}

impl Solution {
    // Observe diagnoal elements has the same row + column indexes
    // Time Complexity: O(n)
    // Space Complexity: O(n)
    pub fn find_diagonal_order(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut m = std::collections::BTreeMap::new();
        nums.iter().enumerate().rev().for_each(|(i, row)| {
            row.iter().enumerate().for_each(|(j, v)| {
                m.entry(i + j)
                    .and_modify(|entry: &mut Vec<i32>| entry.push(*v))
                    .or_insert(vec![*v]);
            })
        });
        m.into_iter()
            .map(|(_k, v)| v.into_iter())
            .flatten()
            .collect()
    }

    // Secondary diagonal access pattern is actually broad-first-search
    // Time complexity: O(n)
    // Space complexity: C
    fn bfs(nums: &Vec<Vec<i32>>) -> Vec<i32> {
        let mut buffer = std::collections::VecDeque::new();
        let mut ans = vec![];
        buffer.push_back((0, 0));
        let mut visisted = vec![];
        nums.iter().for_each(|r| {
            let mut row = vec![];
            row.resize(r.len(), false);
            visisted.push(row);
        });
        visisted[0][0] = true;
        loop {
            if let Some((i, j)) = buffer.pop_front() {
                ans.push(nums[i][j]);

                // At most, there are three neighbor elements in the next layer
                // i + 1, j
                // i, j + 1
                // i - 1, j + 1
                if i + 1 < nums.len() && nums[i + 1].len() > j && !visisted[i + 1][j] {
                    visisted[i + 1][j] = true;
                    buffer.push_back((i + 1, j));
                }

                if nums[i].len() > j + 1 && !visisted[i][j + 1] {
                    visisted[i][j + 1] = true;
                    buffer.push_back((i, j + 1));
                }

                if i >= 1 && nums[i - 1].len() > j + 1 && !visisted[i - 1][j + 1] {
                    visisted[i - 1][j + 1] = true;
                    buffer.push_back((i - 1, j + 1));
                }
            } else {
                break;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn case1() {
        let nums = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(
            super::Solution::find_diagonal_order(nums.clone()),
            vec![1, 4, 2, 7, 5, 3, 8, 6, 9]
        );

        assert_eq!(super::Solution::bfs(&nums), vec![1, 4, 2, 7, 5, 3, 8, 6, 9]);
    }
}
