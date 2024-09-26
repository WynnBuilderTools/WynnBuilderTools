use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct ItemSearchArgs {
    /// Apparel type
    #[arg(short, long)]
    pub r#type: Option<r#Type>,

    /// A limit on the number of results, auto-inflated if the last item has the same value as multiple items
    #[arg(short, long, default_value_t = 10,value_parser = clap::value_parser!(u32).range(1..))]
    pub limit: u32,

    #[arg(short, long, default_value_t = OrderBy::DESC)]
    pub order_by: OrderBy,

    #[arg(short, long)]
    pub sort_by: SortBy,
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
    ASC,
    /// Sort the results in descending order, arrange them from largest to smallest
    DESC,
}
impl ToString for OrderBy {
    fn to_string(&self) -> String {
        match self {
            OrderBy::ASC => String::from("asc"),
            OrderBy::DESC => String::from("desc"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum SortBy {
    /// Level
    LVL,
    /// Hp
    HP,
    /// Hp bonus(max)
    HPB,
    /// Hp regain raw(max)
    HPRRaw,
    /// Hp regain pct(max)
    HPRPct,
    /// Skill point add total
    SPAdd,
    /// Skill point request total
    SPReq,
    /// Spell damage raw(max)
    SDRaw,
    /// Spell damage pct(max)
    SDPct,
    /// Mana regain(max)
    MR,
    /// Walk speed bonus(max)
    SPD,
    /// Life steal(max)
    LS,
    /// Exp bonus(max)
    EXPB,
}
