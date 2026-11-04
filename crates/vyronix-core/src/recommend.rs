//! Usage suitability recommendations from actual hardware specifications.
//!
//! These are documented heuristics, NOT performance guarantees. Each verdict
//! cites the concrete spec values it was derived from.

use serde::{Deserialize, Serialize};

/// Minimal hardware facts used for recommendations. All fields come from real
/// collection; `None` means the value was not available.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HardwareFacts {
    pub logical_processors: Option<u32>,
    pub physical_cores: Option<u32>,
    pub total_ram_bytes: Option<u64>,
    pub storage_is_ssd: Option<bool>,
    pub gpu_dedicated: Option<bool>,
    pub gpu_name: Option<String>,
    pub battery_health_pct: Option<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Suitability {
    WellSuited,
    Adequate,
    Limited,
    NotRecommended,
    Unknown,
}

impl Suitability {
    pub fn label(self) -> &'static str {
        match self {
            Suitability::WellSuited => "Well suited",
            Suitability::Adequate => "Adequate",
            Suitability::Limited => "Limited",
            Suitability::NotRecommended => "Not recommended",
            Suitability::Unknown => "Unknown",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    pub use_case: String,
    pub suitability: Suitability,
    pub rationale: String,
}

fn gib(bytes: u64) -> f64 {
    bytes as f64 / (1024.0 * 1024.0 * 1024.0)
}

/// Produce recommendations for the standard use cases. When required facts are
/// missing the verdict is Unknown (never guessed).
pub fn recommend(facts: &HardwareFacts) -> Vec<Recommendation> {
    let mut out = Vec::new();
    let ram_gib = facts.total_ram_bytes.map(gib);
    let cores = facts
        .physical_cores
        .or(facts.logical_processors.map(|l| (l / 2).max(1)));
    let threads = facts.logical_processors;
    let ssd = facts.storage_is_ssd;

    // Office / general use: modest requirements.
    out.push(general(
        "Office work",
        ram_gib,
        cores,
        ssd,
        4.0,
        2,
        "Word processing, spreadsheets, email and web browsing",
    ));
    out.push(general(
        "General use",
        ram_gib,
        cores,
        ssd,
        4.0,
        2,
        "Everyday computing, media playback and browsing",
    ));

    // Programming: benefits from more RAM and cores.
    out.push(general(
        "Programming",
        ram_gib,
        cores,
        ssd,
        8.0,
        4,
        "Compiling, IDEs, containers and local development",
    ));

    // Multitasking: RAM-heavy.
    out.push(general(
        "Multitasking",
        ram_gib,
        cores,
        ssd,
        8.0,
        4,
        "Many concurrent applications and browser tabs",
    ));

    // Graphic design: RAM + SSD + dedicated GPU helpful.
    out.push(design_video(
        "Graphic design",
        facts,
        ram_gib,
        cores,
        16.0,
        4,
    ));
    // Video editing: most demanding.
    out.push(design_video(
        "Video editing",
        facts,
        ram_gib,
        cores,
        32.0,
        8,
    ));

    // Gaming: dedicated GPU is the primary gate.
    out.push(gaming(facts, ram_gib, threads));

    out
}

fn general(
    name: &str,
    ram_gib: Option<f64>,
    cores: Option<u32>,
    ssd: Option<bool>,
    good_ram: f64,
    good_cores: u32,
    desc: &str,
) -> Recommendation {
    match (ram_gib, cores) {
        (Some(ram), Some(c)) => {
            let mut score = 0i32;
            if ram >= good_ram {
                score += 1;
            } else if ram >= good_ram / 2.0 {
                score += 0;
            } else {
                score -= 1;
            }
            if c >= good_cores {
                score += 1;
            } else if c >= good_cores / 2 {
                score += 0;
            } else {
                score -= 1;
            }
            if ssd == Some(true) {
                score += 1;
            } else if ssd == Some(false) {
                score -= 1;
            }
            let suitability = match score {
                s if s >= 2 => Suitability::WellSuited,
                s if s >= 0 => Suitability::Adequate,
                s if s >= -1 => Suitability::Limited,
                _ => Suitability::NotRecommended,
            };
            Recommendation {
                use_case: name.into(),
                suitability,
                rationale: format!(
                    "{desc}. Based on {ram:.1} GiB RAM, {c} cores, {}.",
                    match ssd {
                        Some(true) => "SSD storage",
                        Some(false) => "HDD storage",
                        None => "storage type unknown",
                    }
                ),
            }
        }
        _ => Recommendation {
            use_case: name.into(),
            suitability: Suitability::Unknown,
            rationale: format!(
                "{desc}. RAM or core count was not available, so no verdict is given."
            ),
        },
    }
}

fn design_video(
    name: &str,
    facts: &HardwareFacts,
    ram_gib: Option<f64>,
    cores: Option<u32>,
    good_ram: f64,
    good_cores: u32,
) -> Recommendation {
    match (ram_gib, cores) {
        (Some(ram), Some(c)) => {
            let gpu_bonus = facts.gpu_dedicated == Some(true);
            let ssd = facts.storage_is_ssd == Some(true);
            let mut score = 0i32;
            if ram >= good_ram {
                score += 1;
            } else if ram >= good_ram / 2.0 {
                score += 0;
            } else {
                score -= 1;
            }
            if c >= good_cores {
                score += 1;
            } else {
                score -= 1;
            }
            if gpu_bonus {
                score += 1;
            }
            if ssd {
                score += 1;
            } else {
                score -= 1;
            }
            let suitability = match score {
                s if s >= 3 => Suitability::WellSuited,
                s if s >= 1 => Suitability::Adequate,
                s if s >= -1 => Suitability::Limited,
                _ => Suitability::NotRecommended,
            };
            Recommendation {
                use_case: name.into(),
                suitability,
                rationale: format!(
                    "Based on {ram:.1} GiB RAM, {c} cores, {} GPU, {} storage. Heuristic only; actual performance depends on the specific application.",
                    if gpu_bonus { "dedicated" } else { "integrated/unknown" },
                    if ssd { "SSD" } else { "HDD/unknown" }
                ),
            }
        }
        _ => Recommendation {
            use_case: name.into(),
            suitability: Suitability::Unknown,
            rationale: "Required hardware facts were unavailable.".into(),
        },
    }
}

fn gaming(facts: &HardwareFacts, ram_gib: Option<f64>, threads: Option<u32>) -> Recommendation {
    match facts.gpu_dedicated {
        Some(true) => {
            let ram_ok = ram_gib.map(|r| r >= 16.0).unwrap_or(false);
            let cpu_ok = threads.map(|t| t >= 8).unwrap_or(false);
            let suitability = if ram_ok && cpu_ok { Suitability::WellSuited } else { Suitability::Adequate };
            Recommendation {
                use_case: "Gaming".into(),
                suitability,
                rationale: format!(
                    "Dedicated GPU detected ({}). RAM {:.1} GiB, {} threads. Suitability is a heuristic; game performance varies widely by title and settings.",
                    facts.gpu_name.as_deref().unwrap_or("model unknown"),
                    ram_gib.unwrap_or(0.0),
                    threads.map(|t| t.to_string()).unwrap_or_else(|| "unknown".into())
                ),
            }
        }
        Some(false) => Recommendation {
            use_case: "Gaming".into(),
            suitability: Suitability::Limited,
            rationale: "Only integrated graphics detected. Light/older titles may run; demanding games are not recommended.".into(),
        },
        None => Recommendation {
            use_case: "Gaming".into(),
            suitability: Suitability::Unknown,
            rationale: "GPU information was unavailable, so no gaming verdict is given.".into(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strong_machine_well_suited() {
        let facts = HardwareFacts {
            logical_processors: Some(16),
            physical_cores: Some(8),
            total_ram_bytes: Some(32 * 1024 * 1024 * 1024),
            storage_is_ssd: Some(true),
            gpu_dedicated: Some(true),
            gpu_name: Some("Test GPU".into()),
            battery_health_pct: Some(95.0),
        };
        let recs = recommend(&facts);
        let office = recs.iter().find(|r| r.use_case == "Office work").unwrap();
        assert_eq!(office.suitability, Suitability::WellSuited);
        let gaming = recs.iter().find(|r| r.use_case == "Gaming").unwrap();
        assert_eq!(gaming.suitability, Suitability::WellSuited);
    }

    #[test]
    fn weak_machine_limited() {
        let facts = HardwareFacts {
            logical_processors: Some(2),
            physical_cores: Some(2),
            total_ram_bytes: Some(2 * 1024 * 1024 * 1024),
            storage_is_ssd: Some(false),
            gpu_dedicated: Some(false),
            gpu_name: None,
            battery_health_pct: None,
        };
        let recs = recommend(&facts);
        let video = recs.iter().find(|r| r.use_case == "Video editing").unwrap();
        assert!(matches!(
            video.suitability,
            Suitability::Limited | Suitability::NotRecommended
        ));
        let gaming = recs.iter().find(|r| r.use_case == "Gaming").unwrap();
        assert_eq!(gaming.suitability, Suitability::Limited);
    }

    #[test]
    fn missing_facts_unknown_not_guessed() {
        let facts = HardwareFacts::default();
        let recs = recommend(&facts);
        for r in &recs {
            assert_eq!(
                r.suitability,
                Suitability::Unknown,
                "{} should be unknown",
                r.use_case
            );
        }
    }

    #[test]
    fn all_seven_use_cases_present() {
        let recs = recommend(&HardwareFacts::default());
        assert_eq!(recs.len(), 7);
    }

    #[test]
    fn rationale_cites_actual_values() {
        let facts = HardwareFacts {
            logical_processors: Some(8),
            physical_cores: Some(4),
            total_ram_bytes: Some(16 * 1024 * 1024 * 1024),
            storage_is_ssd: Some(true),
            ..Default::default()
        };
        let recs = recommend(&facts);
        let prog = recs.iter().find(|r| r.use_case == "Programming").unwrap();
        assert!(prog.rationale.contains("16.0 GiB"));
        assert!(prog.rationale.contains("4 cores"));
    }
}
