use std::collections::HashMap;
pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

pub fn median(numbers: &mut Vec<i32>) -> f32 {
    numbers.sort();
    let mut numbers_copy = numbers.clone();
    let vector_size = numbers_copy.len() as i32;
    if is_even(vector_size) {
        (numbers_copy[((vector_size / 2) - 1 ) as usize] as f32 + numbers_copy[(vector_size / 2) as usize ] as f32) / 2.0 as f32
    } else {
        numbers_copy[(vector_size / 2) as usize] as f32 as f32
    }
}

pub fn mode(numbers: &Vec<i32>) -> Vec<i32> {
    let mut mode_freq: HashMap<i32, i32> = HashMap::new();
    for i in numbers {
        *mode_freq.entry(*i).or_insert(0) += 1;
    }
    let max_freq = *mode_freq.values().max().unwrap();
    let mut modes: Vec<i32> = mode_freq
    .into_iter()
    .filter(|(_, freq)| *freq == max_freq)
    .map(|(num, _)| num)
    .collect();
    modes.sort();
    modes
}

pub fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let median_result = median(&mut numbers);
    assert_eq!(median_result, 5.0);
    let mut numbers2 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let median2_result = median(&mut numbers2);
    assert_eq!(median2_result, 5.5);
    let numbers = vec![1, 2, 2, 2, 3, 4];
    assert_eq!(mode(&numbers), vec![2]);

    let numbers = vec![1, 3, 3, 6, 7, 8, 9];
    assert_eq!(mode(&numbers), vec![3]);

    let numbers = vec![2, 2, 2, 2, 2, 2, 2, 100, 100, 100, 100, 100, 100, 100, 100];
    assert_eq!(mode(&numbers), vec![100]);

    let numbers = vec![1, 1, 2, 2];
    assert_eq!(mode(&numbers), vec![1, 2]);
    println!("All tests passed");
}
