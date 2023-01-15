pub(crate) fn is_alphanumeric_underscore_hyphen(name: &str) -> bool {
    if name.len() == 0 {
        return false;
    }

    name.chars().all(is_alphanumeric_underscore_hyphen_char)
}

pub(crate) fn is_alphanumeric_underscore_hyphen_char(c: char) -> bool {
    match c {
        'a'..='z' => true,
        'A'..='Z' => true,
        '0'..='9' => true,
        '-' => true,
        '_' => true,
        _ => false,
    }
}
