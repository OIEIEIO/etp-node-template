//! Common runtime code for DNA and dna-network.

#![cfg_attr(not(feature = "std"), no_std)]

/// Implementations of some helper traits passed into runtime modules as associated types.
pub mod impls;
pub use impls::*;

// --- substrate ---
pub use frame_support::weights::constants::{
	BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight,
};
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;
// --- DNA ---
#[cfg(feature = "std")]
pub use DNA_staking::StakerStatus;

// --- substrate ---
use frame_support::{
	parameter_types,
	traits::Currency,
	weights::{constants::WEIGHT_PER_SECOND, Weight},
};
use sp_runtime::Perbill;
// --- DNA ---
use DNA_primitives::BlockNumber;

pub type EtpInstance = DNA_balances::Instance0;
pub type DnaInstance = DNA_balances::Instance1;

pub type NegativeImbalance<T> = <DNA_balances::Module<T, EtpInstance> as Currency<
	<T as frame_system::Trait>::AccountId,
>>::NegativeImbalance;

parameter_types! {
	pub const BlockHashCount: BlockNumber = 2400;
	pub const MaximumBlockWeight: Weight = 2 * WEIGHT_PER_SECOND;
	pub const MaximumBlockLength: u32 = 5 * 1024 * 1024;
	pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
}
