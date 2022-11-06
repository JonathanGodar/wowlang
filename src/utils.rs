pub fn extract_whitespace(s: &str) -> (&str, &str) {
    let whitespace_end = s.char_indices().find_map(|(idx, c)| if c == ' ' { None } else { Some(idx)} ).unwrap_or_else(|| s.len());

    let remainder = &s[whitespace_end..];
    let whitespaces = &s[..whitespace_end];
    (remainder, whitespaces)
}

pub fn extract_tag<'a>(s: &'a str, tag: &str) -> Result<&'a str, &'static str> {
    if s.starts_with(tag) {
        return Ok(&s[..tag.len()]);
    }

    Err("Unexpected identifier")
}


type TakeFunc = fn(char) -> bool;
pub fn take_while(s: &str, f: TakeFunc) -> (&str, &str) {
    let taken_end = s.char_indices().find_map(|(idx, c)| if f(c) { None } else {Some(idx)}).unwrap_or_else(|| s.len());

    let taken = &s[..taken_end];
    let remainder = &s[taken_end..];

    (remainder, taken)
}
