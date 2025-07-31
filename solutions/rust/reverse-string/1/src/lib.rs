pub fn reverse(input: &str) -> String {
    let mut reversed = String::new();
    for i in input.chars().rev() {
        reversed.push(i);
    }

    reversed
}
