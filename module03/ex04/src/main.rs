use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Result;
use std::str::FromStr;

#[derive(Debug)]
struct Time {
    hours: u32,
    minutes: u32,
}

#[derive(Debug)]
enum TimeParseError {
    MissingColon,
    InvalidLength,
    InvalidNumber,
}

impl FromStr for Time {
    type Err = TimeParseError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if s.len() != 5 {
            return Err(TimeParseError::InvalidLength);
        }
        let b = s.as_bytes();
        if !b[0].is_ascii_digit()
            || !b[1].is_ascii_digit()
            || !b[3].is_ascii_digit()
            || !b[4].is_ascii_digit()
        {
            return Err(TimeParseError::InvalidNumber);
        } else if b[2] != b':' {
            return Err(TimeParseError::MissingColon);
        }

        let t = Time {
            hours: (b[0] - b'0') as u32 * 10 + (b[1] - b'0') as u32,
            minutes: (b[3] - b'0') as u32 * 10 + (b[4] - b'0') as u32,
        };
        if t.hours >= 24 || t.minutes >= 60 {
            return Err(TimeParseError::InvalidNumber);
        }
        Ok(t)
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        match (self.hours, self.minutes) {
            (1, 1) => write!(f, "{} hour, {} minute", self.hours, self.minutes),
            (1, _) => write!(f, "{} hour, {} minutes", self.hours, self.minutes),
            (_, 1) => write!(f, "{} hours, {} minute", self.hours, self.minutes),
            (_, _) => write!(f, "{} hours, {} minutes", self.hours, self.minutes),
        }
    }
}

impl Display for TimeParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        match self {
            TimeParseError::MissingColon => f.write_str("missing ':'"),
            TimeParseError::InvalidLength => f.write_str("invalid length"),
            TimeParseError::InvalidNumber => f.write_str("invalid number"),
        }
    }
}

fn main() {
    let a: Time = "12:20".parse().unwrap();
    let b: Time = "15:14".parse().unwrap();

    println!("{a}");
    println!("{b}");

    let err1: TimeParseError = "12.20".parse::<Time>().unwrap_err();
    let err2: TimeParseError = "12:2".parse::<Time>().unwrap_err();
    let err3: TimeParseError = "12:2a".parse::<Time>().unwrap_err();
    println!("error: {err1}");
    println!("error: {err2}");
    println!("error: {err3}");
}
