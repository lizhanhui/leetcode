struct Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut s_itr = s.chars();
        let mut t_itr = t.chars();
        let mut sc = s_itr.next();
        let mut tc = t_itr.next();
        loop {
            if sc.is_none() {
                break;
            }

            if tc.is_none() {
                return false;
            }

            if sc == tc {
                sc = s_itr.next();
                tc = t_itr.next();
            } else {
                tc = t_itr.next();
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn case1() {
        let s = "abc";
        let t = "ahbgdc";
        assert_eq!(
            true,
            super::Solution::is_subsequence(s.to_owned(), t.to_owned())
        );
    }

    #[test]
    fn case2() {
        let s = "axc";
        let t = "ahbgdc";
        assert_eq!(
            false,
            super::Solution::is_subsequence(s.to_owned(), t.to_owned())
        );
    }
}
