pub fn into_lines(input: String) -> Vec<String> {
    input
        .split("\n")
        .map(|line| line.trim().to_owned())
        .filter(|line| line.len() > 0)
        .collect()
}

pub fn split(text: String, delimiters: &[char]) -> Vec<String> {
    text.split(delimiters)
        .map(|item| item.trim().to_owned())
        .filter(|item| !item.is_empty())
        .collect::<Vec<String>>()
}
