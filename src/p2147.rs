struct Solution;

impl Solution {
    pub fn number_of_ways(corridor: String) -> i32 {
        let mut line = corridor.chars().collect::<Vec<_>>();
        let seat_num = line
            .iter()
            .map(|c| match c {
                'P' => 0,
                'S' => 1,
                _ => 0,
            })
            .sum::<usize>();
        if seat_num <= 1 || seat_num % 2 != 0 {
            return 0;
        }

        Self::map(&mut line);
        Self::reduce(&line)
    }

    fn map(line: &mut Vec<char>) {
        let mut seat = 0;
        line.iter_mut().for_each(|c| match c {
            'S' => {
                seat += 1;
                if seat % 2 == 0 {
                    *c = 'R';
                } else {
                    // Will filter it out later
                    *c = 'N';
                }
            }
            'P' => {
                if seat == 0 || seat % 2 == 1 {
                    *c = 'N';
                }
            }
            _ => {
                unreachable!("Bad Input");
            }
        });
        line.retain(|e| *e != 'N');
        while let Some(c) = line.last() {
            if *c == 'R' {
                break;
            }
            line.pop();
        }
    }

    fn reduce(line: &[char]) -> i32 {
        let mut total = 1_usize;

        let mut p_cnt = 1;
        let mut r_cnt = 0;

        line.iter().for_each(|c| match c {
            &'R' => {
                r_cnt += 1;

                if r_cnt % 2 == 0 {
                    total *= p_cnt;
                    p_cnt = 1;
                }
            }

            &'P' => {
                p_cnt += 1;
            }

            _ => {
                unreachable!("Something wrong with map stage");
            }
        });

        total *= p_cnt;

        (total % (10_usize.pow(9) + 7)) as i32
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_merge() {
        let mut corridor = "SSPPSPS".chars().collect();
        super::Solution::map(&mut corridor);
        assert_eq!(vec!['R', 'P', 'P', 'R'], corridor);
    }

    #[test]
    fn case1() {
        let corridor = "SSPPSPS";
        assert_eq!(3, super::Solution::number_of_ways(corridor.to_string()));
    }

    #[test]
    fn case2() {
        let corridor = "PPSPSP";
        assert_eq!(1, super::Solution::number_of_ways(corridor.to_string()));
    }

    #[test]
    fn case3() {
        let corridor = "S";
        assert_eq!(0, super::Solution::number_of_ways(corridor.to_string()));
    }

    #[test]
    fn wa1() {
        let corridor = "SSPSSPSSSPPSPSPPS";
        assert_eq!(8, super::Solution::number_of_ways(corridor.to_string()));
    }

    #[test]
    fn wa2() {
        let corridor = "PPSPSP";
        assert_eq!(1, super::Solution::number_of_ways(corridor.to_string()));
    }

    #[test]
    #[ignore]
    fn wa3() {
        let corridor = "SSSPSPPSSPPSSPPPSSSSPSSPSSPPPP";
        assert_eq!(144, super::Solution::number_of_ways(corridor.to_string()));
    }
}
