//! Reproducible draw and verbatim reference corpus of 100 fortune sticks.

use crate::Error;

const CORPUS: &str = include_str!("../references/fortune-sticks.md");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CivilDate {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

impl CivilDate {
    pub fn new(year: u16, month: u8, day: u8) -> Result<Self, Error> {
        if year == 0 || !(1..=12).contains(&month) {
            return Err(Error::InvalidInput("invalid Gregorian date"));
        }
        let leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
        let days = [31, if leap { 29 } else { 28 }, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        if day == 0 || day > days[(month - 1) as usize] {
            return Err(Error::InvalidInput("invalid Gregorian date"));
        }
        Ok(Self { year, month, day })
    }

    pub fn parse(text: &str) -> Result<Self, Error> {
        let mut components = text.split('-');
        let year = components.next().and_then(|v| v.parse::<u16>().ok());
        let month = components.next().and_then(|v| v.parse::<u8>().ok());
        let day = components.next().and_then(|v| v.parse::<u8>().ok());
        if components.next().is_some() {
            return Err(Error::InvalidInput("expected date YYYY-MM-DD"));
        }
        Self::new(
            year.ok_or(Error::InvalidInput("expected date YYYY-MM-DD"))?,
            month.ok_or(Error::InvalidInput("expected date YYYY-MM-DD"))?,
            day.ok_or(Error::InvalidInput("expected date YYYY-MM-DD"))?,
        )
    }

    /// Gregorian ordinal day, 0001-01-01 = 1. Not Unix epoch days.
    pub fn ordinal(self) -> u64 {
        let y = u64::from(self.year - 1);
        let leap = self.year % 4 == 0 && (self.year % 100 != 0 || self.year % 400 == 0);
        let months: [u8; 12] = [31, if leap { 29 } else { 28 }, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        365 * y + y / 4 - y / 100 + y / 400
            + months[..usize::from(self.month - 1)].iter().map(|m| u64::from(*m)).sum::<u64>()
            + u64::from(self.day)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grade {
    Highest,
    High,
    MediumHigh,
    Medium,
    MediumLow,
    Low,
    Lowest,
}

impl Grade {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Highest => "highest", Self::High => "high",
            Self::MediumHigh => "medium_high", Self::Medium => "medium",
            Self::MediumLow => "medium_low", Self::Low => "low",
            Self::Lowest => "lowest",
        }
    }
}

pub const fn grade(number: u8) -> Option<Grade> {
    match number {
        1..=10 => Some(Grade::Highest),
        11..=30 => Some(Grade::High),
        31..=45 => Some(Grade::MediumHigh),
        46..=55 => Some(Grade::Medium),
        56..=70 => Some(Grade::MediumLow),
        71..=85 => Some(Grade::Low),
        86..=100 => Some(Grade::Lowest),
        _ => None,
    }
}

/// Source rule: code-point sum + two-hour branch index + Gregorian day ordinal.
/// The caller explicitly supplies its local date and traditional hour branch.
/// The hour index must be 0..=11 (Zi..Hai); the same inputs always draw the same stick.
pub fn draw_number(name: &str, question: &str, date: CivilDate, hour_branch: u8) -> Result<u8, Error> {
    if hour_branch > 11 {
        return Err(Error::InvalidInput("hour branch index must be 0..=11"));
    }
    if name.trim().is_empty() || question.trim().is_empty() {
        return Err(Error::InvalidInput("name and question must not be blank"));
    }
    let sum: u64 = name.chars().chain(question.chars()).map(|c| u64::from(c as u32)).sum();
    Ok(((sum + u64::from(hour_branch) + date.ordinal()) % 100 + 1) as u8)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FortuneStick {
    pub number: u8,
    pub grade: Grade,
    pub poem: String,
    pub explanation: Option<String>,
    pub relationship_note: Option<String>,
}

/// Read the embedded original Markdown in declaration order. Never synthesize
/// missing verses or interpretations: the source only annotates a subset.
pub fn lookup(number: u8) -> Result<FortuneStick, Error> {
    let grade = grade(number).ok_or(Error::InvalidInput("fortune number must be 1..=100"))?;
    let mut ordinal = 0u8;
    let mut reading = false;
    let mut in_poem = false;
    let mut poem = String::new();
    let mut explanation = None;
    let mut relationship_note = None;
    for line in CORPUS.lines() {
        if line.starts_with("**第") && line.contains("签") {
            if reading {
                break;
            }
            ordinal += 1;
            reading = ordinal == number;
            continue;
        }
        if !reading {
            continue;
        }
        if line.trim() == "```" {
            in_poem = !in_poem;
            continue;
        }
        if in_poem {
            if !poem.is_empty() {
                poem.push('\n');
            }
            poem.push_str(line.trim());
        } else if let Some(note) = line.strip_prefix("签解：") {
            explanation = Some(note.trim().to_owned());
        } else if let Some(note) = line.strip_prefix("姻缘解读：") {
            relationship_note = Some(note.trim().to_owned());
        }
    }
    if !reading || poem.is_empty() {
        return Err(Error::MissingData("fortune-sticks.md does not contain the requested verse"));
    }
    Ok(FortuneStick { number, grade, poem, explanation, relationship_note })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_one_hundred_sticks_are_present() {
        for number in 1..=100 {
            let stick = lookup(number).unwrap();
            assert_eq!(stick.number, number);
            assert!(!stick.poem.is_empty());
        }
        assert!(lookup(0).is_err());
        assert!(lookup(101).is_err());
    }

    #[test]
    fn draw_is_deterministic_and_date_validated() {
        let day = CivilDate::new(2026, 11, 19).unwrap();
        let one = draw_number("测试", "缘分", day, 4).unwrap();
        assert_eq!(one, draw_number("测试", "缘分", day, 4).unwrap());
        assert!(draw_number("测试", "缘分", day, 12).is_err());
        assert!(CivilDate::new(2026, 2, 29).is_err());
        assert!(CivilDate::new(2024, 2, 29).is_ok());
    }
}
