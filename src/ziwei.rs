//! Source-backed Ziwei marriage-palace star interpretations.
//! Chart generation belongs to an external ephemeris/chart engine.

use crate::Error;

const CORPUS: &str = include_str!("../references/ziwei-marriage.md");

pub const MAIN_STARS: [&str; 14] = [
    "紫微", "天机", "太阳", "武曲", "天同", "廉贞", "天府",
    "太阴", "贪狼", "巨门", "天相", "天梁", "七杀", "破军",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StarReading {
    pub star: String,
    pub partner_traits: String,
    pub relationship_style: String,
    pub challenge: String,
    pub advice: String,
}

/// Reads interpretations from the original preserved reference, not an LLM.
pub fn main_star(name: &str) -> Result<StarReading, Error> {
    let target = name.trim().trim_end_matches('星');
    if !MAIN_STARS.contains(&target) {
        return Err(Error::InvalidInput("unsupported Ziwei main star"));
    }
    let mut found = false;
    let mut traits = None;
    let mut style = None;
    let mut challenge = None;
    let mut advice = None;
    for line in CORPUS.lines() {
        if let Some(header) = line.strip_prefix("### ") {
            if found {
                break;
            }
            found = header == format!("{target}星");
            continue;
        }
        if !found {
            continue;
        }
        if let Some(value) = line.strip_prefix("**配偶特征**：") {
            traits = Some(value.trim().to_owned());
        } else if let Some(value) = line.strip_prefix("**感情模式**：") {
            style = Some(value.trim().to_owned());
        } else if let Some(value) = line.strip_prefix("**课题**：") {
            challenge = Some(value.trim().to_owned());
        } else if let Some(value) = line.strip_prefix("**月老建议**：") {
            advice = Some(value.trim().to_owned());
        }
    }
    Ok(StarReading {
        star: target.to_owned(),
        partner_traits: traits.ok_or(Error::MissingData("Ziwei partner traits"))?,
        relationship_style: style.ok_or(Error::MissingData("Ziwei relationship style"))?,
        challenge: challenge.ok_or(Error::MissingData("Ziwei challenge"))?,
        advice: advice.ok_or(Error::MissingData("Ziwei advice"))?,
    })
}

/// Multiple stars are interpreted independently; this is not a generated chart.
pub fn palace(stars: &[&str]) -> Result<Vec<StarReading>, Error> {
    if stars.is_empty() || stars.len() > 14 {
        return Err(Error::InvalidInput("provide 1..=14 main stars"));
    }
    stars.iter().map(|name| main_star(name)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_star_entries_are_backed_by_the_corpus() {
        for name in MAIN_STARS {
            let reading = main_star(name).unwrap();
            assert!(!reading.partner_traits.is_empty());
            assert!(!reading.advice.is_empty());
        }
        assert!(main_star("不存在").is_err());
    }
}
