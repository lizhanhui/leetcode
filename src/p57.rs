struct Solution;

impl Solution {
    pub fn insert(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        if let Some((i, _)) = intervals
            .iter()
            .enumerate()
            .find(|(i, r)| r[0] > new_interval[0])
        {
            intervals.insert(i, new_interval);
        } else {
            intervals.insert(0, new_interval);
        }
        Self::merge(intervals)
    }

    fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_by(|lhs, rhs| {
            lhs.first()
                .unwrap()
                .partial_cmp(rhs.first().unwrap())
                .unwrap()
        });
        let mut ans = vec![];
        let mut cur = vec![];
        intervals.iter().for_each(|r| {
            if cur.is_empty() {
                cur.extend_from_slice(r);
                return;
            }

            let mut c_itr = cur.iter_mut();
            let c_s = c_itr.next().unwrap();
            let c_e = c_itr.next().unwrap();

            let s = r[0];
            let e = r[1];

            if &s > c_e {
                let mut n = vec![];
                n.push(*c_s);
                n.push(*c_e);
                ans.push(n);

                *c_s = s;
                *c_e = e;
                return;
            }

            if &s <= c_e && &e > c_e {
                *c_e = e;
            }
        });
        ans.push(cur);
        ans
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn case1() {
        let intervals = vec![vec![1, 3], vec![6, 9]];
        let new_interval = vec![2, 5];
        let ans = vec![vec![1, 5], vec![6, 9]];
        assert_eq!(ans, super::Solution::insert(intervals, new_interval));
    }

    #[test]
    fn case2() {
        let intervals = vec![vec![1, 5]];
        let new_interval = vec![2, 7];
        let ans = vec![vec![1, 7]];
        assert_eq!(ans, super::Solution::insert(intervals, new_interval));
    }

    #[test]
    fn case3() {
        let intervals = vec![];
        let new_interval = vec![2, 7];
        let ans = vec![vec![2, 7]];
        assert_eq!(ans, super::Solution::insert(intervals, new_interval));
    }
}
