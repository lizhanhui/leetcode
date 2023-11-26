struct Solution;

impl Solution {
    fn next(dp: &Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
        if 0 == i || 0 == j {
            return 0;
        }
        assert!(dp[i - 1][j] >= 0);
        assert!(dp[i - 1][j - 1] >= 0);
        assert!(dp[i][j - 1] >= 0);
        return dp[i - 1][j].min(dp[i - 1][j - 1].min(dp[i][j - 1]));
    }

    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let mut dp = vec![];
        matrix.iter().for_each(|row| {
            let mut r = vec![];
            r.resize(row.len(), -1);
            dp.push(r);
        });

        let mut queue = std::collections::VecDeque::new();
        queue.push_back((0, 0));

        loop {
            if let Some((i, j)) = queue.pop_front() {
                if matrix[i][j] == '0' {
                    dp[i][j] = 0;
                } else {
                    dp[i][j] = 1 + Self::next(&dp, i, j);
                }

                if i + 1 < matrix.len() {
                    queue.push_back((i + 1, j));

                    if j + 1 < matrix[0].len() {
                        queue.push_back((i + 1, j + 1));
                    }
                }

                if j + 1 < matrix[0].len() {
                    queue.push_back((i, j + 1));
                }
            } else {
                break;
            }
        }

        dp.into_iter()
            .map(|r| r.into_iter().max().unwrap())
            .max()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn case1() {
        let matrix = vec![
            vec!['1', '0', '1', '0', '0'],
            vec!['1', '0', '1', '1', '1'],
            vec!['1', '1', '1', '1', '1'],
            vec!['1', '0', '0', '1', '0'],
        ];
        assert_eq!(4, super::Solution::maximal_square(matrix));
    }

    #[test]
    fn case2() {
        let matrix = vec![vec!['0', '1'], vec!['1', '0']];
        assert_eq!(1, super::Solution::maximal_square(matrix));
    }
}
