fn swap(input: &mut Vec<char>, start: usize, end: usize) {
    input.swap(start, end)
}

pub fn reverse(input: &str) -> String {
    if input.is_empty() {
        return String::new();
    }

    let mut input_chars: Vec<char> = input.chars().collect();

    let mut start = 0;
    let mut end = input_chars.len() - 1;

    while start < end {
        swap(&mut input_chars, start, end);
        start += 1;
        end -= 1;
    }

    input_chars.into_iter().collect()
}