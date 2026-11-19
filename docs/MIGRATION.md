# Skill → Rust core mapping (v0.1)

Pinned source: original `SKILL.md` and `references/*.md` on the fork, as of the initial native-core branch.
The original markdown remains intact as a versioned data source and cultural-context reference.

| Mode | Rust API | CLI | Parity / limitation |
| --- | --- | --- | --- |
| Zodiac | `zodiac::compatibility` | `zodiac` | Exact six harmony / three harmony / clash / harm table; punishment/break priority explicitly resolved. Scores only for 95/85/50/60 cases present in source. |
| Bazi | `bazi::compare`, `Pillar::parse`, `Pillar::nayin` | `bazi` | Year/day relationship and 60-Jiazi Nayin rules; no birthtime→calendar engine, full eight-character balance or fabricated weighted score. |
| Ziwei | `ziwei::main_star`, `ziwei::palace` | `ziwei` | Exact 14 source star descriptions from embedded reference; externally calculated marriage palace required. Aux stars, transformations, transits not computed. |
| Fortune | `fortune::draw_number`, `fortune::lookup` | `fortune` | Full 100 verse corpus parsed directly from reference; deterministic inputs. Gregorian ordinal and shichen are explicitly supplied, not read from ambient clock. |
| Taohua | `taohua::report`, `position`, `month_hints` | `taohua` | Birth-branch direction and source month labels; annual scores only for explicitly tabulated 2026. Several original month-label inconsistencies retained verbatim, not treated as Gregorian dates. |
| Redline | `redline::report` | `redline` | Composes day-branch profile + peach position + optional Ziwei star; no ungrounded future year prediction. |

## Architectural boundary

No UI, Python, JavaScript, WebView, network, daemon or external runtime. The library does not invoke the CLI; CLI calls the typed library. Inputs are normalized precomputed charts where required. `--json` is stable machine output with a schema version from `doctor`; output values are currently string fields. stdout contains results only; stderr contains diagnostics; invalid inputs exit 2. No MCP transport yet; it can wrap the same library APIs later.

## Source quality / rights gate

Original repository has no detected `LICENSE` file. This fork retains the original reference texts; do not publish a package containing them until license/provenance and third-party verse attribution have been checked. `publish = false` is deliberate. This is crate-ready separation, **not** a claimed crates.io release.

The original skill contains unsupported interpretation/scoring promises and conflicting lunar month labels. Exact astronomical and natal-chart generation must be implemented by a verified calendar/pillar/chart dependency or a separately verified Rust engine, with independent fixtures, before being claimed.
