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

mod remove_element {

    pub struct Solution;

    impl Solution {
        pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
            let mut len = nums.len();

            for i in (0..len).rev() {
                if nums[i] == val {
                    nums[i..].rotate_left(1);
                    len -= 1;
                }
            }

            len as i32
        }
    }

}

// TODO: Очень долго выполняется. 12 мс - это дольше, чем у большинства.
mod remove_duplicates_from_sorted_array {

    pub struct Solution;

    impl Solution {
        pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
            let mut len = nums.len();
            let mut prev;

            for i in 0..len {
                if i >= len {
                    break;
                }

                prev = nums[i];
                let count = (&nums[i..]).iter().take_while(|x|{ **x == prev }).count();

                if count > 1 {
                    let offset = count - 1;
                    nums[i + 1..].rotate_left(offset);
                    len -= offset;
                }
            }

            len as i32
        }
    }

}

mod check_if_n_and_its_double_exist {

    pub struct Solution;

    impl Solution {
        pub fn check_if_exist(arr: Vec<i32>) -> bool {
            for (i, &v) in arr.iter().enumerate() {
                // Пропускаем последний элемент
                if i == arr.len() - 1 {
                    break;
                }

                let r= arr[i + 1..].iter().find(|&&x|{
                    x * 2 == v || x == v * 2
                });

                if let Some(_) = r {
                    return true;
                }
            }

            false
        }
    }

}

mod valid_mountain_array {

    pub struct Solution;

    enum State {
        None,
        Increase,
        Decrease,
    }

    impl Solution {
        pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
            if arr.len() < 3 {
                return false;
            }

            let first = &arr[..arr.len() - 1];
            let second = &arr[1..arr.len()];
            let mut state = State::None;
            let mut increase = false;
            let mut decrease = false;

            for (&f, &s) in first.iter().zip(second.iter()) {
                match &mut state {
                    State::None => {
                        if f < s {
                            state = State::Increase;
                            increase = true;
                        } else if s <= f {
                            increase = false;
                            break;
                        }
                    },
                    State::Increase => {
                        if f > s {
                            state = State::Decrease;
                            decrease = true;
                        } else if f < s {
                        } else {
                            increase = false;
                            break;
                        }
                    },
                    State::Decrease => {
                        if !(f > s) {
                            decrease = false;
                            break;
                        }
                    },
                }

            }

            increase && decrease
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

    #[test]
    fn remove_element() {
        use super::remove_element::Solution;

        let mut nums = vec![3,2,2,3];
        let val = 3;
        assert_eq!(Solution::remove_element(&mut nums, val), 2);
        assert_eq!(nums[..2], [2,2]);

        let mut nums = vec![0,1,2,2,3,0,4,2];
        let val = 2;
        assert_eq!(Solution::remove_element(&mut nums, val), 5);
        assert_eq!(nums[..5], [0,1,3,0,4]);
    }

    #[test]
    fn remove_duplicates_from_sorted_array() {
        use super::remove_duplicates_from_sorted_array::Solution;

        let mut nums = vec![1,1,2];
        assert_eq!(Solution::remove_duplicates(&mut nums), 2);
        assert_eq!(nums[..2], [1,2]);

        let mut nums = vec![0,0,1,1,1,2,2,3,3,4];
        assert_eq!(Solution::remove_duplicates(&mut nums), 5);
        assert_eq!(nums[..5], [0,1,2,3,4]);
    }

    #[test]
    fn check_if_n_and_its_double_exist() {
        use super::check_if_n_and_its_double_exist::Solution;

        let arr = vec![10,2,5,3];
        assert_eq!(Solution::check_if_exist(arr), true);

        let arr = vec![7,1,14,11];
        assert_eq!(Solution::check_if_exist(arr), true);

        let arr = vec![3,1,7,11];
        assert_eq!(Solution::check_if_exist(arr), false);
    }

    #[test]
    fn valid_mountain_array() {
        use super::valid_mountain_array::Solution;

        let arr = vec![2,1];
        assert_eq!(Solution::valid_mountain_array(arr), false);

        let arr = vec![1,1,1,1,1,1,1,2,1];
        assert_eq!(Solution::valid_mountain_array(arr), false);

        let arr = vec![2,1,2,3,5,7,9,10,12,14,15,16,18,14,13];
        assert_eq!(Solution::valid_mountain_array(arr), false);

        let arr = vec![0,3,2,1];
        assert_eq!(Solution::valid_mountain_array(arr), true);

        let arr = vec![0,1,2,3,4,5,4,3,2,1,0];
        assert_eq!(Solution::valid_mountain_array(arr), true);
    }

}
