use std::fmt::Display;

use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about)]
pub struct ItemSearchArgs {
    /// Apparel type
    #[arg(short, long)]
    pub r#type: Option<r#Type>,

    /// A limit on the number of results, auto-inflated if the last item has the same value as multiple items
    #[arg(short, long, default_value_t = 10,value_parser = clap::value_parser!(u32).range(1..))]
    pub limit: u32,

    /// Minimum level
    #[arg(long, default_value_t = 1, value_parser = clap::value_parser!(u8).range(1..))]
    pub min_lvl: u8,

    /// Maximum level
    #[arg(long, default_value_t = 106, value_parser = clap::value_parser!(u8).range(1..))]
    pub max_lvl: u8,

    /// Order the results in ascending or descending order
    #[arg(short, long, default_value_t = OrderBy::Desc)]
    pub order_by: OrderBy,

    /// Sort the results by a specific field
    #[arg(short, long, num_args = 1..)]
    pub sort_by: Vec<SortBy>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum r#Type {
    Helmets,
    ChestPlate,
    Leggings,
    Boots,
    Ring,
    Bracelet,
    Necklace,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum OrderBy {
    /// Sort the results in ascending order, arrange them from smallest to largest
    Asc,
    /// Sort the results in descending order, arrange them from largest to smallest
    Desc,
}

impl Display for OrderBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderBy::Asc => write!(f, "asc"),
            OrderBy::Desc => write!(f, "desc"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum SortBy {
    /// Level
    Lvl,
    /// Hp
    Hp,
    /// Hp bonus(max)
    Hpb,
    /// Hp regain raw(max)
    HprRaw,
    /// Hp regain pct(max)
    HprPct,
    /// Skill point add total
    SPAdd,
    /// Skill point request total
    SPReq,
    /// Spell damage raw(max)
    SDRaw,
    /// Spell damage pct(max)
    SDPct,
    /// Mana regain(max)
    Mr,
    /// Walk speed bonus(max)
    Spd,
    /// Life steal(max)
    Ls,
    /// Exp bonus(max)
    ExpB,
    /// Neutral damage
    Ndmg,
    /// Earth damage
    Edmg,
    /// Thunder damage
    Tdmg,
    /// Water damage
    Wdmg,
    /// Fire damage
    Fdmg,
    /// Air damage
    Admg,
}
