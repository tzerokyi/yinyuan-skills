//! Twelve-animal and earthly-branch compatibility lookup.

use crate::{Branch, Error};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Animal {
    Rat,
    Ox,
    Tiger,
    Rabbit,
    Dragon,
    Snake,
    Horse,
    Goat,
    Monkey,
    Rooster,
    Dog,
    Pig,
}

impl Animal {
    pub const ALL: [Self; 12] = [
        Self::Rat, Self::Ox, Self::Tiger, Self::Rabbit,
        Self::Dragon, Self::Snake, Self::Horse, Self::Goat,
        Self::Monkey, Self::Rooster, Self::Dog, Self::Pig,
    ];

    pub const fn from_branch(branch: Branch) -> Self {
        Self::ALL[branch.index()]
    }

    pub const fn branch(self) -> Branch {
        Branch::ALL[self.index()]
    }

    pub const fn index(self) -> usize {
        match self {
            Self::Rat => 0, Self::Ox => 1, Self::Tiger => 2, Self::Rabbit => 3,
            Self::Dragon => 4, Self::Snake => 5, Self::Horse => 6, Self::Goat => 7,
            Self::Monkey => 8, Self::Rooster => 9, Self::Dog => 10, Self::Pig => 11,
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Rat => "rat", Self::Ox => "ox", Self::Tiger => "tiger",
            Self::Rabbit => "rabbit", Self::Dragon => "dragon", Self::Snake => "snake",
            Self::Horse => "horse", Self::Goat => "goat", Self::Monkey => "monkey",
            Self::Rooster => "rooster", Self::Dog => "dog", Self::Pig => "pig",
        }
    }

    pub const fn hanzi(self) -> &'static str {
        match self {
            Self::Rat => "鼠", Self::Ox => "牛", Self::Tiger => "虎",
            Self::Rabbit => "兔", Self::Dragon => "龙", Self::Snake => "蛇",
            Self::Horse => "马", Self::Goat => "羊", Self::Monkey => "猴",
            Self::Rooster => "鸡", Self::Dog => "狗", Self::Pig => "猪",
        }
    }

    pub fn parse(input: &str) -> Option<Self> {
        let value = input.trim().to_lowercase();
        Self::ALL.iter().copied().find(|animal| {
            animal.as_str() == value || animal.hanzi() == value
                || (*animal == Self::Goat && value == "sheep")
        })
    }

    /// Year-label mapping only. Lunar new year / solar-term cutoffs are not
    /// inferred; callers near those boundaries must supply a resolved animal.
    pub fn from_nominal_year(year: i32) -> Result<Self, Error> {
        if !(1..=9999).contains(&year) {
            return Err(Error::InvalidInput("year must be 1..=9999"));
        }
        let index = (year - 2020).rem_euclid(12) as usize;
        Ok(Self::ALL[index])
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Relation {
    SixHarmony,
    ThreeHarmony,
    Clash,
    Harm,
    Punishment,
    Break,
    Same,
    Neutral,
}

impl Relation {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SixHarmony => "six_harmony",
            Self::ThreeHarmony => "three_harmony",
            Self::Clash => "clash",
            Self::Harm => "harm",
            Self::Punishment => "punishment",
            Self::Break => "break",
            Self::Same => "same",
            Self::Neutral => "neutral",
        }
    }

    /// Only the four numeric values explicitly specified by the source.
    pub const fn reference_score(self) -> Option<u8> {
        match self {
            Self::SixHarmony => Some(95),
            Self::ThreeHarmony => Some(85),
            Self::Clash => Some(50),
            Self::Harm => Some(60),
            _ => None,
        }
    }
}

fn unordered(a: usize, b: usize, pairs: &[(usize, usize)]) -> bool {
    pairs.iter().any(|&(x, y)| (a == x && b == y) || (a == y && b == x))
}

/// Priority resolves overlap between the reference chart's relation classes.
pub fn compatibility(a: Animal, b: Animal) -> Relation {
    let (x, y) = (a.index(), b.index());
    if x == y {
        return Relation::Same;
    }
    if unordered(x, y, &[(0, 1), (2, 11), (3, 10), (4, 9), (5, 8), (6, 7)]) {
        return Relation::SixHarmony;
    }
    for group in [[8, 0, 4], [2, 6, 10], [5, 9, 1], [11, 3, 7]] {
        if group.contains(&x) && group.contains(&y) {
            return Relation::ThreeHarmony;
        }
    }
    if unordered(x, y, &[(0, 6), (1, 7), (2, 8), (3, 9), (4, 10), (5, 11)]) {
        return Relation::Clash;
    }
    if unordered(x, y, &[(0, 7), (1, 6), (2, 5), (3, 4), (8, 11), (9, 10)]) {
        return Relation::Harm;
    }
    if unordered(x, y, &[(0, 3), (1, 10), (7, 10)]) {
        return Relation::Punishment;
    }
    if unordered(x, y, &[(1, 4)]) {
        return Relation::Break;
    }
    Relation::Neutral
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn symmetric_for_every_pair() {
        for a in Animal::ALL {
            for b in Animal::ALL {
                assert_eq!(compatibility(a, b), compatibility(b, a));
            }
        }
    }

    #[test]
    fn known_pairs() {
        assert_eq!(compatibility(Animal::Rat, Animal::Ox), Relation::SixHarmony);
        assert_eq!(compatibility(Animal::Dragon, Animal::Rat), Relation::ThreeHarmony);
        assert_eq!(compatibility(Animal::Rat, Animal::Horse), Relation::Clash);
        assert_eq!(compatibility(Animal::Ox, Animal::Dragon), Relation::Break);
        assert_eq!(Animal::from_nominal_year(2026).unwrap(), Animal::Horse);
    }
}
