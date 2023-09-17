struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return String::new();
        }

        if strs.iter().any(|s| s.is_empty()) {
            return String::new();
        }

        let mut cnt = 0;
        let mut s = String::new();
        let strs = strs
            .into_iter()
            .map(|s| s.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let min = strs.iter().map(|s| s.len()).min().unwrap();

        loop {
            if cnt >= min {
                break;
            }

            if !strs.iter().map(|v| v[cnt]).all(|c| c == strs[0][cnt]) {
                break;
            }
            s.push(strs[0][cnt]);
            cnt += 1;
        }
        s
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn case1() {
        let strs = vec!["flower", "flow", "flight"];
        let expected = "fl";
        assert_eq!(
            expected,
            &super::Solution::longest_common_prefix(strs.iter().map(|s| s.to_string()).collect())
        );
    }

    #[test]
    fn case2() {
        let strs = vec!["dog", "racecar", "car"];
        let expected = "";
        assert_eq!(
            expected,
            &super::Solution::longest_common_prefix(strs.iter().map(|s| s.to_string()).collect())
        );
    }
}
