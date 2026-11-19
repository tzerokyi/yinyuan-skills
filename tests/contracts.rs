use std::process::Command;

use yinyuan_core::bazi::{compare, ChartInput, Pillar};
use yinyuan_core::fortune::{draw_number, lookup, CivilDate};
use yinyuan_core::redline;
use yinyuan_core::taohua;
use yinyuan_core::ziwei;
use yinyuan_core::zodiac::{compatibility, Animal, Relation};
use yinyuan_core::Branch;

#[test]
fn zodiac_contract() {
    assert_eq!(compatibility(Animal::Rat, Animal::Ox), Relation::SixHarmony);
    assert_eq!(compatibility(Animal::Rat, Animal::Horse), Relation::Clash);
}

#[test]
fn bazi_contract() {
    let left = ChartInput { year: Pillar::parse("甲子").unwrap(), day: Some(Pillar::parse("丙午").unwrap()) };
    let right = ChartInput { year: Pillar::parse("乙丑").unwrap(), day: Some(Pillar::parse("辛卯").unwrap()) };
    // 辛卯 has the correct yin-yang parity; 丙辛 is a five-combination.
    let report = compare(left, right);
    assert_eq!(report.year_branches, Relation::SixHarmony);
    assert!(!report.partial);
    assert!(report.year_nayin_same);
}

#[test]
fn fortune_contract() {
    let date = CivilDate::new(2026, 11, 19).unwrap();
    let number = draw_number("某人", "缘分", date, 5).unwrap();
    assert!((1..=100).contains(&number));
    assert!(!lookup(number).unwrap().poem.is_empty());
}

#[test]
fn ziwei_peach_redline_contracts() {
    assert!(!ziwei::main_star("天府").unwrap().partner_traits.is_empty());
    assert_eq!(taohua::position(Branch::Wu).branch, Branch::Mao);
    let report = redline::report(Animal::Horse, Branch::Zi, Some("天府")).unwrap();
    assert!(report.predicted_window.is_none());
}

#[test]
fn cli_json_and_error_contracts() {
    let bin = env!("CARGO_BIN_EXE_yinyuan");
    let output = Command::new(bin)
        .args(["zodiac", "--a", "rat", "--b", "ox", "--json"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.starts_with('{'));
    assert!(stdout.contains("\"relation\":\"six_harmony\""));
    assert!(stdout.contains("\"disclaimer\":"));
    let invalid = Command::new(bin)
        .args(["ziwei", "--star", "unknown", "--json"])
        .output()
        .unwrap();
    assert_eq!(invalid.status.code(), Some(2));
    assert!(invalid.stdout.is_empty());
    assert!(String::from_utf8(invalid.stderr).unwrap().contains("\"error\""));
}
