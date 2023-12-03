struct Solution;

impl Solution {
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        let mut time = 0;

        let mut current = vec![];
        points.iter().for_each(|point| {
            if current.is_empty() {
                current.extend_from_slice(point);
                return;
            }

            time += Self::cost(&mut current, point);
        });

        time
    }

    fn cost(current: &mut [i32], point: &[i32]) -> i32 {
        assert!(2 == current.len());
        assert!(2 == point.len());
        let mut time = 0;
        loop {
            if current[0] == point[0] && current[1] == point[1] {
                break;
            }
            if current[0] == point[0] {
                time += 1;
                if current[1] < point[1] {
                    current[1] += 1;
                } else {
                    current[1] -= 1;
                }
                continue;
            }

            if current[1] == point[1] {
                time += 1;
                if current[0] < point[0] {
                    current[0] += 1;
                } else {
                    current[0] -= 1;
                }
                continue;
            }

            if current[0] < point[0] {
                current[0] += 1;
            } else {
                current[0] -= 1;
            }

            if current[1] < point[1] {
                current[1] += 1;
            } else {
                current[1] -= 1;
            }
            time += 1;
        }

        time
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn case1() {
        let points = vec![vec![1, 1], vec![3, 4], vec![-1, 0]];
        assert_eq!(7, super::Solution::min_time_to_visit_all_points(points));
    }

    #[test]
    fn case2() {
        let points = vec![vec![3, 2], vec![-2, 2]];
        assert_eq!(5, super::Solution::min_time_to_visit_all_points(points));
    }
}
