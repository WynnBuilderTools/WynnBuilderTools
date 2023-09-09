mod args;

use args::item_search_args::*;
use clap::Parser;
use itertools::Itertools;
use wynn_build_tools::*;

#[tokio::main]
async fn main() {
    let args = ItemSearchArgs::parse();
    let (mut apparels, _) = load_from_json("config/items.json");

    let reverse = match args.order_by {
        OrderBy::ASC => false,
        OrderBy::DESC => true,
    };
    let thresholds = get_threshold(&apparels, args.limit as usize - 1, reverse, |v| match args
        .sort_by
    {
        SortBy::LVL => v.lvl,
        SortBy::HP => v.hp,
        SortBy::HPB => v.hp_bonus_max,
        SortBy::HPRRaw => v.stat_max.hpr_raw() as i32,
        SortBy::HPRPct => v.stat_max.hpr_pct() as i32,
        SortBy::SPAdd => v.add.all() as i32,
        SortBy::SPReq => v.req.all() as i32,
        SortBy::SDRaw => v.stat_max.sd_raw() as i32,
        SortBy::SDPct => v.stat_max.sd_pct() as i32,
        SortBy::MR => v.stat_max.mr() as i32,
        SortBy::SPD => v.stat_max.spd() as i32,
        SortBy::LS => v.stat_max.ls() as i32,
    });

    filter_2d_vector(&mut apparels, |array_index, v| match args.sort_by {
        SortBy::LVL => v.lvl < thresholds[array_index],
        SortBy::HP => v.hp < thresholds[array_index],
        SortBy::HPB => v.hp_bonus_max < thresholds[array_index],
        SortBy::HPRRaw => (v.stat_max.hpr_raw() as i32) < thresholds[array_index],
        SortBy::HPRPct => (v.stat_max.hpr_pct() as i32) < thresholds[array_index],
        SortBy::SPAdd => (v.add.all() as i32) < thresholds[array_index],
        SortBy::SPReq => (v.req.all() as i32) < thresholds[array_index],
        SortBy::SDRaw => (v.stat_max.sd_raw() as i32) < thresholds[array_index],
        SortBy::SDPct => (v.stat_max.sd_pct() as i32) < thresholds[array_index],
        SortBy::MR => (v.stat_max.mr() as i32) < thresholds[array_index],
        SortBy::SPD => (v.stat_max.spd() as i32) < thresholds[array_index],
        SortBy::LS => (v.stat_max.ls() as i32) < thresholds[array_index],
    });

    match args.r#type {
        Some(v) => {
            let apparels = match v {
                Type::Helmets => &apparels[0],
                Type::ChestPlate => &apparels[1],
                Type::Leggings => &apparels[2],
                Type::Boots => &apparels[3],
                Type::Ring => &apparels[4],
                Type::Bracelet => &apparels[5],
                Type::Necklace => &apparels[6],
            };
            let apparels_str = apparels.iter().map(|v| format!("\"{}\"", v.name)).join(",");
            println!("{}", apparels_str);
        }
        None => {
            let apparels_str: [String; 7] =
                apparels.map(|v| v.iter().map(|v| format!("\"{}\"", v.name)).join(","));
            println!("Helmets:   {}", apparels_str[0]);
            println!("ChestPlat: {}", apparels_str[1]);
            println!("Leggings : {}", apparels_str[2]);
            println!("Boots:     {}", apparels_str[3]);
            println!("Ring:      {}", apparels_str[4]);
            println!("Bracelet:  {}", apparels_str[5]);
            println!("Necklace:  {}", apparels_str[6]);
        }
    };
}
