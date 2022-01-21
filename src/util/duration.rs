pub fn to_ms<S: Into<String>>(s: S) -> Option<i32> {
    let mut ms = 0;
    let mut digits = 0;
    for c in s.into().chars() {
        if c.is_ascii_digit() {
            digits *= 10;
            digits += c.to_digit(10).unwrap();
        } else {
            ms += match c {
                's' => digits * 1000,
                'm' => digits * 60 * 1000,
                'h' => digits * 60 * 60 * 1000,
                'd' => digits * 24 * 60 * 60 * 1000,
                _ => { return None; }
            };

            digits = 0;
        }
    }

    Some(ms as i32)
}

pub fn from_ms(ms: i32) -> String {
    let mut s = ms / 1000;
    let days = s / 3600 / 24;
    s -= days * 3600 * 24;
    let hours = s / 3600;
    s -= hours * 3600;
    let mins = s / 60;
    s -= mins * 60;

    format!("{}d {}h {}m {}s", days, hours, mins, s)
}