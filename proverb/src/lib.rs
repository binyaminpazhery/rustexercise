pub fn build_proverb(list: &[&str]) -> String {
    
    let mut words = list.iter();
    let first = match words.next() {
        Some(x) => x,
        None => return String::new()
    };
    let mut need = first;
    let mut lines = vec!();
    for lost in words {
        lines.push(
            format!("For want of a {} the {} was lost.", need, lost));
        need = lost;
    }
    
    lines.push(format!("And all for the want of a {}.", first));
    lines.join("\n")
}