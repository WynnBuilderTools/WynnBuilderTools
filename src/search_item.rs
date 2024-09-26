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

    let min_lvl = args.min_lvl as i32;
    let max_lvl = args.max_lvl as i32;

    apparels.iter_mut().for_each(|v| {
        v.retain(|v| v.lvl >= min_lvl);
    });

    apparels.iter_mut().for_each(|v| {
        v.retain(|v| v.lvl <= max_lvl);
    });

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
        SortBy::EXPB => v.max_exp_bonus,
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
        SortBy::EXPB => v.max_exp_bonus < thresholds[array_index],
    });

    match args.r#type {
        Some(v) => {
            let apparels = match v {
            Type::Helmets => {
                (&apparels[0], "Helmets")
            },
            Type::ChestPlate => {
                (&apparels[1], "Chestplates")
            },
            Type::Leggings => {
                (&apparels[2], "Leggings")
            },
            Type::Boots => {
                (&apparels[3], "Boots")
            },
            Type::Ring => {
                (&apparels[4], "Rings")
            },
            Type::Bracelet => {
                (&apparels[5], "Bracelets")
            },
            Type::Necklace => {
                (&apparels[6], "Necklaces")
            },
            };
            let apparels_str = apparels.0.iter().map(|v| format!("\"{}\"", v.name)).join(",");
            println!("{}:\t{}", apparels.1, apparels_str);
        }
        None => {
            let apparels_str: [String; 7] =
                apparels.map(|v| v.iter().map(|v| format!("\"{}\"", v.name)).join(","));
            println!("Helmets:\t{}", apparels_str[0]);
            println!("Chestplate:\t{}", apparels_str[1]);
            println!("Leggings:\t{}", apparels_str[2]);
            println!("Boots:\t\t{}", apparels_str[3]);
            println!("Ring:\t\t{}", apparels_str[4]);
            println!("Bracelet:\t{}", apparels_str[5]);
            println!("Necklace:\t{}", apparels_str[6]);
        }
    };
}
