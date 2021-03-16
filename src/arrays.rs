#![allow(dead_code)]

mod max_consecutive_ones {

    pub struct Solution;

    impl Solution {
        pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
            let mut max = 0i32;
            let mut current = 0i32;

            nums.iter().for_each(|x|{
                if *x == 1 {
                    current += 1;
                } else {
                    max = std::cmp::max(max, current);
                    current = 0;
                }
            });

            std::cmp::max(max, current)
        }
    }

}

mod find_numbers_with_even_number_of_digits {

    pub struct Solution;

    impl Solution {
        pub fn find_numbers(nums: Vec<i32>) -> i32 {
            nums.iter().map(|num|{
                let digits = [1, 10, 100, 1000, 10000, 100000];

                digits.iter().map(|digit| {
                    num / digit
                })
                    .filter(|remain| *remain != 0)
                    .count()
            })
                .filter(|num| num % 2 == 0)
                .count() as i32
        }
    }

}

mod squares_of_a_sorted_array {

    pub struct Solution;

    impl Solution {
        pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
            let mut v = nums.iter()
                .map(|num| num * num)
                .collect::<Vec<_>>();

            v.sort();

            v
        }
    }

}

// TODO: Попытаться реализовать алгоритм без использования дополнительной памяти.
// Возможно, в Расте этого нельзя сделать без unsafe блока, т.к. система заимствования
// в Расте не позволяет писать и читать одновременно из одной области памяти.
mod duplicate_zeros {

    pub struct Solution;

    impl Solution {
        pub fn duplicate_zeros(arr: &mut Vec<i32>) {
            let mut result: Vec<i32> = vec![];

            arr.iter_mut()
                .for_each(|x| {
                    if *x == 0 {
                        result.push(0);
                        result.push(0);
                    } else {
                        result.push(*x);
                    }
                });

            let len = arr.len();
            arr.copy_from_slice(&result[..len]);
        }
    }

}

// TODO: Попытаться реализовать алгоритм без использования дополнительной памяти.
mod merge_sorted_array {

    pub struct Solution;

    impl Solution {
        pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
            let mut i_iter = nums1.iter().take(m as usize);
            let mut j_iter = nums2.iter().take(n as usize);
            let mut res = vec![];
            let mut i = i_iter.next();
            let mut j = j_iter.next();

            loop {
                if let (Some(i_value), Some(j_value)) = (i, j) {
                    if i_value < j_value {
                        // Копируем значение из первого вектора
                        res.push(*i_value);
                        i = i_iter.next();
                    } else {
                        // Копируем значение из второго вектора
                        res.push(*j_value);
                        j = j_iter.next();
                    }
                } else if let Some(i) = i {
                    // Копируем остатки первого вектора
                    res.push(*i);
                    res.extend(i_iter.clone());
                    break;
                } else if let Some(j) = j {
                    // Копируем остатки второго вектора
                    res.push(*j);
                    res.extend(j_iter.clone());
                    break;
                }
            }

            nums1.copy_from_slice(&res);
        }
    }

}

#[cfg(test)]
mod tests {
     #[test]
     fn max_consecutive_ones() {
         use super::max_consecutive_ones::Solution;

         assert_eq!(Solution::find_max_consecutive_ones(vec![1,1,0,1,1,1]), 3);
         assert_eq!(Solution::find_max_consecutive_ones(vec![1,0,1,1,0,1]), 2);
     }

    #[test]
    fn find_numbers_with_even_number_of_digits() {
        use super::find_numbers_with_even_number_of_digits::Solution;

        assert_eq!(Solution::find_numbers(vec![12,345,2,6,7896]), 2);
        assert_eq!(Solution::find_numbers(vec![555,901,482,1771]), 1);
    }

    #[test]
    fn squares_of_a_sorted_array() {
        use super::squares_of_a_sorted_array::Solution;

        assert_eq!(Solution::sorted_squares(vec![-4,-1,0,3,10]), [0,1,9,16,100]);
        assert_eq!(Solution::sorted_squares(vec![-7,-3,2,3,11]), [4,9,9,49,121]);
    }

    #[test]
    fn duplicate_zeros() {
        use super::duplicate_zeros::Solution;

        let mut arr1 = vec![1,0,2,3,0,4,5,0];
        let mut arr2 = vec![1,2,3];

        Solution::duplicate_zeros(&mut arr1);
        assert_eq!(arr1.len(), 8);
        assert_eq!(arr1, [1, 0, 0, 2, 3, 0, 0, 4]);

        Solution::duplicate_zeros(&mut arr2);
        assert_eq!(arr2.len(), 3);
        assert_eq!(arr2, [1, 2, 3]);
    }

    #[test]
    fn merge_sorted_array() {
        use super::merge_sorted_array::Solution;

        let mut nums1 = vec![1,2,3,0,0,0];
        let m = 3;
        let mut nums2 = vec![2,5,6];
        let n = 3;
        Solution::merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, [1, 2, 2, 3, 5, 6]);

        let mut nums1 = vec![1];
        let m = 1;
        let mut nums2 = vec![];
        let n = 0;
        Solution::merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, [1]);
    }

}
