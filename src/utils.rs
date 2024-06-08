pub fn error(line: usize, message: String) -> Result<(), String> {
    report(line, "", &message);
    return Err(message)
}

pub fn report(line: usize, at: &str, message: &str) {
    eprintln!("[line {line}] Error {at}: {message}");
}

pub fn is_digit(x: char) -> bool {
    return '0' <= x && x <= '9'
}

pub fn is_alpha(x: char) -> bool {
    return ('a' <= x && x <= 'z') || ('A' <= x && x <= 'Z') || (x == '_')
}

pub fn is_alphanumeric(x: char) -> bool {
    return is_digit(x) || is_alpha(x)
}
