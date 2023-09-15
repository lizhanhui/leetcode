struct Solution;

// The h-index is defined as the maximum value of h 
// such that the given researcher has published at least 
// h papers that have each been cited at least h times.
impl Solution {
    pub fn h_index(mut citations: Vec<i32>) -> i32 {
        citations.sort();
        let n = citations.len();
        let mut h = 0;
        for i in (0..n).rev() {
            let cnt = n - i;
            let citation = citations[i] as usize;
            let c = cnt.min(citation);
            if c > h {
                h = c;
            }
            if citation < h {
                break;
            }
        }

        h as i32
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn case1() {
        let citations = vec![3, 0, 6, 1, 5];
        let expected = 3;
        assert_eq!(super::Solution::h_index(citations), expected);
    }

    #[test]
    fn case2() {
        let citations = vec![1, 3, 1];
        let expected = 1;
        assert_eq!(super::Solution::h_index(citations), expected);
    }
}
