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
}
