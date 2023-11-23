struct Solution;

impl Solution {
    ///
    /// Time Complexity: O(MN)
    /// Space Complexity: O(MN)
    /// TODO: reduce space complexity to o(N) ?
    /// TODO: non-recursive implementation?
    pub fn min_distance(word1: String, word2: String) -> i32 {
        // d[i][j] = min-distance(&word1[..i], &word2[..j]);
        //
        // ans = d[m][n]
        // if word1[i] == word2[j] ==> d[i][j] = d[i-1][j-1]
        // if word1[i] != word2[j] ==> d[i][j] = min {
        //    d[i-1][j - 1] + 1, /* change a char */
        //    d[i][j-1] + 1, /* append a char to word1[..i] after prev modidifcation*/
        //    d[i-1][j] + 1, /* append a char to word2[..j] after previous modification  */
        // };
        //
        // d[i][0] = i
        // d[0][j] = j

        // (m + 1) * (n + 1) where m = word1.len() and n = word2.len()
        let mut dp = vec![];
        let m = word1.len();
        let n = word2.len();
        (0..=m).for_each(|_| {
            let mut row = vec![];
            row.resize(word2.len() + 1, -1);
            dp.push(row);
        });

        (0..=m).for_each(|i| {
            dp[i][0] = i as i32;
        });

        (0..=n).for_each(|j| {
            dp[0][j] = j as i32;
        });
        let lhs = word1.chars().collect::<Vec<_>>();
        let rhs = word2.chars().collect::<Vec<_>>();
        Self::dp(&mut dp, m, n, &lhs, &rhs)
    }

    fn dp(d: &mut Vec<Vec<i32>>, i: usize, j: usize, lhs: &[char], rhs: &[char]) -> i32 {
        if d[i][j] >= 0 {
            return d[i][j];
        }
        let d1 = Self::dp(d, i - 1, j - 1, lhs, rhs);
        let d2 = Self::dp(d, i - 1, j, lhs, rhs);
        let d3 = Self::dp(d, i, j - 1, lhs, rhs);
        let prev_min = d1.min(d2.min(d3));
        if lhs[i - 1] == rhs[j - 1] {
            d[i][j] = prev_min;
        } else {
            d[i][j] = 1 + prev_min;
        }
        return d[i][j];
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn case1() {
        let word1 = "horse";
        let word2 = "ros";

        assert_eq!(
            3,
            super::Solution::min_distance(word1.to_owned(), word2.to_owned())
        );
    }
}
