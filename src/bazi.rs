//! Bazi relationship rules over externally calculated year/day pillars.
//! Calendar, longitude, time-zone and solar-term chart computation are
//! intentionally not guessed from birth timestamps.

use crate::zodiac::{compatibility, Animal, Relation};
use crate::{Branch, ElementRelation, Error, Stem};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pillar {
    pub stem: Stem,
    pub branch: Branch,
}

impl Pillar {
    pub fn new(stem: Stem, branch: Branch) -> Result<Self, Error> {
        if stem.index() % 2 != branch.index() % 2 {
            return Err(Error::InvalidInput("stem and branch have incompatible yin/yang parity"));
        }
        Ok(Self { stem, branch })
    }

    pub fn parse(input: &str) -> Result<Self, Error> {
        let mut chars = input.trim().chars();
        let a = chars.next().ok_or(Error::InvalidInput("pillar requires two characters"))?;
        let b = chars.next().ok_or(Error::InvalidInput("pillar requires two characters"))?;
        if chars.next().is_some() {
            return Err(Error::InvalidInput("pillar requires exactly two characters"));
        }
        let stem = Stem::parse(&a.to_string()).ok_or(Error::InvalidInput("unknown heavenly stem"))?;
        let branch = Branch::parse(&b.to_string()).ok_or(Error::InvalidInput("unknown earthly branch"))?;
        Self::new(stem, branch)
    }

    pub fn sexagenary_index(self) -> usize {
        (0..60)
            .find(|index| index % 10 == self.stem.index() && index % 12 == self.branch.index())
            .expect("validated stem/branch parity guarantees a position")
    }

    pub fn nayin(self) -> &'static str {
        const NAYIN: [&str; 30] = [
            "海中金", "炉中火", "大林木", "路旁土", "剑锋金", "山头火",
            "涧下水", "城墙土", "白蜡金", "杨柳木", "泉中水", "屋上土",
            "霹雳火", "松柏木", "长流水", "砂石金", "山下火", "平地木",
            "壁上土", "金箔金", "覆灯火", "天河水", "大驿土", "钗钏金",
            "桑柘木", "大溪水", "沙中土", "天上火", "石榴木", "大海水",
        ];
        NAYIN[self.sexagenary_index() / 2]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChartInput {
    pub year: Pillar,
    pub day: Option<Pillar>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StemRelation {
    FiveCombination,
    Clash,
    Same,
    Generates,
    GeneratedBy,
    Controls,
    ControlledBy,
}

impl StemRelation {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FiveCombination => "five_combination",
            Self::Clash => "clash",
            Self::Same => "same",
            Self::Generates => "generates",
            Self::GeneratedBy => "generated_by",
            Self::Controls => "controls",
            Self::ControlledBy => "controlled_by",
        }
    }
}

pub fn stem_relation(a: Stem, b: Stem) -> StemRelation {
    let (x, y) = (a.index(), b.index());
    let is_pair = |pairs: &[(usize, usize)]| {
        pairs.iter().any(|&(i, j)| (x == i && y == j) || (x == j && y == i))
    };
    if is_pair(&[(0, 5), (1, 6), (2, 7), (3, 8), (4, 9)]) {
        return StemRelation::FiveCombination;
    }
    if is_pair(&[(0, 6), (1, 7), (2, 8), (3, 9)]) {
        return StemRelation::Clash;
    }
    match a.element().relation_to(b.element()) {
        ElementRelation::Same => StemRelation::Same,
        ElementRelation::Generates => StemRelation::Generates,
        ElementRelation::GeneratedBy => StemRelation::GeneratedBy,
        ElementRelation::Controls => StemRelation::Controls,
        ElementRelation::ControlledBy => StemRelation::ControlledBy,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BaziComparison {
    pub year_stems: StemRelation,
    pub year_branches: Relation,
    pub year_nayin_same: bool,
    pub year_nayin_a: &'static str,
    pub year_nayin_b: &'static str,
    pub day_stems: Option<StemRelation>,
    pub day_branches: Option<Relation>,
    pub day_elements: Option<ElementRelation>,
    pub partial: bool,
}

/// No synthetic composite score: the source supplies weights but does not
/// define an unambiguous per-factor scoring function.
pub fn compare(a: ChartInput, b: ChartInput) -> BaziComparison {
    let (day_stems, day_branches, day_elements) = match (a.day, b.day) {
        (Some(left), Some(right)) => (
            Some(stem_relation(left.stem, right.stem)),
            Some(compatibility(Animal::from_branch(left.branch), Animal::from_branch(right.branch))),
            Some(left.stem.element().relation_to(right.stem.element())),
        ),
        _ => (None, None, None),
    };
    BaziComparison {
        year_stems: stem_relation(a.year.stem, b.year.stem),
        year_branches: compatibility(
            Animal::from_branch(a.year.branch),
            Animal::from_branch(b.year.branch),
        ),
        year_nayin_same: a.year.nayin() == b.year.nayin(),
        year_nayin_a: a.year.nayin(),
        year_nayin_b: b.year.nayin(),
        day_stems,
        day_branches,
        day_elements,
        partial: day_stems.is_none(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parity_and_known_nayin() {
        assert_eq!(Pillar::parse("甲子").unwrap().nayin(), "海中金");
        assert_eq!(Pillar::parse("乙丑").unwrap().nayin(), "海中金");
        assert!(Pillar::parse("甲丑").is_err());
        assert!(Pillar::parse("甲子多").is_err());
    }

    #[test]
    fn stem_rules() {
        assert_eq!(stem_relation(Stem::Jia, Stem::Ji), StemRelation::FiveCombination);
        assert_eq!(stem_relation(Stem::Jia, Stem::Geng), StemRelation::Clash);
        assert_eq!(stem_relation(Stem::Ding, Stem::Ren), StemRelation::FiveCombination);
    }
}
