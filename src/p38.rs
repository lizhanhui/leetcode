struct Solution;

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        if 1 == n {
            let v = vec![1];
            Solution::to_string(&v)
        } else {
            let mut c = vec![1];
            let mut v = vec![];
            for _ in 2..=(n as usize) {
                Solution::next(&c, &mut v);
                std::mem::swap(&mut c, &mut v);
            }
            Solution::to_string(&c)
        }
    }

    fn to_string(v: &Vec<i8>) -> String {
        let mut s = String::new();
        for n in v.iter() {
            s.push_str(&format!("{}", *n));
        }
        s
    }

    fn append(mut cnt: usize, c: i8, n: &mut Vec<i8>) {
        if 0 == cnt {
            return;
        }

        // push back digits of cnt
        let mut digits = vec![];
        loop {
            digits.push((cnt % 10) as i8);
            cnt = cnt / 10;
            if cnt == 0 {
                break;
            }
        }
        digits.reverse();
        for digit in digits {
            n.push(digit);
        }
        n.push(c);
    }

    fn next(v: &Vec<i8>, n: &mut Vec<i8>) {
        n.clear();
        let mut cnt = 0;
        let mut c = 0;
        for m in v.iter() {
            if c != *m {
                if cnt > 0 {
                    Solution::append(cnt, c, n);
                }
                c = *m;
                cnt = 1;
            } else {
                cnt += 1;
            }
        }
        Self::append(cnt, c, n);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_count_and_say_case1() {
        assert_eq!(&super::Solution::count_and_say(1), "1");
    }

    #[test]
    fn test_count_and_say_case2() {
        assert_eq!(&super::Solution::count_and_say(2), "11");
    }

    #[test]
    fn test_count_and_say_case3() {
        assert_eq!(&super::Solution::count_and_say(3), "21");
    }

    #[test]
    fn test_count_and_say_case4() {
        assert_eq!(&super::Solution::count_and_say(4), "1211");
    }
}
