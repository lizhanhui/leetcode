struct Solution;

impl Solution {
    pub fn find_nth_digit(n: i32) -> i32 {
        // 1~9 --> one digit only: 9
        // 10 ~ 99 --> two digits: 89 * 2
        // 100 ~ 999 --> three digits: 899 * 3
        // 1000 ~ 9999 --> four digits: 8999 * 4
        let mut remain = n as usize;
        let mut round = 0;
        loop {
            round += 1;
            let level_digits = Self::num_count(round) * round;
            if remain > level_digits {
                remain -= level_digits;
                continue;
            }
            let m = remain / round;
            let remain = remain % round;

            let base = if 0 == round {
                0
            } else {
                10_usize.pow(round as u32 - 1) - 1
            };

            if remain == 0 {
                return Self::nth_digit(base + m, round - 1) as i32;
            }

            // the 'remain' digit of  (base + m + 1)
            return Self::nth_digit(base + m + 1, remain - 1) as i32;
        }
    }

    fn num_count(r: usize) -> usize {
        if 1 == r {
            return 9;
        }

        let mut n = r;
        let mut ans = 0;
        loop {
            if n == 0 {
                break;
            }
            ans = ans * 10 + 9;
            n -= 1;
        }

        ans - 10_usize.pow(r as u32 - 1)
    }

    fn nth_digit(mut num: usize, nth: usize) -> usize {
        let mut digits = std::collections::VecDeque::new();
        loop {
            let r = num % 10;
            num /= 10;
            digits.push_front(r);
            if 0 == num {
                break;
            }
        }
        digits.into_iter().nth(nth).unwrap()
    }

    fn slow(mut n: usize) {
        for i in 1.. {
            let c = Self::digit_cnt(i);
            if n <= c {
                println!("nth: {n}, num: {i}");
                break;
            } else {
                n -= c;
            }
        }
    }

    fn digit_cnt(mut n: usize) -> usize {
        let mut cnt = 0;
        loop {
            cnt += 1;
            n /= 10;
            if n == 0 {
                break;
            }
        }
        cnt
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_num_count() {
        assert_eq!(9, super::Solution::num_count(1));
        assert_eq!(89, super::Solution::num_count(2));
        assert_eq!(899, super::Solution::num_count(3));
    }

    #[test]
    fn case1() {
        assert_eq!(3, super::Solution::find_nth_digit(3));
        assert_eq!(0, super::Solution::find_nth_digit(11));
        assert_eq!(1, super::Solution::find_nth_digit(10));
    }

    #[test]
    fn wa1() {
        super::Solution::slow(1000);
        assert_eq!(3, super::Solution::find_nth_digit(1000));
    }
}
