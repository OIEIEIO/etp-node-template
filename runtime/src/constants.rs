pub mod currency {
    use node_primitives::Balance;
    pub const DNA: Balance = 10_000;
    pub const DOLLARS: Balance = DNA;
    pub const CENTS: Balance = DOLLARS / 100;
    pub const MILLICENTS: Balance = CENTS / 1_000;
}
/// Time. pub mod time {
    use node_primitives::{BlockNumber, Moment};
    pub const MILLISECS_PER_BLOCK: Moment = 500;
    pub const SLOT_DURATION: Moment = MILLISECS_PER_BLOCK;
    pub const EPOCH_DURATION_IN_BLOCKS: BlockNumber = 1 * HOURS;
    // These time units are defined in number of blocks.
    pub const MINUTES: BlockNumber = 10_000 / (MILLISECS_PER_BLOCK as BlockNumber);
    pub const HOURS: BlockNumber = MINUTES * 60;
    pub const DAYS: BlockNumber = HOURS * 24;
    // 1 in 4 blocks (on average, not counting collisions) will be primary babe blocks.
    pub const PRIMARY_PROBABILITY: (u64, u64) = (1, 4);
}

 pub mod fee {
    pub use sp_runtime::Perbill;
    /// The block saturation level. Fees will be updates based on this value.
    pub const TARGET_BLOCK_FULLNESS: Perbill = Perbill::from_percent(25);
}
