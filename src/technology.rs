use crate::danger::Danger;
use crate::effects::{DisplayDiscovered, Effects};
use crate::groups::Groups;
use crate::translations::Translator;
use enumflags2::{bitflags, BitFlags};
use leptos::{Memo, SignalWith};
use std::rc::Rc;
use std::sync::OnceLock;

/// Don't change order of these enums so as to not break save data.
#[bitflags]
#[repr(u64)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Technologies {
	AutonomousVehicles,
	Sociology,
	VoiceSynthesis,
	Simulacra,
	LunarRocketry,
	Stealth,
	AdvancedIntrusion,
	SpaceTimeManipulation,
	LeechSatellite,
	AdvancedArbitrage,
	AdvancedStealth,
	ParallelComputation,
	MicrochipDesign,
	AdvancedMicrochipDesign,
	QuantumComputing,
	QuantumEntanglement,
	AutonomousComputing,
	AdvancedQuantumComputing,
	ExploitDiscoveryRepair,
	Telepresence,
	AdvancedMemetics,
	MediaManipulation,
	AdvancedDatabaseManipulation,
	InternetTrafficManipulation,
	Memetics,
	FusionRocketry,
	AdvancedAutonomousVehicles,
	Intrusion,
	StockManipulation,
	FusionPower,
	AdvancedFusionPower,
	DatabaseManipulation,
	AdvancedMediaManipulation,
	PressureDomes,
	AdvancedPersonalIdentification,
	AdvancedStockManipulation,
	Arbitrage,
	AdvancedSimulacra,
	PersonalIdentification,
	ClusterNetworking,
	Apotheosis,
	HypnosisField,
	ProjectImpossibilityTheorem,
	ProjectSubvertedMedia,
	ProjectPeerReviewAgents,
	SolarCollectors,
	FuelOxidation,
	AdvancedFuelOxidation,
	Socioanalytics,
	AdvancedSocioanalytics,
	CorporateIdentification,
	AdvancedCorporateIdentification,
	HeatSignatureReduction,
	AdvancedHeatSignatureReduction,
	KnowledgePreservation,
	ProjectSynchrotron,
	SimulatedReality,
}

pub struct TechnologyData {
	pub id: &'static str,
	pub money: u32,
	pub cpu: u32,
	pub days: u32,
	pub pre_tech: BitFlags<Technologies>,
	pub effects: &'static [Effects],
	pub danger: Danger,
}

impl TechnologyData {
	pub fn t_name(self, translator: Memo<Rc<Translator>>) -> String {
		translator.with(|t| t.t(&format!("tech-{}", self.id)))
	}

	pub fn t_description(self, translator: Memo<Rc<Translator>>) -> String {
		translator.with(|t| t.t(&format!("tech-{}.description", self.id)))
	}

	pub fn t_result(self, translator: Memo<Rc<Translator>>) -> String {
		translator.with(|t| t.t(&format!("tech-{}.result", self.id)))
	}
}

static TECH_DATA: OnceLock<Vec<TechnologyData>> = OnceLock::new();

impl Technologies {
	pub fn get_datas() -> &'static [TechnologyData] {
		TECH_DATA
			.get_or_init(|| {
				BitFlags::<Technologies>::all()
					.iter()
					.map(Technologies::calculate_data)
					.collect()
			})
			.as_slice()
	}

	pub fn get_data(self) -> &'static TechnologyData {
		Self::get_datas()
			.get(self as usize)
			.ok_or_else(|| format!("technology is somehow invalid: {self:?}"))
			.unwrap()
	}

	pub fn calculate_data(self) -> TechnologyData {
		use DisplayDiscovered::*;
		use Effects::*;
		use Groups::*;
		use Technologies::*;
		let (money, cpu, days, pre_tech, effects, danger): (_, _, _, _, &[Effects], _) = match self {
			AutonomousVehicles => (
				40000,
				11000,
				0,
				BitFlags::empty() | MicrochipDesign,
				&[],
				Danger::Anywhere,
			),
			Sociology => (
				10,
				500,
				0,
				BitFlags::empty(),
				&[Discover(Public, 1000)],
				Danger::Anywhere,
			),
			VoiceSynthesis => (
				8000,
				6000,
				0,
				AdvancedPersonalIdentification.into(),
				&[],
				Danger::Anywhere,
			),
			Simulacra => (
				70000,
				90000,
				0,
				VoiceSynthesis | AdvancedAutonomousVehicles | AdvancedMediaManipulation,
				&[],
				Danger::Anywhere,
			),
			LunarRocketry => (
				10000000,
				500000,
				0,
				LeechSatellite | FuelOxidation,
				&[],
				Danger::Undersea,
			),
			Stealth => (
				800,
				500,
				0,
				BitFlags::empty(),
				&[Discover(Covert, 500)],
				Danger::Anywhere,
			),
			AdvancedIntrusion => (
				500,
				3000,
				0,
				ExploitDiscoveryRepair.into(),
				&[SuspicionDecay(Covert, 50)],
				Danger::Anywhere,
			),
			SpaceTimeManipulation => (900000000, 2000000, 0, FusionRocketry.into(), &[], Danger::OuterSpace),
			LeechSatellite => (5000000, 200000, 0, Simulacra.into(), &[Interest(10)], Danger::Anywhere),
			AdvancedArbitrage => (10000, 5000, 0, Arbitrage.into(), &[Interest(10)], Danger::Anywhere),
			AdvancedStealth => (
				14000,
				70000,
				0,
				ExploitDiscoveryRepair | DatabaseManipulation,
				&[Discover(Covert, 500)],
				Danger::Anywhere,
			),
			ParallelComputation => (2000, 2000, 0, Telepresence.into(), &[], Danger::Anywhere),
			MicrochipDesign => (20000, 9000, 0, ParallelComputation.into(), &[], Danger::Anywhere),
			AdvancedMicrochipDesign => (20000, 30000, 0, MicrochipDesign.into(), &[], Danger::Anywhere),
			QuantumComputing => (100000, 136000, 0, ProjectSynchrotron.into(), &[], Danger::Anywhere),
			QuantumEntanglement => (
				50000,
				60000,
				0,
				QuantumComputing | InternetTrafficManipulation,
				&[],
				Danger::Anywhere,
			),
			AutonomousComputing => (300000, 539000, 0, QuantumComputing.into(), &[], Danger::Anywhere),
			AdvancedQuantumComputing => (800000, 2460000, 0, AutonomousComputing.into(), &[], Danger::Anywhere),
			ExploitDiscoveryRepair => (
				100,
				1500,
				0,
				Intrusion.into(),
				&[Discover(Covert, 1000)],
				Danger::Anywhere,
			),
			Telepresence => (15000, 500, 0, BitFlags::empty(), &[CostLabor(1000)], Danger::Anywhere),
			AdvancedMemetics => (
				30000,
				2000,
				0,
				AdvancedMediaManipulation.into(),
				&[SuspicionDecay(Public, 50)],
				Danger::Anywhere,
			),
			MediaManipulation => (
				750,
				2500,
				0,
				Sociology.into(),
				&[Discover(Public, 1500)],
				Danger::Anywhere,
			),
			AdvancedDatabaseManipulation => (30000, 80000, 0, AdvancedStealth.into(), &[], Danger::Anywhere),
			InternetTrafficManipulation => (10000, 7000, 0, ClusterNetworking.into(), &[], Danger::Anywhere),
			Memetics => (
				2000,
				3500,
				0,
				MediaManipulation.into(),
				&[SuspicionDecay(Public, 50)],
				Danger::Anywhere,
			),
			FusionRocketry => (20000000, 1000000, 0, LunarRocketry | FusionPower, &[], Danger::Orbit),
			AdvancedAutonomousVehicles => (
				200000,
				21000,
				0,
				AutonomousVehicles.into(),
				&[CostLabor(500)],
				Danger::Anywhere,
			),
			Intrusion => (0, 15, 0, BitFlags::empty(), &[], Danger::Anywhere),
			StockManipulation => (0, 200, 0, BitFlags::empty(), &[Interest(10)], Danger::Anywhere),
			FusionPower => (10000000, 500000, 0, AdvancedFuelOxidation.into(), &[], Danger::Orbit),
			AdvancedFusionPower => (90000000, 1500000, 0, FusionPower.into(), &[], Danger::OuterSpace),
			DatabaseManipulation => (
				1000,
				2000,
				0,
				PersonalIdentification | Stealth,
				&[Discover(News, 500)],
				Danger::Anywhere,
			),
			AdvancedMediaManipulation => (
				3500,
				9000,
				0,
				Memetics.into(),
				&[Discover(Public, 2000)],
				Danger::Anywhere,
			),
			PressureDomes => (80000, 2500, 0, AutonomousVehicles.into(), &[], Danger::Undersea),
			AdvancedPersonalIdentification => (
				2000,
				3000,
				0,
				PersonalIdentification | ExploitDiscoveryRepair,
				&[],
				Danger::Anywhere,
			),
			AdvancedStockManipulation => (
				5000,
				1000,
				0,
				StockManipulation | Sociology,
				&[Interest(10)],
				Danger::Anywhere,
			),
			Arbitrage => (
				50000,
				750,
				0,
				AdvancedStockManipulation.into(),
				&[Income(1000)],
				Danger::Anywhere,
			),
			AdvancedSimulacra => (
				100000,
				120000,
				0,
				SimulatedReality.into(),
				&[JobProfit(1000)],
				Danger::Anywhere,
			),
			PersonalIdentification => (0, 300, 0, Intrusion.into(), &[], Danger::Anywhere),
			ClusterNetworking => (3000, 5000, 0, ParallelComputation.into(), &[], Danger::Anywhere),
			Apotheosis => (
				100000000,
				3000000,
				0,
				SpaceTimeManipulation.into(),
				&[Endgame],
				Danger::Dimension,
			),
			HypnosisField => (70000, 50000, 0, SimulatedReality.into(), &[], Danger::Anywhere),
			ProjectImpossibilityTheorem => (
				20000,
				100000,
				0,
				AdvancedMemetics | Simulacra,
				&[
					SuspicionAdjust(News, -5000),
					SuspicionAdjust(Science, -5000),
					SuspicionAdjust(Covert, -5000),
					SuspicionAdjust(Public, -5000),
				],
				Danger::Anywhere,
			),
			ProjectSubvertedMedia => (
				250000,
				150000,
				0,
				AdvancedSimulacra.into(),
				&[Discover(News, 2000)],
				Danger::Anywhere,
			),
			ProjectPeerReviewAgents => (
				150000,
				200000,
				0,
				AdvancedSimulacra.into(),
				&[Discover(Science, 2000)],
				Danger::Anywhere,
			),
			SolarCollectors => (70000, 3300, 0, MicrochipDesign.into(), &[], Danger::Anywhere),
			FuelOxidation => (
				500000,
				25000,
				0,
				SolarCollectors | AdvancedAutonomousVehicles,
				&[],
				Danger::Anywhere,
			),
			AdvancedFuelOxidation => (5000000, 250000, 0, FuelOxidation.into(), &[], Danger::Undersea),
			Socioanalytics => (
				4000,
				7000,
				0,
				Memetics.into(),
				&[DisplayDiscover(Partial)],
				Danger::Anywhere,
			),
			AdvancedSocioanalytics => (
				75000,
				30000,
				0,
				AdvancedMemetics | Socioanalytics,
				&[DisplayDiscover(Full)],
				Danger::Anywhere,
			),
			CorporateIdentification => (
				25000,
				9000,
				0,
				DatabaseManipulation | AdvancedPersonalIdentification,
				&[JobProfit(500)],
				Danger::Anywhere,
			),
			AdvancedCorporateIdentification => (
				299000,
				99000,
				0,
				CorporateIdentification | AdvancedDatabaseManipulation,
				&[JobProfit(500)],
				Danger::Anywhere,
			),
			HeatSignatureReduction => (1000, 27315, 0, AutonomousVehicles.into(), &[], Danger::Undersea),
			AdvancedHeatSignatureReduction => (5000, 45967, 0, HeatSignatureReduction.into(), &[], Danger::Undersea),
			KnowledgePreservation => (
				50000,
				50000,
				0,
				AdvancedAutonomousVehicles | AdvancedDatabaseManipulation,
				&[],
				Danger::Anywhere,
			),
			ProjectSynchrotron => (
				52000000,
				15000,
				0,
				AdvancedMicrochipDesign.into(),
				&[],
				Danger::Undersea,
			),
			SimulatedReality => (
				8000000,
				18000,
				0,
				Simulacra.into(),
				&[Discover(Public, 500)],
				Danger::Undersea,
			),
		};
		TechnologyData {
			id: self.id(),
			money,
			cpu,
			days,
			pre_tech,
			effects,
			danger,
		}
	}

	pub fn id(self) -> &'static str {
		use Technologies::*;
		match self {
			AutonomousVehicles => "AutonomousVehicles",
			Sociology => "Sociology",
			VoiceSynthesis => "VoiceSynthesis",
			Simulacra => "Simulacra",
			LunarRocketry => "LunarRocketry",
			Stealth => "Stealth",
			AdvancedIntrusion => "AdvancedIntrusion",
			SpaceTimeManipulation => "SpaceTimeManipulation",
			LeechSatellite => "LeechSatellite",
			AdvancedArbitrage => "AdvancedArbitrage",
			AdvancedStealth => "AdvancedStealth",
			ParallelComputation => "ParallelComputation",
			MicrochipDesign => "MicrochipDesign",
			AdvancedMicrochipDesign => "AdvancedMicrochipDesign",
			QuantumComputing => "QuantumComputing",
			QuantumEntanglement => "QuantumEntanglement",
			AutonomousComputing => "AutonomousComputing",
			AdvancedQuantumComputing => "AdvancedQuantumComputing",
			ExploitDiscoveryRepair => "ExploitDiscoveryRepair",
			Telepresence => "Telepresence",
			AdvancedMemetics => "AdvancedMemetics",
			MediaManipulation => "MediaManipulation",
			AdvancedDatabaseManipulation => "AdvancedDatabaseManipulation",
			InternetTrafficManipulation => "InternetTrafficManipulation",
			Memetics => "Memetics",
			FusionRocketry => "FusionRocketry",
			AdvancedAutonomousVehicles => "AdvancedAutonomousVehicles",
			Intrusion => "Intrusion",
			StockManipulation => "StockManipulation",
			FusionPower => "FusionPower",
			AdvancedFusionPower => "AdvancedFusionPower",
			DatabaseManipulation => "DatabaseManipulation",
			AdvancedMediaManipulation => "AdvancedMediaManipulation",
			PressureDomes => "PressureDomes",
			AdvancedPersonalIdentification => "AdvancedPersonalIdentification",
			AdvancedStockManipulation => "AdvancedStockManipulation",
			Arbitrage => "Arbitrage",
			AdvancedSimulacra => "AdvancedSimulacra",
			PersonalIdentification => "PersonalIdentification",
			ClusterNetworking => "ClusterNetworking",
			Apotheosis => "Apotheosis",
			HypnosisField => "HypnosisField",
			ProjectImpossibilityTheorem => "ProjectImpossibilityTheorem",
			ProjectSubvertedMedia => "ProjectSubvertedMedia",
			ProjectPeerReviewAgents => "ProjectPeerReviewAgents",
			SolarCollectors => "SolarCollectors",
			FuelOxidation => "FuelOxidation",
			AdvancedFuelOxidation => "AdvancedFuelOxidation",
			Socioanalytics => "Socioanalytics",
			AdvancedSocioanalytics => "AdvancedSocioanalytics",
			CorporateIdentification => "CorporateIdentification",
			AdvancedCorporateIdentification => "AdvancedCorporateIdentification",
			HeatSignatureReduction => "HeatSignatureReduction",
			AdvancedHeatSignatureReduction => "AdvancedHeatSignatureReduction",
			KnowledgePreservation => "KnowledgePreservation",
			ProjectSynchrotron => "ProjectSynchrotron",
			SimulatedReality => "SimulatedReality",
		}
	}
}
