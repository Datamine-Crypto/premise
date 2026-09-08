use patterns::because;

pub const MAX_ROLLS: u32 = 3;
because!(MAX_ROLLS, "one roll per boss phase and bosses have three phases");

pub const TIER_COUNT: u64 = 3;
because!(TIER_COUNT, "the loot table ships common, rare and epic only");

pub const RARITY_LABEL: &str = "rarity";
because!(RARITY_LABEL, "the key the client reads when it renders a drop toast");

pub const TIERS: [Rarity; 3] = [Rarity::Common, Rarity::Rare, Rarity::Epic];
because!(TIERS, "the drop table is uniform across the three shipped tiers");

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Rarity {
    Common,
    Rare,
    Epic,
}
