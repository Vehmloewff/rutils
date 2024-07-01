use std::ops::{Add, Mul};

pub fn get_sum<T: Add<Output = T> + Clone>(items: &[T]) -> Option<T> {
    let mut acc = None;

    for item in items {
        if let Some(local_acc) = acc {
            acc = Some(local_acc + item.clone())
        } else {
            acc = Some(item.clone())
        }
    }

    acc
}

pub fn get_product<T: Mul<Output = T> + Clone>(items: &[T]) -> Option<T> {
    let mut acc = None;

    for item in items {
        if let Some(local_acc) = acc {
            let owned_item = item.clone();

            acc = Some(local_acc * owned_item);
        } else {
            acc = Some(item.clone())
        }
    }

    acc
}

pub fn min<T: PartialOrd + Clone>(items: &[T]) -> Option<T> {
    let mut lowest = None;

    for item in items {
        if let Some(local_lowest) = &lowest {
            if item < local_lowest {
                lowest = Some(item.clone())
            }
        } else {
            lowest = Some(item.clone())
        }
    }

    lowest
}

pub fn max<T: PartialOrd + Clone>(items: &[T]) -> Option<T> {
    let mut highest = None;

    for item in items {
        if let Some(local_highest) = &highest {
            if item > local_highest {
                highest = Some(item.clone())
            }
        } else {
            highest = Some(item.clone())
        }
    }

    highest
}

pub fn collect_numbers(input: &[String]) -> Vec<f64> {
    input
        .iter()
        .map(|item| item.parse::<f64>().unwrap())
        .collect::<Vec<_>>()
}

pub fn safe_subtract_u64(a: u64, b: u64) -> u64 {
    if b > a {
        0
    } else {
        a - b
    }
}

pub fn safe_subtract_usize(a: usize, b: usize) -> usize {
    if b > a {
        0
    } else {
        a - b
    }
}

pub fn safe_subtract_u32(a: u32, b: u32) -> u32 {
    if b > a {
        0
    } else {
        a - b
    }
}

pub fn get_factors(number: u64) -> Vec<u64> {
    (1..number + 1)
        .into_iter()
        .filter(|&x| number % x == 0)
        .collect::<Vec<u64>>()
}

pub fn get_common_numbers(mut numbers: Vec<Vec<u64>>) -> Vec<u64> {
    let driver = numbers.pop().unwrap();
    let mut common = Vec::new();

    for number in driver {
        let mut is_missing = false;

        for list in &numbers {
            if !list.contains(&number) {
                is_missing = true
            }
        }

        if is_missing {
            common.push(number)
        }
    }

    common
}

pub fn least_common_multiple(numbers: &[u64]) -> u64 {
    let mut answer = numbers.get(0).unwrap().clone();

    for index in 1..numbers.len() {
        let number = numbers.get(index).unwrap().clone();
        answer = (number * answer) / (greatest_common_factor(number, answer));
    }

    answer
}

pub fn greatest_common_factor(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        greatest_common_factor(b, a % b)
    }
}
