use slice_group_by::GroupBy;
use std::collections::HashMap;

/// Given a list of integers, use a vector and return
/// - the mean (the average value)
/// - median (when sorted, the value in the middle position)
/// - and mode (the value that occurs most often; a hash map will be helpful here)
///
/// of the list.
///
/// 整数のリストが与えられたとき、ベクトルを使ってリストの
/// - 平均値（平均的な値）
/// - 中央値（ソートしたときに中央に位置する値）
/// - 最頻値（最も頻繁に出現する値)
///
/// を返却する。
///
/// # example #1
///
pub fn mean_median_mode(list: Vec<i32>) -> (f64, i32, i32) {
    if list.len() == 0 {
        panic!("Specify one or more numbers.");
    }
    let mean = mean(&list);
    let median = median(&list);
    let mode = mode(&list);
    (mean, median, mode)
}

/// Given a list of integers, use a vector and return the mean (the average value).
/// 整数のリストが与えられたとき、ベクトルを使ってリストの平均値（平均的な値）を返却する
fn mean(list: &Vec<i32>) -> f64 {
    list.iter().as_slice().iter().sum::<i32>() as f64 / list.len() as f64
}

/// Given a list of integers, use a vector and return the median (when sorted, the value in the middle position).
/// 中央値（ソートしたときに中央に位置する値）を返却する。
fn median(list: &Vec<i32>) -> i32 {
    let mut sorted = list.clone();
    sorted.sort();
    sorted[sorted.len() / 2]
}

/// Given a list of integers, use a vector and return the mode (the value that occurs most often; a hash map will be helpful here).
/// 最頻値（最も頻繁に出現する値)を返却する。
fn mode(list: &Vec<i32>) -> i32 {
    let mut sorted = list.clone();
    sorted.sort();
    let mut most_often_value: i32 = -1;
    let mut map = HashMap::new();
    for group_of_same_value in sorted.linear_group_by(|a, b| a == b) {
        let current_value = group_of_same_value[0];
        map.insert(current_value, group_of_same_value.len());
        most_often_value = match map.get(&most_often_value) {
            Some(most_often_value_len) => {
                if &group_of_same_value.len() < most_often_value_len {
                    most_often_value
                } else {
                    current_value
                }
            }
            None => current_value,
        }
    }
    most_often_value
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn take_1_return_1_1_1() {
        let ret = mean_median_mode(vec![1]);
        assert_eq!(ret, (1.0, 1, 1));
    }
    #[test]
    fn it_can_take_negative_number() {
        let ret = mean_median_mode(vec![-2, 1, 2, -2]);
        let mean: f64 = -1.0 / 4.0;
        let median = 1;
        let mode = -2;
        assert_eq!(ret, (mean, median, mode));
    }
    #[test]
    #[should_panic(expected = "Specify one or more numbers.")]
    fn take_none() {
        mean_median_mode(vec![]);
    }
}
