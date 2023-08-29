struct Solution;

impl Solution {
    pub fn best_closing_time(customers: String) -> i32 {
        let chars = customers.chars().collect::<Vec<_>>();
        let mut idx = 0;
        let mut penalty = i32::MAX;

        for i in 0..=chars.len() {
            let mut p = 0;
            for j in 0..chars.len() {
                if j < i && chars[j] == 'N' {
                    p += 1;
                    continue;
                }

                if j >= i && chars[j] == 'Y' {
                    p += 1;
                }
            }
            if p < penalty {
                idx = i;
                penalty = p;
            }
        }

        idx as i32
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn case1() {
        let customers = "NNNNN";
        assert_eq!(0, super::Solution::best_closing_time(customers.to_owned()));
    }

    #[test]
    fn case2() {
        let customers = "YYYY";
        assert_eq!(4, super::Solution::best_closing_time(customers.to_owned()));
    }
}
