use std::collections::VecDeque;

struct Solution;

impl Solution {
    fn merge(l: &mut (i32, i32), r: &(i32, i32)) -> bool {
        if r.0 <= l.1 {
            if r.1 > l.1 {
                l.1 = r.1;
            }
            return true;
        }

        false
    }
    fn covers(taps: &[i32], ranges: &[i32]) -> bool {
        let len = (ranges.len() - 1) as i32;
        let mut slices = taps
            .iter()
            .map(|tap| {
                (
                    0.max(*tap - ranges[*tap as usize]),
                    len.min(*tap + ranges[*tap as usize]),
                )
            })
            .collect::<Vec<_>>();
        slices.sort_by(|l, r| l.0.cmp(&r.0));
        let res = slices.into_iter().reduce(|mut acc, e| {
            Self::merge(&mut acc, &e);
            acc
        });
        if let Some((b, e)) = res {
            return b <= 0 && e >= len;
        }

        false
    }

    pub fn min_taps(n: i32, ranges: Vec<i32>) -> i32 {
        // Use backtracking
        let mut candidates = VecDeque::new();

        for i in 0..=n {
            candidates.push_back(vec![i]);
        }

        while let Some(v) = candidates.pop_front() {
            if Self::covers(&v, &ranges) {
                return v.len() as i32;
            }
            if let Some(&max) = v.iter().max() {
                for i in (max + 1)..=n {
                    let mut opt = v.clone();
                    opt.push(i);
                    candidates.push_back(opt);
                }
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn case1() {
        let n = 5;
        let ranges = vec![3, 4, 1, 1, 0, 0];
        assert_eq!(1, super::Solution::min_taps(n, ranges));
    }

    #[test]
    fn case2() {
        let n = 3;
        let ranges = vec![0, 0, 0, 0];
        assert_eq!(-1, super::Solution::min_taps(n, ranges));
    }
}
