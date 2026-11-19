//! Peach-blossom direction/month reference lookup, without predictive claims.

use crate::zodiac::Animal;
use crate::Branch;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    East,
    West,
    South,
    North,
}

impl Direction {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::East => "east", Self::West => "west",
            Self::South => "south", Self::North => "north",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PeachPosition {
    pub branch: Branch,
    pub direction: Direction,
}

pub const fn position(birth_branch: Branch) -> PeachPosition {
    match birth_branch {
        Branch::Yin | Branch::Wu | Branch::Xu => PeachPosition { branch: Branch::Mao, direction: Direction::East },
        Branch::Shen | Branch::Zi | Branch::Chen => PeachPosition { branch: Branch::You, direction: Direction::West },
        Branch::Si | Branch::You | Branch::Chou => PeachPosition { branch: Branch::Wu, direction: Direction::South },
        Branch::Hai | Branch::Mao | Branch::Wei => PeachPosition { branch: Branch::Zi, direction: Direction::North },
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MonthHints {
    /// Original labels are lunar/branch names, NOT Gregorian month numbers.
    /// Several source rows conflict internally; retained verbatim for audit.
    pub favorable: [&'static str; 3],
    pub unfavorable: &'static str,
}

pub const fn month_hints(animal: Animal) -> MonthHints {
    match animal {
        Animal::Rat => MonthHints { favorable: ["七月（申月）", "八月（酉月）", "十二月（丑月）"], unfavorable: "五月（午月）" },
        Animal::Ox => MonthHints { favorable: ["四月（巳月）", "八月（酉月）", "十二月（子月）"], unfavorable: "六月（未月）" },
        Animal::Tiger => MonthHints { favorable: ["五月（午月）", "九月（戌月）", "十月（亥月）"], unfavorable: "七月（申月）" },
        Animal::Rabbit => MonthHints { favorable: ["二月（卯月）", "六月（未月）", "十月（亥月）"], unfavorable: "八月（酉月）" },
        Animal::Dragon => MonthHints { favorable: ["四月（巳月）", "八月（申月）", "十一月（子月）"], unfavorable: "九月（戌月）" },
        Animal::Snake => MonthHints { favorable: ["三月（辰月）", "七月（申月）", "十一月（子月）"], unfavorable: "十月（亥月）" },
        Animal::Horse => MonthHints { favorable: ["正月（寅月）", "六月（未月）", "十月（戌月）"], unfavorable: "十一月（子月）" },
        Animal::Goat => MonthHints { favorable: ["二月（卯月）", "七月（午月）", "十一月（亥月）"], unfavorable: "十二月（丑月）" },
        Animal::Monkey => MonthHints { favorable: ["四月（巳月）", "八月（酉月）", "十一月（子月）"], unfavorable: "正月（寅月）" },
        Animal::Rooster => MonthHints { favorable: ["三月（辰月）", "九月（丑月）", "十二月（丑月）"], unfavorable: "二月（卯月）" },
        Animal::Dog => MonthHints { favorable: ["正月（寅月）", "五月（午月）", "九月（卯月）"], unfavorable: "三月（辰月）" },
        Animal::Pig => MonthHints { favorable: ["二月（卯月）", "六月（未月）", "十月（寅月）"], unfavorable: "四月（巳月）" },
    }
}

/// Only the explicitly documented 2026 ratings are returned. Other years do
/// not inherit a fabricated extrapolation.
pub const fn reference_annual_rating(animal: Animal, year: i32) -> Option<u8> {
    if year != 2026 {
        return None;
    }
    Some(match animal {
        Animal::Rat => 3, Animal::Ox => 4, Animal::Tiger => 3,
        Animal::Rabbit => 3, Animal::Dragon => 4, Animal::Snake => 3,
        Animal::Horse => 2, Animal::Goat => 5, Animal::Monkey => 4,
        Animal::Rooster => 3, Animal::Dog => 3, Animal::Pig => 3,
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PeachReport {
    pub animal: Animal,
    pub position: PeachPosition,
    pub months: MonthHints,
    pub annual_rating: Option<u8>,
}

pub const fn report(animal: Animal, year: i32) -> PeachReport {
    PeachReport {
        animal,
        position: position(animal.branch()),
        months: month_hints(animal),
        annual_rating: reference_annual_rating(animal, year),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PartnerProfile {
    pub personality: &'static str,
    pub appearance: &'static str,
    pub vocation: &'static str,
}

/// A folklore-style profile indexed by an *already determined* day branch.
pub const fn partner_profile(day_branch: Branch) -> PartnerProfile {
    let (personality, appearance, vocation) = match day_branch {
        Branch::Zi => ("聪明灵活，善沟通", "清秀灵动", "教育/IT/媒体"),
        Branch::Chou => ("踏实稳重，可靠", "稳重端庄", "金融/工程/管理"),
        Branch::Yin => ("热情大方，有魄力", "高大挺拔", "创业/领导/军警"),
        Branch::Mao => ("温柔文雅，有艺术感", "清秀优雅", "艺术/设计/文化"),
        Branch::Chen => ("有主见，果断", "气质出众", "管理/法律/政界"),
        Branch::Si => ("深沉内敛，观察力强", "精致干练", "研究/医疗/金融"),
        Branch::Wu => ("热情开朗，行动力强", "阳光健康", "销售/体育/创业"),
        Branch::Wei => ("温和善良，重视家庭", "柔和可亲", "教育/服务/农业"),
        Branch::Shen => ("机智灵活，幽默风趣", "精神干练", "商业/科技/传媒"),
        Branch::You => ("注重细节，审美高", "五官精致", "艺术/法律/金融"),
        Branch::Xu => ("忠诚可靠，有责任感", "稳健大方", "军警/工程/管理"),
        Branch::Hai => ("善良包容，有智慧", "随和亲和", "学术/文化/公益"),
    };
    PartnerProfile { personality, appearance, vocation }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_groups_are_exhaustive() {
        assert_eq!(position(Branch::Zi).branch, Branch::You);
        assert_eq!(position(Branch::Wu).branch, Branch::Mao);
        assert_eq!(position(Branch::Chou).direction, Direction::South);
        assert_eq!(position(Branch::Hai).direction, Direction::North);
        assert_eq!(reference_annual_rating(Animal::Goat, 2026), Some(5));
        assert_eq!(reference_annual_rating(Animal::Goat, 2027), None);
    }
}
