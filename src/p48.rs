struct Solution;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        // 1, 2, 3
        // 4, 5, 6
        // 7, 8, 9
        //
        //
        // 7, 4, 1
        // 8, 5, 2
        // 9, 6, 3
        Self::flip(matrix);
        Self::diag_rotate(matrix);
    }

    fn diag_rotate(matrix: &mut Vec<Vec<i32>>) {
        let row = matrix.len();
        let column = matrix[0].len();
        for i in 0..row {
            for j in 0..i {
                let t = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = t;
            }
        }
    }

    fn flip(matrix: &mut Vec<Vec<i32>>) {
        let row = matrix.len();
        for i in 0..row / 2 {
            for j in 0..matrix[i].len() {
                let t = matrix[i][j];
                matrix[i][j] = matrix[row - i - 1][j];
                matrix[row - 1 - i][j] = t;
            }
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_flip() {
        let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let expected = vec![vec![7, 8, 9], vec![4, 5, 6], vec![1, 2, 3]];
        super::Solution::flip(&mut matrix);
        assert_eq!(expected, matrix);
    }

    #[test]
    fn case1() {
        let mut matrix = vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]];
        let expected = vec![vec![7,4,1], vec![8,5,2], vec![9,6,3]];
        super::Solution::rotate(&mut matrix);
        assert_eq!(expected, matrix);
    }
}
