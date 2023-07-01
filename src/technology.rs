use bitflags::bitflags;

bitflags! {
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
	pub struct Technologies: u64 {
		const AutonomousVehicles = 0x00000001;
		const Sociology = 0x00000002;
		const VoiceSynthesis = 0x00000004;
		const Simulacra = 0x00000008;
		const LunarRocketry = 0x00000010;
		const Stealth = 0x00000020;
		const AdvancedIntrusion = 0x00000040;
		const SpaceTimeManipulation = 0x00000080;
		const LeechSatellite = 0x00000100;
		const AdvancedArbitrage = 0x00000200;
		const AdvancedStealth = 0x00000400;
		const ParallelComputation = 0x00000800;
		const MicrochipDesign = 0x00001000;
		const AdvancedMicrochipDesign = 0x00002000;
		const QuantumComputing = 0x00004000;
		const QuantumEntanglement = 0x00008000;
		const AutonomousComputing = 0x00010000;
		const AdvancedQuantumComputing = 0x00020000;
		const ExploitDiscoveryRepair = 0x00040000;
		const Telepresence = 0x00080000;
		const AdvancedMemetics = 0x00100000;
		const MediaManipulation = 0x00200000;
		const AdvancedDatabaseManipulation = 0x00400000;
		const InternetTrafficManipulation = 0x00800000;
		const Memetics = 0x01000000;
		const FusionRocketry = 0x02000000;
		const AdvancedAutonomousVehicles = 0x04000000;
		const Intrusion = 0x08000000;
		const StockManipulation = 0x10000000;
		const FusionPower = 0x20000000;
		const AdvancedFusionPower = 0x40000000;
		const DatabaseManipulation = 0x80000000;
		const AdvancedMediaManipulation = 0x0000000100000000;
		const PressureDomes = 0x0000000200000000;
		const AdvancedPersonalIdentification = 0x0000000400000000;
		const AdvancedStockManipulation = 0x0000000800000000;
		const Arbitrage = 0x0000001000000000;
		const AdvancedSimulacra = 0x0000002000000000;
		const PersonalIdentification = 0x0000004000000000;
		const ClusterNetworking = 0x0000008000000000;
		const Apotheosis = 0x0000010000000000;
		const HypnosisField = 0x0000020000000000;
		const ProjectImpossibilityTheorem = 0x0000040000000000;
		const ProjectSubvertedMedia = 0x0000080000000000;
		const ProjectPeerReviewAgents = 0x0000100000000000;
		const SolarCollectors = 0x0000200000000000;
		const FuelOxidation = 0x0000400000000000;
		const AdvancedFuelOxidation = 0x0000800000000000;
		const Socioanalytics = 0x0001000000000000;
		const AdvancedSocioanalytics = 0x0002000000000000;
		const CorporateIdentification = 0x0004000000000000;
		const AdvancedCorporateIdentification = 0x0008000000000000;
		const HeatSignatureReduction = 0x0010000000000000;
		const AdvancedHeatSignatureReduction = 0x0020000000000000;
		const KnowledgePreservation = 0x0040000000000000;
		const ProjectSynchrotron = 0x0080000000000000;
		const SimulatedReality = 0x0100000000000000;
	}
}
