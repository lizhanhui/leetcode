struct Solution;

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut row = vec![0; row_index as usize + 1];
        row[0] = 1;
        for i in 1..=(row_index as usize) {
            row[i] = 1;
            for j in (1..i).into_iter().rev() {
                row[j] += row[j - 1];
            }
            row[0] = 1;
        }
        row
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_row() {
        assert_eq!(Solution::get_row(3), vec![1, 3, 3, 1]);
        assert_eq!(Solution::get_row(0), vec![1]);
        assert_eq!(Solution::get_row(1), vec![1, 1]);
    }
}