//! Thin std-only CLI adapter over yinyuan_core.

use std::collections::BTreeMap;
use std::fmt::Write as _;

use yinyuan_core::bazi::{self, ChartInput, Pillar};
use yinyuan_core::fortune::{self, CivilDate};
use yinyuan_core::redline;
use yinyuan_core::taohua;
use yinyuan_core::ziwei;
use yinyuan_core::zodiac::{self, Animal};
use yinyuan_core::{Branch, DISCLAIMER};

const HELP: &str = "yinyuan - native Rust cultural-divination reference CLI

Usage: yinyuan <command> [options] [--json]

Commands:
  doctor                                               Show capabilities
  zodiac  --a RAT --b OX                               Zodiac compatibility
          [--year-a YYYY --year-b YYYY]                Nominal-year alternative
  bazi    --year-a 甲子 --year-b 乙丑                    Pillar-based comparison
          [--day-a 丙午 --day-b 辛卯]                    Optional day pillars
  ziwei   --star 天府 [--star 太阴]                      Existing chart's stars
  fortune --name NAME --question TEXT                   Deterministic draw
          --date YYYY-MM-DD --hour-branch 0..11
  taohua  --animal HORSE [--year 2026]                 Direction and month hints
  redline --animal HORSE --day-branch 午 [--star 天府]   Combined reference

Use --json for machine-readable stdout. Errors go to stderr (exit 2).
Dates, charts and zodiac boundary cutoffs are NOT computed from birth times.
All modes are traditional-cultural entertainment, not factual predictions.
";

#[derive(Default)]
struct Args {
    command: Option<String>,
    flags: BTreeMap<String, Vec<String>>,
    json: bool,
}

impl Args {
    fn parse() -> Result<Self, String> {
        let mut args = Self::default();
        let mut tokens = std::env::args().skip(1);
        while let Some(token) = tokens.next() {
            match token.as_str() {
                "--json" => args.json = true,
                "--version" | "-V" => args.command = Some("version".to_owned()),
                "--help" | "-h" => args.command = Some("help".to_owned()),
                value if value.starts_with("--") => {
                    let next = tokens.next().ok_or_else(|| format!("missing value for {value}"))?;
                    if next.starts_with("--") {
                        return Err(format!("missing value for {value}"));
                    }
                    args.flags.entry(value.to_owned()).or_default().push(next);
                }
                value => {
                    if args.command.is_some() {
                        return Err(format!("unexpected argument: {value}"));
                    }
                    args.command = Some(value.to_owned());
                }
            }
        }
        Ok(args)
    }

    fn one(&self, key: &str) -> Result<&str, String> {
        match self.flags.get(key).map(Vec::as_slice) {
            Some([value]) => Ok(value),
            Some(_) => Err(format!("{key} must appear exactly once")),
            None => Err(format!("missing required {key}")),
        }
    }

    fn optional(&self, key: &str) -> Result<Option<&str>, String> {
        match self.flags.get(key).map(Vec::as_slice) {
            Some([value]) => Ok(Some(value)),
            Some(_) => Err(format!("{key} must appear at most once")),
            None => Ok(None),
        }
    }

    fn validate(&self, allowed: &[&str]) -> Result<(), String> {
        if let Some(key) = self.flags.keys().find(|key| !allowed.contains(&key.as_str())) {
            return Err(format!("unknown option: {key}"));
        }
        Ok(())
    }
}

fn json_quote(input: &str) -> String {
    let mut result = String::from("\"");
    for ch in input.chars() {
        match ch {
            '"' => result.push_str("\\\""),
            '\\' => result.push_str("\\\\"),
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\t' => result.push_str("\\t"),
            c if c.is_control() => {
                let _ = write!(result, "\\u{:04x}", c as u32);
            }
            c => result.push(c),
        }
    }
    result.push('"');
    result
}

struct Output {
    label: &'static str,
    fields: Vec<(&'static str, String)>,
}

impl Output {
    fn new(label: &'static str) -> Self {
        Self { label, fields: Vec::new() }
    }

    fn add(&mut self, key: &'static str, value: impl ToString) {
        self.fields.push((key, value.to_string()));
    }

    fn print(self, json: bool) {
        if json {
            let mut fields = vec![("mode", json_quote(self.label))];
            fields.extend(self.fields.iter().map(|(key, value)| (*key, json_quote(value))));
            fields.push(("disclaimer", json_quote(DISCLAIMER)));
            let text = fields.into_iter().map(|(key, value)| format!("{}:{value}", json_quote(key))).collect::<Vec<_>>().join(",");
            println!("{{{text}}}");
        } else {
            println!("{}", self.label);
            for (key, value) in self.fields {
                println!("{key}: {value}");
            }
            println!("\n{DISCLAIMER}");
        }
    }
}

fn animal(args: &Args, key: &str, year_key: &str) -> Result<Animal, String> {
    let name = args.optional(key)?;
    let year = args.optional(year_key)?;
    match (name, year) {
        (Some(name), None) => Animal::parse(name).ok_or_else(|| format!("unknown animal: {name}")),
        (None, Some(year)) => {
            let number: i32 = year.parse().map_err(|_| format!("invalid nominal year: {year}"))?;
            Animal::from_nominal_year(number).map_err(|err| err.to_string())
        }
        (Some(_), Some(_)) => Err(format!("choose either {key} or {year_key}, not both")),
        (None, None) => Err(format!("missing {key} (or {year_key})")),
    }
}

fn run(args: &Args) -> Result<Output, String> {
    match args.command.as_deref().unwrap_or("help") {
        "doctor" => {
            args.validate(&[])?;
            let mut output = Output::new("doctor");
            output.add("engine", "native-rust");
            output.add("corpus", "100 fortune sticks; zodiac; bazi; ziwei; taohua; redline");
            output.add("birth_chart_generation", "unsupported: requires external calendrical engine");
            output.add("schema_version", "1");
            Ok(output)
        }
        "zodiac" => {
            args.validate(&["--a", "--b", "--year-a", "--year-b"])?;
            let a = animal(args, "--a", "--year-a")?;
            let b = animal(args, "--b", "--year-b")?;
            let relation = zodiac::compatibility(a, b);
            let mut output = Output::new("zodiac");
            output.add("a", a.as_str());
            output.add("b", b.as_str());
            output.add("relation", relation.as_str());
            output.add("reference_score", relation.reference_score().map_or_else(|| "null".to_owned(), |score| score.to_string()));
            output.add("nominal_year_warning", "year-label mapping has no lunar/solar cutoff");
            Ok(output)
        }
        "bazi" => {
            args.validate(&["--year-a", "--year-b", "--day-a", "--day-b"])?;
            let year_a = Pillar::parse(args.one("--year-a")?).map_err(|err| err.to_string())?;
            let year_b = Pillar::parse(args.one("--year-b")?).map_err(|err| err.to_string())?;
            let day_a = args.optional("--day-a")?.map(Pillar::parse).transpose().map_err(|err| err.to_string())?;
            let day_b = args.optional("--day-b")?.map(Pillar::parse).transpose().map_err(|err| err.to_string())?;
            if day_a.is_some() != day_b.is_some() {
                return Err("day pillars must be supplied for both people".to_owned());
            }
            let result = bazi::compare(ChartInput { year: year_a, day: day_a }, ChartInput { year: year_b, day: day_b });
            let mut output = Output::new("bazi");
            output.add("year_stems", result.year_stems.as_str());
            output.add("year_branches", result.year_branches.as_str());
            output.add("nayin_a", result.year_nayin_a);
            output.add("nayin_b", result.year_nayin_b);
            output.add("nayin_same", result.year_nayin_same);
            output.add("day_stems", result.day_stems.map_or("not_supplied", |value| value.as_str()));
            output.add("day_branches", result.day_branches.map_or("not_supplied", |value| value.as_str()));
            output.add("day_elements", result.day_elements.map_or("not_supplied", |value| value.as_str()));
            output.add("partial", result.partial);
            output.add("composite_score", "not_defined_by_source");
            Ok(output)
        }
        "fortune" => {
            args.validate(&["--name", "--question", "--date", "--hour-branch"])?;
            let name = args.one("--name")?;
            let question = args.one("--question")?;
            let date = CivilDate::parse(args.one("--date")?).map_err(|err| err.to_string())?;
            let hour = args.one("--hour-branch")?.parse::<u8>().map_err(|_| "invalid hour branch".to_owned())?;
            let number = fortune::draw_number(name, question, date, hour).map_err(|err| err.to_string())?;
            let stick = fortune::lookup(number).map_err(|err| err.to_string())?;
            let mut output = Output::new("fortune");
            output.add("number", stick.number);
            output.add("grade", stick.grade.as_str());
            output.add("poem", stick.poem);
            output.add("explanation", stick.explanation.unwrap_or_default());
            output.add("relationship_note", stick.relationship_note.unwrap_or_default());
            output.add("date_rule", "Gregorian ordinal day; traditional hour branch index (Zi=0)");
            Ok(output)
        }
        "taohua" => {
            args.validate(&["--animal", "--year", "--birth-year"])?;
            let animal = animal(args, "--animal", "--birth-year")?;
            let year = args.optional("--year")?.unwrap_or("2026").parse::<i32>().map_err(|_| "invalid year".to_owned())?;
            let result = taohua::report(animal, year);
            let mut output = Output::new("taohua");
            output.add("animal", animal.as_str());
            output.add("peach_branch", result.position.branch.hanzi());
            output.add("direction", result.position.direction.as_str());
            output.add("favorable_months", result.months.favorable.join("; "));
            output.add("unfavorable_month", result.months.unfavorable);
            output.add("annual_rating", result.annual_rating.map_or_else(|| "not_available".to_owned(), |n| n.to_string()));
            output.add("month_labels", "source lunar/branch labels; not Gregorian months");
            Ok(output)
        }
        "ziwei" => {
            args.validate(&["--star"])?;
            let names = args.flags.get("--star").ok_or("provide at least one --star".to_owned())?;
            let values = names.iter().map(String::as_str).collect::<Vec<_>>();
            let readings = ziwei::palace(&values).map_err(|err| err.to_string())?;
            let mut output = Output::new("ziwei");
            output.add("stars", readings.iter().map(|r| r.star.as_str()).collect::<Vec<_>>().join(", "));
            output.add("partner_traits", readings.iter().map(|r| r.partner_traits.as_str()).collect::<Vec<_>>().join("; "));
            output.add("relationship_style", readings.iter().map(|r| r.relationship_style.as_str()).collect::<Vec<_>>().join("; "));
            output.add("challenge", readings.iter().map(|r| r.challenge.as_str()).collect::<Vec<_>>().join("; "));
            output.add("advice", readings.iter().map(|r| r.advice.as_str()).collect::<Vec<_>>().join("; "));
            output.add("chart_calculated", false);
            Ok(output)
        }
        "redline" => {
            args.validate(&["--animal", "--birth-year", "--day-branch", "--star"])?;
            let animal = animal(args, "--animal", "--birth-year")?;
            let branch = Branch::parse(args.one("--day-branch")?).ok_or("invalid day branch".to_owned())?;
            let result = redline::report(animal, branch, args.optional("--star")?).map_err(|err| err.to_string())?;
            let mut output = Output::new("redline");
            output.add("animal", animal.as_str());
            output.add("day_branch", branch.hanzi());
            output.add("partner_personality", result.profile.personality);
            output.add("partner_appearance", result.profile.appearance);
            output.add("partner_vocation", result.profile.vocation);
            output.add("direction", result.peach_position.direction.as_str());
            output.add("ziwei_star", result.ziwei.as_ref().map_or("not_supplied", |r| r.star.as_str()));
            output.add("predicted_window", "not_computed");
            Ok(output)
        }
        other => Err(format!("unknown command: {other}")),
    }
}

fn main() {
    let args = match Args::parse() {
        Ok(args) => args,
        Err(error) => {
            eprintln!("error: {error}\nRun yinyuan --help for usage.");
            std::process::exit(2);
        }
    };
    match args.command.as_deref().unwrap_or("help") {
        "help" => {
            print!("{HELP}");
        }
        "version" => println!("yinyuan {}", env!("CARGO_PKG_VERSION")),
        _ => match run(&args) {
            Ok(output) => output.print(args.json),
            Err(error) => {
                if args.json {
                    eprintln!("{{\"error\":{}}}", json_quote(&error));
                } else {
                    eprintln!("error: {error}\nRun yinyuan --help for usage.");
                }
                std::process::exit(2);
            }
        },
    }
}
