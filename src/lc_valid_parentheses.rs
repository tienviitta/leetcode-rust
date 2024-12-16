pub fn is_valid(s: String) -> bool {
    let mut stack: Vec<char> = Vec::new();
    for ch in s.chars() {
        match ch {
            '(' => stack.push(ch),
            '{' => stack.push(ch),
            '[' => stack.push(ch),
            ')' => {
                if stack.is_empty() || stack.pop() != Some('(') {
                    return false;
                }
            }
            '}' => {
                if stack.is_empty() || stack.pop() != Some('{') {
                    return false;
                }
            }
            ']' => {
                if stack.is_empty() || stack.pop() != Some('[') {
                    return false;
                }
            }
            _ => return false,
        }
    }
    stack.is_empty()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn ex1() {
        assert_eq!(is_valid(String::from("()")), true);
    }
    #[test]
    fn ex2() {
        assert_eq!(is_valid(String::from("()[]{}")), true);
    }
}
