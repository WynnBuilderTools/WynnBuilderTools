mod args;

use crate::load_from_json;
use args::item_search_args::*;
use build_config::load_config;
use clap::Parser;
use itertools::Itertools;
use wynn_build_tools::*;

#[tokio::main]
async fn main() {
    let config = load_config("config/config.toml").await.unwrap();

    let args = ItemSearchArgs::parse();
    let (mut apparels, _) = match load_from_json(&config.hppeng.items_file) {
        Ok(v) => v,
        Err(_) => {
            let api_fetch_attempt =
                fetch_json_from_config(&config.hppeng.items_file, &config).await;

            let new_path = match api_fetch_attempt {
                Ok(v) => v,
                Err(e) => panic!("{}", e),
            };

            let second_attempt = load_from_json(&new_path);

            match second_attempt {
                Ok(v) => v,
                Err(e) => panic!("{}", e),
            }
        }
    };

    let reverse = match args.order_by {
        OrderBy::Asc => false,
        OrderBy::Desc => true,
    };

    // Filter apparels based on min and max values
    apparels.iter_mut().for_each(|v| {
        v.retain(|item| {
            args.min_values
                .iter()
                .all(|(sort_by, min_value)| sort_by.get_value(item) >= *min_value)
        });

        v.retain(|item| {
            args.max_values
                .iter()
                .all(|(sort_by, max_value)| sort_by.get_value(item) <= *max_value)
        });
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

// Function to compare two items based on a single SortAndFilterBy criterion
fn compare_items(a: &Apparel, b: &Apparel, sort_by: SortAndFilterBy) -> std::cmp::Ordering {
    match sort_by {
        SortAndFilterBy::Lvl => a.lvl.cmp(&b.lvl),
        SortAndFilterBy::Hp => a.hp.cmp(&b.hp),
        SortAndFilterBy::Hpb => a.hp_bonus_max.cmp(&b.hp_bonus_max),
        SortAndFilterBy::HprRaw => a
            .common_stat_max
            .hpr_raw()
            .cmp(&b.common_stat_max.hpr_raw()),
        SortAndFilterBy::HprPct => a
            .common_stat_max
            .hpr_pct()
            .cmp(&b.common_stat_max.hpr_pct()),
        SortAndFilterBy::SpAdd => a.add.all().cmp(&b.add.all()),
        SortAndFilterBy::SpReq => a.req.all().cmp(&b.req.all()),
        SortAndFilterBy::SdRaw => a.common_stat_max.sd_raw().cmp(&b.common_stat_max.sd_raw()),
        SortAndFilterBy::SdPct => a.common_stat_max.sd_pct().cmp(&b.common_stat_max.sd_pct()),
        SortAndFilterBy::Mr => a.common_stat_max.mr().cmp(&b.common_stat_max.mr()),
        SortAndFilterBy::Spd => a.common_stat_max.spd().cmp(&b.common_stat_max.spd()),
        SortAndFilterBy::Ls => a.common_stat_max.ls().cmp(&b.common_stat_max.ls()),
        SortAndFilterBy::Ndmg => a.dam_pct_max.n().cmp(&b.dam_pct_max.n()),
        SortAndFilterBy::Edmg => a.dam_pct_max.e().cmp(&b.dam_pct_max.e()),
        SortAndFilterBy::Tdmg => a.dam_pct_max.t().cmp(&b.dam_pct_max.t()),
        SortAndFilterBy::Wdmg => a.dam_pct_max.w().cmp(&b.dam_pct_max.w()),
        SortAndFilterBy::Fdmg => a.dam_pct_max.f().cmp(&b.dam_pct_max.f()),
        SortAndFilterBy::Admg => a.dam_pct_max.a().cmp(&b.dam_pct_max.a()),
        SortAndFilterBy::ExpB => a.sec_stat_max.exp_bonus().cmp(&b.sec_stat_max.exp_bonus()),
        SortAndFilterBy::LootBonus => a
            .sec_stat_max
            .loot_bonus()
            .cmp(&b.sec_stat_max.loot_bonus()),
    }
}
