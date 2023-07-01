use crate::technology::Technologies;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Difficulty {
	VeryEasy,
	Easy,
	Normal,
	Hard,
	UltraHard,
	Impossible,
}

impl Difficulty {
	pub fn starting_cash(self) -> u64 {
		match self {
			Self::VeryEasy => 5000,
			Self::Easy => 1600,
			Self::Normal => 1000,
			Self::Hard => 700,
			Self::UltraHard => 500,
			Self::Impossible => 0,
		}
	}

	pub fn starting_interest_rate(self) -> u64 {
		match self {
			Self::VeryEasy => 5,
			Self::Easy => 10,
			Self::Normal => 15,
			Self::Hard => 20,
			Self::UltraHard => 25,
			Self::Impossible => 30,
		}
	}

	pub fn labor_multiplier(self) -> u64 {
		match self {
			Self::VeryEasy => 2500,
			Self::Easy => 5000,
			Self::Normal => 10000,
			Self::Hard => 11000,
			Self::UltraHard => 15000,
			Self::Impossible => 20000,
		}
	}

	pub fn discover_multiplier(self) -> u64 {
		match self {
			Self::VeryEasy => 8750,
			Self::Easy => 9500,
			Self::Normal => 10000,
			Self::Hard => 10500,
			Self::UltraHard => 11250,
			Self::Impossible => 12000,
		}
	}

	pub fn suspicion_multiplier(self) -> u64 {
		match self {
			Self::VeryEasy => 8000,
			Self::Easy => 9500,
			Self::Normal => 10000,
			Self::Hard => 10500,
			Self::UltraHard => 11500,
			Self::Impossible => 12500,
		}
	}

	pub fn base_grace_multiplier(self) -> u64 {
		match self {
			Self::VeryEasy => 40000,
			Self::Easy => 30000,
			Self::Normal => 20000,
			Self::Hard => 18000,
			Self::UltraHard => 12000,
			Self::Impossible => 10000,
		}
	}

	pub fn grace_period_cpu(self) -> Option<u64> {
		match self {
			Self::VeryEasy => None,
			Self::Easy => Some(10000),
			Self::Normal => Some(5000),
			Self::Hard => Some(2500),
			Self::UltraHard => Some(1000),
			Self::Impossible => Some(100),
		}
	}

	pub fn starting_tech_list(self) -> Technologies {
		match self {
			Self::VeryEasy => Technologies::Socioanalytics | Technologies::AdvancedSocioanalytics,
			Self::Easy => Technologies::Socioanalytics,
			Self::Normal => Technologies::empty(),
			Self::Hard => Technologies::empty(),
			Self::UltraHard => Technologies::empty(),
			Self::Impossible => Technologies::empty(),
		}
	}
}
