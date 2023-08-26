# WynnCraft Build Batch Generation Tool

[中文版简介](./doc/zh-CN.md)

## Introduction

This tool is a third-party utility for the Minecraft server [WynnCraft](https://wynncraft.com/), designed to generate build combinations in bulk within the game, aiming to discover the most suitable builds with optimal attributes.

## Features

- [x] Traverse and combine builds based on the provided equipment list, identifying valid builds and storing them in a database.
- [x] Set conditions for validity checks, such as minimum attribute requirements, to filter out ineligible builds.
- [x]  Calculate basic attributes (hp, ehp, hpr, elemental defenses, attribute points, mana steal, mana regen, life steal, life regen, walk speed).
- [x] Convert builds into [hppeng-wynn](https://hppeng-wynn.github.io/builder) URLs.
- [ ] Additional attribute calculations.
- [ ] Implement legality checks for Hive equipment.
- [ ] Damage calculations.
  - [ ] Skill calculations.
  - [ ] Tomes calculations.
  - [ ] Powders calculations.
- [ ] Equipment filtering tool to extract fitting equipment from all WynnCraft equipment, facilitating the creation of equipment lists.

## User Guide

This tool includes an executable program and a configuration file. All interactions are handled through the configuration file. The configuration file is outlined as follows:

```toml
# The configuration file primarily serves two functions:
# 1. Set up the equipment list.
# 2. Define filtering conditions.
# The tool will traverse all possible equipment combinations, check if their attribute points are valid, and store valid builds in the database.
# If the generation process is slow or results in numerous builds with inadequate attributes, filtering conditions can be applied.
#
# The provided example uses RawFish's [Idol build](https://hppeng-wynn.github.io/builder?v=4#8_0Au0K70r50Qr0OK0K20K40OH0Qf160e2I1S0e1g00010039I1004fI18180H0I0I0E0o--hOsKbv3)

[player] # Player attribute settings
lvl = 106 # Level
available_point = 200 # Available attribute points; not tied to the level here for compatibility with points from tomes
base_hp = 500 # Base health points; typically 500

[hppeng] # hppeng related settings
url_refix = "https://hppeng-wynn.github.io/builder/?v=4#" # Prefix for generated URLs
url_suffix = "00001004fI0z0z0+0+0+0+0---hOsKbv3"          # Suffix for generated URLs; includes powders, tomes, and skills; not needed once these calculations are supported
# The final URL generated when running this configuration will look like this: https://hppeng-wynn.github.io/builder/?v=4#8_0Au0K70r50Qr0OK0K20K40OH0Qf0P0e2I1Q0e1g00001004fI0z0z0+0+0+0+0---hOsKbv3

[threshold_first] # First filtering threshold; attributes here are calculated first, and most builds can be filtered out here to improve speed
# Optional configuration items in this file are commented out; uncomment to apply
# min_hp = 0

[threshold_second] # Second filtering threshold
# min_hpr_raw = 0
# min_hpr_pct = 0
# min_mr = 0
# min_ls = 0
# min_ms = 0
# min_spd = 0
# min_sd_raw = 0
# min_sd_pct = 0
# min_hpr = 0

[threshold_third]
# min_earth_defense = 0
# min_thunder_defense = 0
# min_water_defense = 0
# min_fire_defense = 0
# min_air_defense = 0

[threshold_fourth]
# min_earth_point = 0
# min_thunder_point = 0
# min_water_point = 0
# min_fire_point = 0
# min_air_point = 0
# min_ehp = 0

[items] # Equipment list; multiple items can be specified for all slots except weapon
helmets = ["Cumulonimbus"]
chestplates = ["Soulflare"]
leggings = ["Vaward"]
boots = ["Resurgence"]
rings = ["Diamond Hydro Ring", "Moon Pool Circlet"]
bracelets = ["Prowess"]
necklaces = ["Diamond Fusion Necklace"]
weapon = "Idol"
```

Once the configuration file is set up, running the executable program will generate builds. Valid builds will be printed during the process, similar to the following:

```text
helmet:1
chestplate:1
leggings:1
boots:1
bracelet:1
necklace:1
rings:2
total combinations: 3
https://hppeng-wynn.github.io/builder/?v=4#8_0Au0K70r50Qr0OK0K20K40OH0Qf0P0e2I1Q0e1g00001004fI0z0z0+0+0+0+0---hOsKbv3
max_stat:(mr:80, ms:14, spd:23, ls:440, hpr_raw:507, hpr_pct:20, sd_raw:343, sd_pct:15)
max_hpr:608
max_hp:14975
max_ehp:42217
skill_point:
assign:         earth:10        thunder:15      water:52        fire:65 air:15
original:       earth:25        thunder:40      water:146       fire:90 air:40
max_def:        earth:85        thunder:290     water:-30       fire:15 air:79
...
done
```

Generated builds are immediately stored in the database, allowing direct filtering through the database. If you're unfamiliar with databases, you can use [DB Browser for SQLite (DB4S)](https://github.com/sqlitebrowser/sqlitebrowser) to browse and filter the results.
