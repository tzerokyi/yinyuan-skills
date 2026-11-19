//! Combined folklore report from explicitly supplied normalized inputs.

use crate::taohua::{partner_profile, position, PartnerProfile, PeachPosition};
use crate::ziwei::{main_star, StarReading};
use crate::zodiac::Animal;
use crate::{Branch, Error};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedlineReport {
    pub animal: Animal,
    pub day_branch: Branch,
    pub profile: PartnerProfile,
    pub peach_position: PeachPosition,
    pub ziwei: Option<StarReading>,
    /// No future year/window is invented without a complete chart.
    pub predicted_window: Option<(i32, i32)>,
}

pub fn report(
    animal: Animal,
    day_branch: Branch,
    palace_star: Option<&str>,
) -> Result<RedlineReport, Error> {
    Ok(RedlineReport {
        animal,
        day_branch,
        profile: partner_profile(day_branch),
        peach_position: position(animal.branch()),
        ziwei: palace_star.map(main_star).transpose()?,
        predicted_window: None,
    })
}
