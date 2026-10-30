use std::cmp::Ordering;

/// A value in Scratch.
///
/// The [`PartialEq`] implementation for this enum is created with [`derive`], so it is not
/// the same as the `<a = b>` block in Scratch which is case-insensitive and treats text that
/// look like numbers as numbers. For that, you should use [`Value::scratch_eq`] which matches the
/// equation implementation of Scratch. For `<a > b>` and `<a < b>` you should use
/// [`Value::scratch_cmp`] which returns [`Ordering`].
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    Text(String),
    Boolean(bool),
}

impl Value {
    /// Compares a [`Value`] with another [`Value`] like how Scratch would.
    pub fn scratch_cmp(&self, other: &Value) -> Ordering {
        let (a, b) = (self.compare_number(), other.compare_number());
        if a.is_nan() || b.is_nan() {
            return self.compare_string().cmp(&other.compare_string());
        }
        a.partial_cmp(&b).unwrap()
    }

    /// Equates a [`Value`] with another [`Value`] like how Scratch would.
    pub fn scratch_eq(&self, other: &Self) -> bool {
        self.scratch_cmp(other) == Ordering::Equal
    }

    fn compare_number(&self) -> f64 {
        match self {
            Value::Number(n) => *n,
            Value::Boolean(b) => f64::from(u8::from(*b)),
            Value::Text(s) if s.trim().is_empty() => f64::NAN,
            Value::Text(s) => parse_js_number(s),
        }
    }

    fn compare_string(&self) -> String {
        match self {
            Value::Text(s) => s.to_lowercase(),
            Value::Boolean(b) => b.to_string(),
            Value::Number(n) => js_number_to_string(*n).to_lowercase(),
        }
    }
}

fn js_number_to_string(n: f64) -> String {
    if n == 0.0 { "0".into() } else { n.to_string() }
}

fn parse_js_number(s: &str) -> f64 {
    let s = s.trim();
    if s.is_empty() {
        return 0.0;
    }

    // no sign allowed, integers only for different bases
    let radix = match s.get(..2) {
        Some("0x" | "0X") => Some(16),
        Some("0b" | "0B") => Some(2),
        Some("0o" | "0O") => Some(8),
        _ => None,
    };
    if let Some(radix) = radix {
        return u64::from_str_radix(&s[2..], radix).map_or(f64::NAN, |n| n as f64);
    }

    match s {
        "Infinity" | "+Infinity" => return f64::INFINITY,
        "-Infinity" => return f64::NEG_INFINITY,
        _ => {}
    }

    if !s
        .bytes()
        .all(|b| matches!(b, b'0'..=b'9' | b'.' | b'e' | b'E' | b'+' | b'-'))
    {
        return f64::NAN;
    }
    s.parse().unwrap_or(f64::NAN)
}
