struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        // Naive
        // KMP
        let haystack = haystack.chars().collect::<Vec<_>>();
        let needle = needle.chars().collect::<Vec<_>>();

        let mut i = 0;
        loop {
            if i >= haystack.len() {
                break;
            }
            let mut j = 0;
            loop {
                if j >= needle.len() {
                    return i as i32;
                }
                if i + j >= haystack.len() {
                    return -1;
                }
                if needle[j] != haystack[i + j] {
                    break;
                }
                j += 1;
            }
            i += 1;
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn case1() {
        let haystack = "sadbutsad";
        let needle = "sad";
        let expected = 0;
        assert_eq!(
            expected,
            super::Solution::str_str(haystack.to_string(), needle.to_string())
        );
    }

    #[test]
    fn case2() {
        let haystack = "leetcode";
        let needle = "leeto";
        let expected = -1;
        assert_eq!(
            expected,
            super::Solution::str_str(haystack.to_string(), needle.to_string())
        );
    }
}
