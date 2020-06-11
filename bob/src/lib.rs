pub fn is_uppercase(text: &str) -> bool {
    let mut contain_letter = false;
    for letter in text.chars() {
        if letter.is_alphabetic() {
            contain_letter = true;
        }
        if letter.is_lowercase() {
            return false;
        }
    }
    true && contain_letter
}

pub fn reply(message: &str) -> &str {
    let is_question = message.trim().ends_with("?");
    let is_uppercase = is_uppercase(message);
    let is_empty = message.trim().is_empty();

    if is_empty {
        return "Fine. Be that way!";
    } else if is_question && is_uppercase {
        return "Calm down, I know what I'm doing!";
    } else if is_question {
        return "Sure.";
    } else if is_uppercase {
        return "Whoa, chill out!";
    } else {
        return "Whatever.";
    }
}