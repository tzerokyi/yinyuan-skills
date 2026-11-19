//! Headless, deterministic rules adapted from the yinyuan reference skill.
//!
//! These operations describe a cultural entertainment tradition, not a
//! scientific compatibility or relationship prediction. Birth-chart and
//! solar-term calculations are deliberately outside this crate's scope.

pub mod bazi;
pub mod fortune;
pub mod redline;
pub mod taohua;
pub mod ziwei;
pub mod zodiac;

use std::fmt;

pub const DISCLAIMER: &str = "Traditional-cultural entertainment only; not a factual prediction or relationship decision.";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    InvalidInput(&'static str),
    MissingData(&'static str),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInput(message) => write!(f, "invalid input: {message}"),
            Self::MissingData(message) => write!(f, "missing data: {message}"),
        }
    }
}

impl std::error::Error for Error {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Element {
    Wood,
    Fire,
    Earth,
    Metal,
    Water,
}

impl Element {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Wood => "wood",
            Self::Fire => "fire",
            Self::Earth => "earth",
            Self::Metal => "metal",
            Self::Water => "water",
        }
    }

    const fn index(self) -> u8 {
        match self {
            Self::Wood => 0,
            Self::Fire => 1,
            Self::Earth => 2,
            Self::Metal => 3,
            Self::Water => 4,
        }
    }

    /// Relationship of the left element to the right element.
    pub const fn relation_to(self, other: Self) -> ElementRelation {
        if self.index() == other.index() {
            ElementRelation::Same
        } else if (self.index() + 1) % 5 == other.index() {
            ElementRelation::Generates
        } else if (other.index() + 1) % 5 == self.index() {
            ElementRelation::GeneratedBy
        } else if (self.index() + 2) % 5 == other.index() {
            ElementRelation::Controls
        } else {
            ElementRelation::ControlledBy
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElementRelation {
    Same,
    Generates,
    GeneratedBy,
    Controls,
    ControlledBy,
}

impl ElementRelation {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Same => "same",
            Self::Generates => "generates",
            Self::GeneratedBy => "generated_by",
            Self::Controls => "controls",
            Self::ControlledBy => "controlled_by",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Branch {
    Zi,
    Chou,
    Yin,
    Mao,
    Chen,
    Si,
    Wu,
    Wei,
    Shen,
    You,
    Xu,
    Hai,
}

impl Branch {
    pub const ALL: [Self; 12] = [
        Self::Zi, Self::Chou, Self::Yin, Self::Mao, Self::Chen, Self::Si,
        Self::Wu, Self::Wei, Self::Shen, Self::You, Self::Xu, Self::Hai,
    ];

    pub const fn index(self) -> usize {
        match self {
            Self::Zi => 0, Self::Chou => 1, Self::Yin => 2, Self::Mao => 3,
            Self::Chen => 4, Self::Si => 5, Self::Wu => 6, Self::Wei => 7,
            Self::Shen => 8, Self::You => 9, Self::Xu => 10, Self::Hai => 11,
        }
    }

    pub const fn hanzi(self) -> &'static str {
        match self {
            Self::Zi => "子", Self::Chou => "丑", Self::Yin => "寅",
            Self::Mao => "卯", Self::Chen => "辰", Self::Si => "巳",
            Self::Wu => "午", Self::Wei => "未", Self::Shen => "申",
            Self::You => "酉", Self::Xu => "戌", Self::Hai => "亥",
        }
    }

    pub fn parse(input: &str) -> Option<Self> {
        let input = input.trim();
        Self::ALL.iter().copied().find(|branch| {
            branch.hanzi() == input || zodiac::Animal::from_branch(*branch).as_str() == input
                || zodiac::Animal::from_branch(*branch).hanzi() == input
        })
    }

    pub const fn element(self) -> Element {
        match self {
            Self::Zi | Self::Hai => Element::Water,
            Self::Yin | Self::Mao => Element::Wood,
            Self::Si | Self::Wu => Element::Fire,
            Self::Shen | Self::You => Element::Metal,
            _ => Element::Earth,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stem {
    Jia,
    Yi,
    Bing,
    Ding,
    Wu,
    Ji,
    Geng,
    Xin,
    Ren,
    Gui,
}

impl Stem {
    pub const ALL: [Self; 10] = [
        Self::Jia, Self::Yi, Self::Bing, Self::Ding, Self::Wu,
        Self::Ji, Self::Geng, Self::Xin, Self::Ren, Self::Gui,
    ];

    pub const fn index(self) -> usize {
        match self {
            Self::Jia => 0, Self::Yi => 1, Self::Bing => 2, Self::Ding => 3,
            Self::Wu => 4, Self::Ji => 5, Self::Geng => 6, Self::Xin => 7,
            Self::Ren => 8, Self::Gui => 9,
        }
    }

    pub const fn hanzi(self) -> &'static str {
        match self {
            Self::Jia => "甲", Self::Yi => "乙", Self::Bing => "丙",
            Self::Ding => "丁", Self::Wu => "戊", Self::Ji => "己",
            Self::Geng => "庚", Self::Xin => "辛", Self::Ren => "壬", Self::Gui => "癸",
        }
    }

    pub fn parse(input: &str) -> Option<Self> {
        Self::ALL.iter().copied().find(|stem| stem.hanzi() == input.trim())
    }

    pub const fn element(self) -> Element {
        match self {
            Self::Jia | Self::Yi => Element::Wood,
            Self::Bing | Self::Ding => Element::Fire,
            Self::Wu | Self::Ji => Element::Earth,
            Self::Geng | Self::Xin => Element::Metal,
            Self::Ren | Self::Gui => Element::Water,
        }
    }
}
