pub fn brackets_are_balanced(string: &str) -> bool {
    let mut set = Vec::new();

    for bkt in string.chars() {
        match bkt {
            '[' | '{' | '(' => set.push(bkt),
            ']' => {
                if set.pop() != Some('[') {
                    return false;
                }
            }
            '}' => {
                if set.pop() != Some('{') {
                    return false;
                }
            }
            ')' => {
                if set.pop() != Some('(') {
                    return false;
                }
            }
            _ => continue,
        }
    }

    set.is_empty()
}