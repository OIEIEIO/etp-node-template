//! DNA chain configurations.

// --- crates ---
use serde::{Deserialize, Serialize};
// --- substrate ---
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use sc_chain_spec::{ChainSpecExtension, ChainType};
use sc_finality_grandpa::AuthorityId as GrandpaId;
use sc_service::Properties;
use sc_telemetry::TelemetryEndpoints;
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public};
use sp_runtime::{traits::IdentifyAccount, Perbill};
// --- DNA ---
use array_bytes::fixed_hex_bytes_unchecked;
use dna-network_runtime::{constants::currency::COIN as ETP, GenesisConfig as dna-networkGenesisConfig};
use DNA_primitives::{AccountId, AccountPublic, Balance};

const CDNA: Balance = ETP;
const dna-network_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";
const DEFAULT_PROTOCOL_ID: &str = "dar";

/// Node `ChainSpec` extensions.
///
/// Additional parameters for some Substrate core modules,
/// customizable from the chain spec.
#[derive(Default, Clone, Serialize, Deserialize, ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
	/// Block numbers with known hashes.
	pub fork_blocks: sc_client_api::ForkBlocks<DNA_primitives::Block>,
	/// Known bad block hashes.
	pub bad_blocks: sc_client_api::BadBlocks<DNA_primitives::Block>,
}

/// The `ChainSpec parametrised for dna-network runtime`.
pub type dna-networkChainSpec = sc_service::GenericChainSpec<dna-networkGenesisConfig, Extensions>;

pub fn dna-network_config() -> Result<dna-networkChainSpec, Stetp> {
	dna-networkChainSpec::from_json_bytes(&include_bytes!("../res/dna-network.json")[..])
}

/// Session keys for dna-network.
pub fn dna-network_session_keys(
	babe: BabeId,
	grandpa: GrandpaId,
	im_online: ImOnlineId,
	authority_discovery: AuthorityDiscoveryId,
) -> dna-network_runtime::SessionKeys {
	dna-network_runtime::SessionKeys {
		babe,
		grandpa,
		im_online,
		authority_discovery,
	}
}

/// Properties for dna-network.
pub fn dna-network_properties() -> Properties {
	let mut properties = Properties::new();

	properties.insert("ss58Format".into(), 42.into());
	properties.insert("tokenDecimals".into(), 9.into());
	properties.insert("tokenSymbol".into(), "ETP".into());
	properties.insert("DNATokenDecimals".into(), 9.into());
	properties.insert("DNATokenSymbol".into(), "CDNA".into());

	properties
}

pub fn dna-network_build_spec_genesis() -> dna-networkGenesisConfig {
	const ETP_ENDOWMENT: Balance = 1_000_000 * ETP;
	const DNA_ENDOWMENT: Balance = 10_000 * CDNA;

	struct Staker {
		sr: [u8; 32],
		ed: [u8; 32],
	}

	impl Staker {
		fn build_init_auth(
			&self,
		) -> (
			AccountId,
			AccountId,
			BabeId,
			GrandpaId,
			ImOnlineId,
			AuthorityDiscoveryId,
		) {
			(
				self.sr.into(),
				self.sr.into(),
				self.sr.unchecked_into(),
				self.ed.unchecked_into(),
				self.sr.unchecked_into(),
				self.sr.unchecked_into(),
			)
		}
	}

	// 5FGWcEpsd5TbDh14UGJEzRQENwrPXUt7e2ufzFzfcCEMesAQ
	let multi_sign: AccountId = fixed_hex_bytes_unchecked!(
		"0x8db5c746c14cf05e182b10576a9ee765265366c3b7fd53c41d43640c97f4a8b8",
		32
	)
	.into();

	let root_key: AccountId = fixed_hex_bytes_unchecked!(
		"0x0a66532a23c418cca12183fee5f6afece770a0bb8725f459d7d1b1b598f91c49",
		32
	)
	.into();

	let stakers = [
		// AlexChien
		Staker {
			sr: fixed_hex_bytes_unchecked!(
				"0x80a5d9612f5504f3e04a31ca19f1d6108ca77252bd05940031eb446953409c1a",
				32
			),
			ed: fixed_hex_bytes_unchecked!(
				"0x1b861031d9a6edea47c6478cb3765d7cd4881b36bfb1c665f6b6deb5e0d9c253",
				32
			),
		},
		// AurevoirXavier
		Staker {
			// 5G9z8Ttoo7892VqBHiSWCbnd2aEdH8noJLqZ4HFMzMVNhvgP
			sr: fixed_hex_bytes_unchecked!(
				"0xb4f7f03bebc56ebe96bc52ea5ed3159d45a0ce3a8d7f082983c33ef133274747",
				32
			),
			// 5ETtsEtnsGQZc5jcAJazedgmiePShJ43VyrY88aCvdQmkvj8
			ed: fixed_hex_bytes_unchecked!(
				"0x6a282c7674945c039a9289b702376ae168e8b67c9ed320054e2a019015f236fd",
				32
			),
		},
		// clearloop
		Staker {
			sr: fixed_hex_bytes_unchecked!(
				"0x6e6844ba5c73db6c4c6b67ea59c2787dd6bd2f9b8139a69c33e14a722d1e801d",
				32
			),
			ed: fixed_hex_bytes_unchecked!(
				"0x13c0b78d9573e99a74c313ddcf30f8fc3d3bc0503f8864427ad34654804e1bc5",
				32
			),
		},
		// freehere107
		Staker {
			sr: fixed_hex_bytes_unchecked!(
				"0xc4429847f3598f40008d0cbab53476a2f19165696aa41002778524b3ecf82938",
				32
			),
			ed: fixed_hex_bytes_unchecked!(
				"0x2c8cb4d2de3192df18c60551038a506033cb2a85fbe0a3ff8cff413dac11f50a",
				32
			),
		},
		// HackFisher
		Staker {
			sr: fixed_hex_bytes_unchecked!(
				"0xb62d88e3f439fe9b5ea799b27bf7c6db5e795de1784f27b1bc051553499e420f",
				32
			),
			ed: fixed_hex_bytes_unchecked!(
				"0x398f7935e0ea85cc2d1af71dab00d93f53b2cbf35e2afb1e6087f7554d2fdf96",
				32
			),
		},
		// WoeOm
		Staker {
			// 5C8thCAFsaTHuJFMJZz2CrT47XDWebP72Vwr9d1sL4eSJ4UM
			sr: fixed_hex_bytes_unchecked!(
				"0x0331760198d850b159844f3bfa620f6e704167973213154aca27675f7ddd987e",
				32
			),
			// 5D2ocj7mvu5oemVwK2TXUz7tmNumtPSYdjs4fmFmNKQ9PJ3A
			ed: fixed_hex_bytes_unchecked!(
				"0x2ac9219ace40f5846ed675dded4e25a1997da7eabdea2f78597a71d6f3803148",
				32
			),
		},
		// yanganto
		Staker {
			sr: fixed_hex_bytes_unchecked!(
				"0xc45f075b5b1aa0145c469f57bd741c02272c1c0c41e9518d5a32426030d98232",
				32
			),
			ed: fixed_hex_bytes_unchecked!(
				"0xaf78c408272f929225861c8276c6e8700c8f45c195b9ba82a0b246aade0937ec",
				32
			),
		},
	];

	// local tester
	let local_tester = Staker {
		// Secret phrase `pulse upset spoil fatigue agent credit dirt language forest aware boat broom` is account:
		// Network ID/version: substrate
		// Secret seed:        0x76c87263b2a385fcb7faed857d0fe105b5e40cdc8cb5f1b2a188d7f57488e595
		// Public key (hex):   0x584ea8f083c3a9038d57acc5229ab4d790ab6132921d5edc5fae1be4ed89ec1f
		// Account ID:         0x584ea8f083c3a9038d57acc5229ab4d790ab6132921d5edc5fae1be4ed89ec1f
		// SS58 Address:       5E4VSMKXm9VFaLMu4Jjbny3Uy7NnPizoGkf92A15XjS45C4A
		sr: fixed_hex_bytes_unchecked!(
			"0x584ea8f083c3a9038d57acc5229ab4d790ab6132921d5edc5fae1be4ed89ec1f",
			32
		),
		// Secret phrase `ecology admit arrest canal cage believe satoshi anger napkin sign decorate use` is account:
		// Network ID/version: substrate
		// Secret seed:        0x7b37f9bd46a368748e0e28992e2cd2bc77060cd8267784aef625fb812908fb7f
		// Public key (hex):   0x70fa82107e81f20bb4e5b059f4ac800d55aafcff9e918e000899569b4f207976
		// Account ID:         0x70fa82107e81f20bb4e5b059f4ac800d55aafcff9e918e000899569b4f207976
		// SS58 Address:       5Ecqdt4nxP76MdwNfBwwYBi4mxWq7MYLDN1GXMtDFUSaerjG
		ed: fixed_hex_bytes_unchecked!(
			"0x70fa82107e81f20bb4e5b059f4ac800d55aafcff9e918e000899569b4f207976",
			32
		),
	};

	let endowed_accounts = stakers
		.iter()
		.map(|staker| staker.sr.into())
		.collect::<Vec<_>>();

	let initial_authorities = [stakers[1].build_init_auth(), local_tester.build_init_auth()];

	dna-networkGenesisConfig {
		// --- substrate ---
		frame_system: Some(dna-network_runtime::SystemConfig {
			code: dna-network_runtime::WASM_BINARY.to_vec(),
			changes_trie_config: Default::default(),
		}),
		pallet_babe: Some(Default::default()),
		pallet_indices: Some(Default::default()),
		pallet_session: Some(dna-network_runtime::SessionConfig {
			keys: initial_authorities
				.iter()
				.cloned()
				.map(|x| (x.0.clone(), x.0, dna-network_session_keys(x.2, x.3, x.4, x.5)))
				.collect(),
		}),
		pallet_grandpa: Some(Default::default()),
		pallet_im_online: Some(Default::default()),
		pallet_authority_discovery: Some(Default::default()),
		pallet_collective_Instance1: Some(Default::default()),
		pallet_collective_Instance2: Some(Default::default()),
		pallet_membership_Instance1: Some(Default::default()),
		pallet_sudo: Some(dna-network_runtime::SudoConfig {
			key: root_key.clone(),
		}),
		// --- DNA ---
		DNA_balances_Instance0: Some(dna-network_runtime::BalancesConfig {
			balances: endowed_accounts
				.iter()
				.cloned()
				.map(|k| (k, ETP_ENDOWMENT))
				.chain(
					vec![
						(root_key, 25_000_000 * ETP),
						(multi_sign, 700_000_000 * ETP),
						(local_tester.sr.into(), ETP),
					]
					.into_iter(),
				)
				.collect(),
		}),
		DNA_balances_Instance1: Some(dna-network_runtime::DnaConfig {
			balances: endowed_accounts
				.iter()
				.cloned()
				.map(|k| (k, DNA_ENDOWMENT))
				.collect(),
		}),
		DNA_staking: Some(dna-network_runtime::StakingConfig {
			minimum_validator_count: 2,
			validator_count: 7,
			stakers: initial_authorities
				.iter()
				.cloned()
				.map(|x| (x.0, x.1, ETP, dna-network_runtime::StakerStatus::Validator))
				.collect(),
			force_era: dna-network_runtime::Forcing::NotForcing,
			slash_reward_fraction: Perbill::from_percent(10),
			payout_fraction: Perbill::from_percent(50),
			..Default::default()
		}),
		DNA_elections_phragmen: Some(Default::default()),
		DNA_claims: Some({
			dna-network_runtime::ClaimsConfig {
				claims_list: dna-network_runtime::ClaimsList::from_file(
					"./service/res/dna-network_claims_list.json",
					"CLAIMS_LIST_PATH",
				),
			}
		}),
		DNA_eth_backing: Some(dna-network_runtime::EthBackingConfig {
			etp_redeem_address: fixed_hex_bytes_unchecked!(
				"0x4e99Ed57FF0C5B95f1F46AC314dAef3d547Bf7e4",
				20
			)
			.into(),
			DNA_redeem_address: fixed_hex_bytes_unchecked!(
				"0x4e99Ed57FF0C5B95f1F46AC314dAef3d547Bf7e4",
				20
			)
			.into(),
			deposit_redeem_address: fixed_hex_bytes_unchecked!(
				"0x458B84F0Da1A157d34ea48c3863DF80b1D50EB8d",
				20
			)
			.into(),
			etp_locked: 7_569_833 * ETP,
			DNA_locked: 30_000 * ETP,
			..Default::default()
		}),
		DNA_eth_relay: Some(dna-network_runtime::EthRelayConfig {
			genesis_header: Some((
				0x7c2b522520376,
				vec![
					249, 2, 21, 160, 51, 118, 13, 128, 187, 35, 172, 229, 179, 73, 30, 227, 114,
					68, 193, 247, 169, 153, 156, 222, 107, 253, 65, 244, 199, 209, 50, 236, 211,
					71, 76, 76, 160, 29, 204, 77, 232, 222, 199, 93, 122, 171, 133, 181, 103, 182,
					204, 212, 26, 211, 18, 69, 27, 148, 138, 116, 19, 240, 161, 66, 253, 64, 212,
					147, 71, 148, 90, 11, 84, 213, 220, 23, 224, 170, 220, 56, 61, 45, 180, 59, 10,
					13, 62, 2, 156, 76, 160, 170, 137, 13, 105, 60, 0, 150, 36, 9, 0, 141, 222, 7,
					81, 76, 213, 223, 181, 227, 161, 58, 105, 228, 19, 245, 144, 198, 85, 177, 41,
					91, 241, 160, 170, 170, 213, 227, 172, 215, 36, 181, 118, 199, 56, 191, 232,
					70, 124, 187, 126, 91, 186, 195, 110, 134, 212, 101, 20, 110, 9, 244, 221, 146,
					183, 32, 160, 172, 176, 231, 72, 144, 120, 210, 152, 245, 224, 66, 30, 41, 139,
					153, 87, 168, 164, 92, 233, 100, 65, 111, 96, 90, 120, 116, 34, 125, 23, 43,
					235, 185, 1, 0, 156, 160, 54, 144, 128, 28, 25, 48, 200, 4, 160, 47, 99, 38,
					154, 67, 64, 216, 4, 37, 84, 140, 48, 24, 224, 87, 208, 22, 98, 211, 177, 110,
					48, 193, 112, 117, 117, 133, 111, 0, 160, 20, 38, 2, 8, 143, 39, 32, 0, 97,
					132, 132, 210, 73, 196, 129, 11, 200, 114, 134, 245, 44, 56, 96, 162, 201, 17,
					134, 218, 40, 197, 99, 111, 2, 229, 221, 10, 48, 94, 77, 106, 7, 106, 162, 145,
					44, 160, 7, 130, 42, 0, 100, 22, 164, 52, 105, 212, 64, 65, 130, 70, 66, 210,
					141, 132, 69, 228, 140, 128, 128, 88, 0, 41, 64, 82, 216, 76, 160, 85, 41, 15,
					166, 104, 151, 113, 49, 28, 8, 56, 248, 129, 4, 122, 40, 104, 2, 4, 2, 226, 35,
					128, 86, 227, 111, 156, 5, 133, 14, 125, 226, 16, 14, 129, 194, 168, 200, 0,
					60, 64, 243, 134, 22, 5, 142, 1, 98, 48, 0, 38, 8, 9, 131, 19, 147, 154, 15,
					134, 34, 128, 136, 160, 198, 8, 42, 0, 84, 74, 96, 58, 97, 72, 85, 32, 152,
					194, 126, 4, 72, 20, 68, 168, 130, 186, 58, 64, 43, 32, 14, 45, 66, 210, 246,
					192, 40, 136, 129, 128, 128, 120, 26, 55, 147, 165, 1, 73, 81, 6, 152, 25, 134,
					24, 81, 132, 28, 0, 20, 192, 40, 240, 17, 72, 34, 76, 82, 26, 131, 150, 130, 9,
					139, 152, 0, 140, 58, 80, 72, 135, 7, 194, 181, 34, 82, 3, 118, 131, 152, 10,
					122, 131, 152, 45, 20, 131, 152, 34, 4, 132, 94, 168, 200, 77, 148, 115, 112,
					97, 114, 107, 112, 111, 111, 108, 45, 101, 116, 104, 45, 99, 110, 45, 104, 122,
					51, 160, 116, 165, 221, 119, 93, 66, 155, 159, 213, 183, 132, 248, 100, 173,
					114, 41, 170, 215, 11, 70, 199, 46, 207, 61, 147, 142, 199, 17, 191, 195, 87,
					175, 136, 166, 205, 126, 48, 0, 44, 96, 35,
				],
			)),
			authorities: stakers.iter().map(|staker| staker.sr.into()).collect(),
			dags_merkle_roots_loader: dna-network_runtime::DagsMerkleRootsLoader::from_file(
				"./service/res/dags_merkle_roots_loader.json",
				"DAG_MERKLE_ROOTS_PATH",
			),
			..Default::default()
		}),
	}
}

/// dna-network config.
pub fn dna-network_build_spec_config() -> dna-networkChainSpec {
	let boot_nodes = vec![];
	dna-networkChainSpec::from_genesis(
		"dna-network",
		"dna-network",
		ChainType::Live,
		dna-network_build_spec_genesis,
		boot_nodes,
		Some(
			TelemetryEndpoints::new(vec![(dna-network_TELEMETRY_URL.to_stetp(), 0)])
				.expect("dna-network telemetry url is valid; qed"),
		),
		Some(DEFAULT_PROTOCOL_ID),
		Some(dna-network_properties()),
		Default::default(),
	)
}

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_stetp(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate stash, controller and session key from seed
pub fn get_authority_keys_from_seed(
	seed: &str,
) -> (
	AccountId,
	AccountId,
	BabeId,
	GrandpaId,
	ImOnlineId,
	AuthorityDiscoveryId,
) {
	(
		get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
		get_account_id_from_seed::<sr25519::Public>(seed),
		get_from_seed::<BabeId>(seed),
		get_from_seed::<GrandpaId>(seed),
		get_from_seed::<ImOnlineId>(seed),
		get_from_seed::<AuthorityDiscoveryId>(seed),
	)
}

fn testnet_accounts() -> Vec<AccountId> {
	vec![
		get_account_id_from_seed::<sr25519::Public>("Oli"),
		get_account_id_from_seed::<sr25519::Public>("Razvan"),
		get_account_id_from_seed::<sr25519::Public>("Adrian"),
		get_account_id_from_seed::<sr25519::Public>("Jorge"),
		get_account_id_from_seed::<sr25519::Public>("Carlos"),
		get_account_id_from_seed::<sr25519::Public>("Eric"),
		get_account_id_from_seed::<sr25519::Public>("Oli//stash"),
		get_account_id_from_seed::<sr25519::Public>("Razvan//stash"),
		get_account_id_from_seed::<sr25519::Public>("Adrian//stash"),
		get_account_id_from_seed::<sr25519::Public>("Jorge//stash"),
		get_account_id_from_seed::<sr25519::Public>("Carlos//stash"),
		get_account_id_from_seed::<sr25519::Public>("Eric//stash"),
	]
}

/// Helper function to create dna-network GenesisConfig for testing
pub fn dna-network_testnet_genesis(
	initial_authorities: Vec<(
		AccountId,
		AccountId,
		BabeId,
		GrandpaId,
		ImOnlineId,
		AuthorityDiscoveryId,
	)>,
	root_key: AccountId,
	endowed_accounts: Option<Vec<AccountId>>,
) -> dna-networkGenesisConfig {
	let endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(testnet_accounts);

	dna-networkGenesisConfig {
		// --- substrate ---
		frame_system: Some(dna-network_runtime::SystemConfig {
			code: dna-network_runtime::WASM_BINARY.to_vec(),
			changes_trie_config: Default::default(),
		}),
		pallet_babe: Some(Default::default()),
		pallet_indices: Some(Default::default()),
		pallet_session: Some(dna-network_runtime::SessionConfig {
			keys: initial_authorities
				.iter()
				.cloned()
				.map(|x| (x.0.clone(), x.0, dna-network_session_keys(x.2, x.3, x.4, x.5)))
				.collect(),
		}),
		pallet_grandpa: Some(Default::default()),
		pallet_im_online: Some(Default::default()),
		pallet_authority_discovery: Some(Default::default()),
		pallet_collective_Instance1: Some(Default::default()),
		pallet_collective_Instance2: Some(Default::default()),
		pallet_membership_Instance1: Some(Default::default()),
		pallet_sudo: Some(dna-network_runtime::SudoConfig { key: root_key }),
		// --- DNA ---
		DNA_balances_Instance0: Some(dna-network_runtime::BalancesConfig {
			balances: endowed_accounts
				.iter()
				.cloned()
				.map(|k| (k, 1 << 60))
				.collect(),
		}),
		DNA_balances_Instance1: Some(dna-network_runtime::DnaConfig {
			balances: endowed_accounts
				.iter()
				.cloned()
				.map(|k| (k, 1 << 60))
				.collect(),
		}),
		DNA_staking: Some(dna-network_runtime::StakingConfig {
			minimum_validator_count: 1,
			validator_count: 2,
			stakers: initial_authorities
				.iter()
				.cloned()
				.map(|x| (x.0, x.1, 1 << 60, dna-network_runtime::StakerStatus::Validator))
				.collect(),
			invulnerables: initial_authorities.iter().cloned().map(|x| x.0).collect(),
			force_era: dna-network_runtime::Forcing::NotForcing,
			slash_reward_fraction: Perbill::from_percent(10),
			payout_fraction: Perbill::from_percent(50),
			..Default::default()
		}),
		DNA_elections_phragmen: Some(Default::default()),
		DNA_claims: Some({
			dna-network_runtime::ClaimsConfig {
				claims_list: dna-network_runtime::ClaimsList::from_file(
					"./service/res/dna-network_claims_list.json",
					"CLAIMS_LIST_PATH",
				),
			}
		}),
		DNA_eth_backing: Some(dna-network_runtime::EthBackingConfig {
			etp_redeem_address: fixed_hex_bytes_unchecked!(
				"0x4e99Ed57FF0C5B95f1F46AC314dAef3d547Bf7e4",
				20
			)
			.into(),
			DNA_redeem_address: fixed_hex_bytes_unchecked!(
				"0x4e99Ed57FF0C5B95f1F46AC314dAef3d547Bf7e4",
				20
			)
			.into(),
			deposit_redeem_address: fixed_hex_bytes_unchecked!(
				"0x458B84F0Da1A157d34ea48c3863DF80b1D50EB8d",
				20
			)
			.into(),
			etp_locked: 1 << 60,
			DNA_locked: 1 << 60,
			..Default::default()
		}),
		DNA_eth_relay: Some(dna-network_runtime::EthRelayConfig {
			genesis_header: Some((
				0x7c2b522520376,
				vec![
					249, 2, 21, 160, 51, 118, 13, 128, 187, 35, 172, 229, 179, 73, 30, 227, 114,
					68, 193, 247, 169, 153, 156, 222, 107, 253, 65, 244, 199, 209, 50, 236, 211,
					71, 76, 76, 160, 29, 204, 77, 232, 222, 199, 93, 122, 171, 133, 181, 103, 182,
					204, 212, 26, 211, 18, 69, 27, 148, 138, 116, 19, 240, 161, 66, 253, 64, 212,
					147, 71, 148, 90, 11, 84, 213, 220, 23, 224, 170, 220, 56, 61, 45, 180, 59, 10,
					13, 62, 2, 156, 76, 160, 170, 137, 13, 105, 60, 0, 150, 36, 9, 0, 141, 222, 7,
					81, 76, 213, 223, 181, 227, 161, 58, 105, 228, 19, 245, 144, 198, 85, 177, 41,
					91, 241, 160, 170, 170, 213, 227, 172, 215, 36, 181, 118, 199, 56, 191, 232,
					70, 124, 187, 126, 91, 186, 195, 110, 134, 212, 101, 20, 110, 9, 244, 221, 146,
					183, 32, 160, 172, 176, 231, 72, 144, 120, 210, 152, 245, 224, 66, 30, 41, 139,
					153, 87, 168, 164, 92, 233, 100, 65, 111, 96, 90, 120, 116, 34, 125, 23, 43,
					235, 185, 1, 0, 156, 160, 54, 144, 128, 28, 25, 48, 200, 4, 160, 47, 99, 38,
					154, 67, 64, 216, 4, 37, 84, 140, 48, 24, 224, 87, 208, 22, 98, 211, 177, 110,
					48, 193, 112, 117, 117, 133, 111, 0, 160, 20, 38, 2, 8, 143, 39, 32, 0, 97,
					132, 132, 210, 73, 196, 129, 11, 200, 114, 134, 245, 44, 56, 96, 162, 201, 17,
					134, 218, 40, 197, 99, 111, 2, 229, 221, 10, 48, 94, 77, 106, 7, 106, 162, 145,
					44, 160, 7, 130, 42, 0, 100, 22, 164, 52, 105, 212, 64, 65, 130, 70, 66, 210,
					141, 132, 69, 228, 140, 128, 128, 88, 0, 41, 64, 82, 216, 76, 160, 85, 41, 15,
					166, 104, 151, 113, 49, 28, 8, 56, 248, 129, 4, 122, 40, 104, 2, 4, 2, 226, 35,
					128, 86, 227, 111, 156, 5, 133, 14, 125, 226, 16, 14, 129, 194, 168, 200, 0,
					60, 64, 243, 134, 22, 5, 142, 1, 98, 48, 0, 38, 8, 9, 131, 19, 147, 154, 15,
					134, 34, 128, 136, 160, 198, 8, 42, 0, 84, 74, 96, 58, 97, 72, 85, 32, 152,
					194, 126, 4, 72, 20, 68, 168, 130, 186, 58, 64, 43, 32, 14, 45, 66, 210, 246,
					192, 40, 136, 129, 128, 128, 120, 26, 55, 147, 165, 1, 73, 81, 6, 152, 25, 134,
					24, 81, 132, 28, 0, 20, 192, 40, 240, 17, 72, 34, 76, 82, 26, 131, 150, 130, 9,
					139, 152, 0, 140, 58, 80, 72, 135, 7, 194, 181, 34, 82, 3, 118, 131, 152, 10,
					122, 131, 152, 45, 20, 131, 152, 34, 4, 132, 94, 168, 200, 77, 148, 115, 112,
					97, 114, 107, 112, 111, 111, 108, 45, 101, 116, 104, 45, 99, 110, 45, 104, 122,
					51, 160, 116, 165, 221, 119, 93, 66, 155, 159, 213, 183, 132, 248, 100, 173,
					114, 41, 170, 215, 11, 70, 199, 46, 207, 61, 147, 142, 199, 17, 191, 195, 87,
					175, 136, 166, 205, 126, 48, 0, 44, 96, 35,
				],
			)),
			check_authority: false,
			dags_merkle_roots_loader: dna-network_runtime::DagsMerkleRootsLoader::from_file(
				"./service/res/dags_merkle_roots_loader.json",
				"DAG_MERKLE_ROOTS_PATH",
			),
			..Default::default()
		}),
	}
}

/// dna-network development config (single validator Alice)
pub fn dna-network_development_config() -> dna-networkChainSpec {
	fn dna-network_development_genesis() -> dna-networkGenesisConfig {
		dna-network_testnet_genesis(
			vec![get_authority_keys_from_seed("Alice")],
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			None,
		)
	}

	dna-networkChainSpec::from_genesis(
		"Development",
		"dna-network_dev",
		ChainType::Development,
		dna-network_development_genesis,
		vec![],
		None,
		Some(DEFAULT_PROTOCOL_ID),
		Some(dna-network_properties()),
		Default::default(),
	)
}

/// dna-network local testnet config (multivalidator Alice + Bob)
pub fn dna-network_local_testnet_config() -> dna-networkChainSpec {
	fn dna-network_local_testnet_genesis() -> dna-networkGenesisConfig {
		dna-network_testnet_genesis(
			vec![
				get_authority_keys_from_seed("Alice"),
				get_authority_keys_from_seed("Bob"),
			],
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			None,
		)
	}

	dna-networkChainSpec::from_genesis(
		"dna-network Local Testnet",
		"dna-network_local_testnet",
		ChainType::Local,
		dna-network_local_testnet_genesis,
		vec![],
		None,
		Some(DEFAULT_PROTOCOL_ID),
		Some(dna-network_properties()),
		Default::default(),
	)
}
