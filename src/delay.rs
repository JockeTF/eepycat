use std::ops::Range;
use std::str::Chars;
use std::time::Duration;

use fastrand::Rng;

const INDENT: [char; 2] = [' ', '\t'];

pub struct CharDelay<'a> {
    cursor: Chars<'a>,
    escape: bool,
    indent: bool,
    random: Rng,
}

impl CharDelay<'_> {
    fn delay_range(&mut self, char: char) -> Range<u64> {
        if self.escape || char == '\x1b' {
            self.escape = char != 'm';
            0..1
        } else if self.indent {
            self.indent = INDENT.contains(&char);
            match char {
                ' ' | '\t' => 0..1,
                _ => 500..1000,
            }
        } else {
            self.indent = char == '\n';
            match char {
                ' ' | '0'..='9' => 75..150,
                '-' | '+' | '.' => 75..175,
                '<' | '>' => 200..400,
                'a'..='z' => 75..200,
                'A'..='Z' => 75..350,
                '\n' => 500..1000,
                _ => 100..400,
            }
        }
    }
}

impl<'a> From<Chars<'a>> for CharDelay<'a> {
    fn from(value: Chars<'a>) -> Self {
        Self {
            cursor: value,
            escape: false,
            indent: false,
            random: Rng::new(),
        }
    }
}

impl Iterator for CharDelay<'_> {
    type Item = (Duration, char);

    fn next(&mut self) -> Option<Self::Item> {
        let char = self.cursor.next()?;
        let range = self.delay_range(char);
        let millis = self.random.u64(range);
        let duration = Duration::from_millis(millis);

        Some((duration, char))
    }
}
