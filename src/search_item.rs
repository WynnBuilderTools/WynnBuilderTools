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

    apparels.iter_mut().for_each(|v| {
        v.retain(|item| item.lvl >= args.min_lvl.into() && item.lvl <= args.max_lvl.into());
    });

    // Sort apparels based on multiple sort_by criteria
    for apparel_list in &mut apparels {
        apparel_list.sort_by(|a, b| {
            let mut ordering = std::cmp::Ordering::Equal;
            for &sort_key in &args.sort_by {
                ordering = compare_items(a, b, sort_key);
                if ordering != std::cmp::Ordering::Equal {
                    break;
                }
            }
            if reverse {
                ordering.reverse()
            } else {
                ordering
            }
        });
    }

    // Apply the limit
    let limit = args.limit as usize;
    for apparel_list in &mut apparels {
        if apparel_list.len() > limit {
            apparel_list.truncate(limit);
        }
    }

    // Print the results based on the type
    match args.r#type {
        Some(v) => {
            let apparels = match v {
                Type::Helmets => (&apparels[0], "Helmets"),
                Type::ChestPlate => (&apparels[1], "Chestplates"),
                Type::Leggings => (&apparels[2], "Leggings"),
                Type::Boots => (&apparels[3], "Boots"),
                Type::Ring => (&apparels[4], "Ring"),
                Type::Bracelet => (&apparels[5], "Bracelet"),
                Type::Necklace => (&apparels[6], "Necklace"),
            };
            let apparels_str = apparels
                .0
                .iter()
                .map(|v| format!("\"{}\"", v.name))
                .join(",");
            println!("{}:\t{}", apparels.1, apparels_str);
        }
        None => {
            let apparels_str: Vec<String> = apparels
                .iter()
                .map(|v| v.iter().map(|v| format!("\"{}\"", v.name)).join(","))
                .collect();
            println!("Helmets:\t{}", apparels_str[0]);
            println!("Chestplates:\t{}", apparels_str[1]);
            println!("Leggings:\t{}", apparels_str[2]);
            println!("Boots:\t\t{}", apparels_str[3]);
            println!("Ring:\t\t{}", apparels_str[4]);
            println!("Bracelet:\t{}", apparels_str[5]);
            println!("Necklace:\t{}", apparels_str[6]);
        }
    };
}

// Function to compare two items based on a single SortBy criterion
fn compare_items(a: &Apparel, b: &Apparel, sort_by: SortBy) -> std::cmp::Ordering {
    match sort_by {
        SortBy::LVL => a.lvl.cmp(&b.lvl),
        SortBy::HP => a.hp.cmp(&b.hp),
        SortBy::HPB => a.hp_bonus_max.cmp(&b.hp_bonus_max),
        SortBy::HPRRaw => a.stat_max.hpr_raw().cmp(&b.stat_max.hpr_raw()),
        SortBy::HPRPct => a.stat_max.hpr_pct().cmp(&b.stat_max.hpr_pct()),
        SortBy::SPAdd => a.add.all().cmp(&b.add.all()),
        SortBy::SPReq => a.req.all().cmp(&b.req.all()),
        SortBy::SDRaw => a.stat_max.sd_raw().cmp(&b.stat_max.sd_raw()),
        SortBy::SDPct => a.stat_max.sd_pct().cmp(&b.stat_max.sd_pct()),
        SortBy::MR => a.stat_max.mr().cmp(&b.stat_max.mr()),
        SortBy::SPD => a.stat_max.spd().cmp(&b.stat_max.spd()),
        SortBy::LS => a.stat_max.ls().cmp(&b.stat_max.ls()),
        SortBy::NDMG => a.dam_pct_max.n().cmp(&b.dam_pct_max.n()),
        SortBy::EDMG => a.dam_pct_max.e().cmp(&b.dam_pct_max.e()),
        SortBy::TDMG => a.dam_pct_max.t().cmp(&b.dam_pct_max.t()),
        SortBy::WDMG => a.dam_pct_max.w().cmp(&b.dam_pct_max.w()),
        SortBy::FDMG => a.dam_pct_max.f().cmp(&b.dam_pct_max.f()),
        SortBy::ADMG => a.dam_pct_max.a().cmp(&b.dam_pct_max.a()),
        SortBy::EXPB => a.max_exp_bonus.cmp(&b.max_exp_bonus),
    }
}
