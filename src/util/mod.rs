pub fn is_digit(ch: char) -> bool {
    return ch.is_digit(10);
}

pub fn is_alpha(ch: char) -> bool {
    return ch.is_alphabetic();
}

pub fn is_alpha_num(ch: char) -> bool {
    return ch.is_alphanumeric();
}

pub fn align_to(offset: i32, align: i32) -> i32 {
    (offset + align - 1) / align * align
}

pub fn x86_program_offset(offset: i32) -> String {
    let aligned = align_to(offset, 16);
    append_str("$", aligned, "")
}

pub fn append_str<T: ToString>(s: &str, val: T, suffix: &str) -> String {
    let mut output = s.to_string();
    output.push_str(val.to_string().as_str());
    if suffix.len() > 0 {
        output.push_str(suffix);
    }

    output
}
