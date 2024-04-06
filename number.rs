use std::ops::Mul;

pub fn sum(items: Vec<u64>) -> u64 {
    let mut acc = 0;

    for item in items {
        acc += item;
    }

    acc
}

pub fn mul<N: Mul<Output = N>>(items: Vec<N>) -> N {
    let mut acc = None;

    for item in items {
        if acc.is_none() {
            acc = Some(item)
        } else {
            acc = Some(acc.unwrap() * item);
        }
    }

    acc.unwrap()
}

pub fn min(items: Vec<u64>) -> u64 {
    let mut lowest = items.first().unwrap().to_owned();

    for item in items {
        if item < lowest {
            lowest = item
        }
    }

    lowest.to_owned()
}

pub fn max(items: Vec<u64>) -> u64 {
    let mut highest = items.first().unwrap().to_owned();

    for item in items {
        if item > highest {
            highest = item
        }
    }

    highest.to_owned()
}

pub fn collect_numbers(input: Vec<String>) -> Vec<u64> {
    input
        .iter()
        .map(|item| item.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
}

pub fn sub_u64(a: u64, b: u64) -> u64 {
    if b > a {
        0
    } else {
        a - b
    }
}

pub fn sub_usize(a: usize, b: usize) -> usize {
    if b > a {
        0
    } else {
        a - b
    }
}

pub fn sub_u32(a: u32, b: u32) -> u32 {
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

pub fn least_common_multiple(numbers: Vec<u64>) -> u64 {
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
