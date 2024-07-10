use rand::distributions::{Distribution, Standard};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum BetKind {
    Fish,
    Prawn,
    Crab,
    Cock,
    Calabash,
    Tiger,
}

impl Distribution<BetKind> for Standard {
    fn sample<R: rand::prelude::Rng + ?Sized>(&self, rng: &mut R) -> BetKind {
        match rng.gen_range(0..=5) {
            0 => BetKind::Fish,
            1 => BetKind::Prawn,
            2 => BetKind::Crab,
            3 => BetKind::Cock,
            4 => BetKind::Calabash,
            _ => BetKind::Tiger,
        }
    }
}

impl Into<String> for BetKind {
    fn into(self) -> String {
        match self {
            BetKind::Fish => "Fish".to_string(),
            BetKind::Prawn => "Prawn".to_string(),
            BetKind::Crab => "Crab".to_string(),
            BetKind::Cock => "Cock".to_string(),
            BetKind::Calabash => "Calabash".to_string(),
            BetKind::Tiger => "Tiger".to_string(),
        }
    }
}

impl BetKind {
    pub fn random() -> BetKind {
        rand::random()
    }
}
