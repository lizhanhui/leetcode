struct Solution;

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut result = vec![];
        let mut start = None;
        let mut current = None;
        for n in nums.iter() {
            if start.is_none() {
                start = Some(n);
                current = Some(n);
                continue;
            }

            if let Some(ref mut c) = current {
                if *c + 1 == *n {
                    *c = n;
                    continue;
                } else {
                    if let Some(ref mut s) = start {
                        if c == s {
                            result.push(format!("{}", *s));
                        } else {
                            result.push(format!("{}->{}", *s, *c));
                        }
                        // start with n
                        *s = n;
                        // current value is set n
                        *c = n;
                    }
                }
            }
        }
        // Handle the last one
        if let Some((s, c)) = start.zip(current) {
            if c == s {
                result.push(format!("{}", s));
            } else {
                result.push(format!("{}->{}", s, c));
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_summary_ranges_case1() {
        let nums = vec![0, 1, 2, 4, 5, 7];
        let result = vec!["0->2", "4->5", "7"];
        assert_eq!(super::Solution::summary_ranges(nums), result);
    }

    #[test]
    fn test_summary_ranges_case2() {
        let nums = vec![0, 2, 3, 4, 6, 8, 9];
        let result = vec!["0", "2->4", "6", "8->9"];
        assert_eq!(super::Solution::summary_ranges(nums), result);
    }
}
