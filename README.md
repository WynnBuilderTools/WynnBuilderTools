# WynnCraft Build Tools

[中文版简介](./doc/zh-CN.md)

## Introduction

This tool is a third-party utility for the Minecraft server [WynnCraft](https://wynncraft.com/), designed to generate build combinations in bulk within the game, aiming to discover the most suitable builds with optimal attributes.

The current toolkit consists of two main components: the build batch generation tool and the equipment filtering tool.

## Features

- [x] Traverse and combine builds based on the provided equipment list, identifying valid builds and storing them in a database.
- [x] Set conditions for validity checks, such as minimum attribute requirements, to filter out ineligible builds.
- [x]  Calculate basic attributes (hp, ehp, hpr, elemental defenses, attribute points, mana steal, mana regen, life steal, life regen, walk speed).
- [x] Convert builds into [WynnBuilder](https://hppeng-wynn.github.io/builder) URLs.
- [ ] Additional attribute calculations.
  - [x] dam_pct
  - [ ] cost
- [x] Implement legality checks for Hive equipment.
- [ ] Damage calculations.
  - [ ] Skill calculations.
  - [ ] Tomes calculations.
  - [ ] Powders calculations.
- [x] Equipment filtering tool to extract fitting equipment from all WynnCraft equipment, facilitating the creation of equipment lists.
- [x] Remaining time based on moving average of last ten speeds
- [x] Remaining combinations to process

## Batch Generation Tool User Guide

The batch generation tool includes a build generator (builder.exe), a configuration file (config.toml) in the config folder, a WynnCraft item data file (items.json) again in the config folder, and a database file (data.db) in the db folder.
The item data file is sourced from [hppeng-wynn](https://github.com/hppeng-wynn/hppeng-wynn.github.io/tree/dev/data).

The final directory structure should look something like this:

binaries/

├── builder.exe

├── config/
  
  │   ├── config.toml

  │   └── items.json

└── db/

  └── data.db

All interactions are handled through the configuration file, as shown below:

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
url_prefix = "https://hppeng-wynn.github.io/builder/?v=8#" # Prefix for generated URLs
url_suffix = "00001004fI0z0z0+0+0+0+0---hOsKbv3"          # Suffix for generated URLs; includes powders, tomes, and skills; not needed once these calculations are supported
log_builds = false # Whether to log builds to the console; useful for debugging
log_db_errors = false # Whether to log database errors to the console; useful for debugging
db_retry_count = 10 # Number of retries for database operations
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
# min_neutral_dam_pct = 0
# min_earth_dam_pct = 0
# min_thunder_dam_pct = 0
# min_water_dam_pct = 0
# min_fire_dam_pct = 0
# min_air_dam_pct = 0

[threshold_fifth]
# min_earth_point = 0
# min_thunder_point = 0
# min_water_point = 0
# min_fire_point = 0
# min_air_point = 0
# min_ehp = 0

[items] # Equipment list; multiple items can be specified for all slots except weapon
helmets = ["Cumulonimbus"]
chest_plates = ["Soulflare"]
leggings = ["Vaward"]
boots = ["Resurgence"]
rings = ["Diamond Hydro Ring", "Moon Pool Circlet"]
bracelets = ["Prowess"]
necklaces = ["Diamond Fusion Necklace"]
weapon = "Idol"

illegal_combinations = [ # Illegal equipment combination inspections are used to check the equipment that cannot exist at the same time, such as Hive
    [
        "Abyss-Imbued Leggings",
        "Boreal-Patterned Crown",
        "Anima-Infused Cuirass",
        "Chaos-Woven Greaves",
        "Elysium-Engraved Aegis",
        "Eden-Blessed Guards",
        "Gaea-Hewn Boots",
        "Hephaestus-Forged Sabatons",
        "Obsidian-Framed Helmet",
        "Twilight-Gilded Cloak",
        "Infused Hive Relik",
        "Infused Hive Wand",
        "Infused Hive Spear",
        "Infused Hive Dagger",
        "Infused Hive Bow",
        "Contrast",
        "Prowess",
        "Intensity",
    ],
    [
        "Sparkling Visor",
        "Insulated Plate Mail",
        "Static-Charged Leggings",
        "Thunderous Step",
        "Bottled Thunderstorm",
        "Lightning Flash",
    ],
    [
        "Pride of the Aerie",
        "Gale's Freedom",
        "Turbine Greaves",
        "Flashstep",
        "Breezehands",
        "Vortex Bracer",
    ],
    [
        "Ambertoise Shell",
        "Beetle Aegis",
        "Elder Oak Roots",
        "Humbark Moccasins",
        "Subur Clip",
        "Golemlus Core",
    ],
    [
        "Whitecap Crown",
        "Stillwater Blue",
        "Trench Scourer",
        "Silt of the Seafloor",
        "Coral Ring",
        "Moon Pool Circlet",
    ],
    [
        "Sparkweaver",
        "Soulflare",
        "Cinderchain",
        "Mantlewalkers",
        "Clockwork",
        "Dupliblaze",
    ],
    [
        "Ornate Shadow Cowl",
        "Ornate Shadow Garb",
        "Ornate Shadow Cover",
        "Ornate Shadow Cloud",
    ],
    [
        "Dragon's Eye Bracelet",
        "Draoi Fair",
        "Renda Langit",
    ],
]
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

## Equipment Filtering Tool User Guide

The equipment filtering tool is a command-line utility designed to filter out equipment with specific attributes that rank high. It can print the results, which can then be copied into the batch generation tool's configuration file.

--help:

```txt
Usage: search_item [OPTIONS] --sort-by <SORT_BY>

Options:
  -t, --type <TYPE>
          Apparel type
          
          [possible values: helmets, chest-plate, leggings, boots, ring, bracelet, necklace]

  -l, --limit <LIMIT>
          A limit on the number of results, auto-inflated if the last item has the same value as multiple items
          
          [default: 10]

  -o, --order-by <ORDER_BY>
          [default: desc]

          Possible values:
          - asc:  Sort the results in ascending order, arrange them from smallest to largest
          - desc: Sort the results in descending order, arrange them from largest to smallest

  -s, --sort-by <SORT_BY>
          Possible values:
          - lvl:     Level
          - hpb:     Hp bonus(max)
          - hpr-raw: Hp regain raw(max)
          - hpr-pct: Hp regain pct(max)
          - sp-add:  Skill point add total
          - sp-req:  Skill point request total
          - sd-raw:  Spell damage raw(max)
          - sd-pct:  Spell damage pct(max)
          - mr:      Mana regain(max)
          - spd:     Walk speed bonus(max)
          - ls:      Life steal(max)

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

example:

```txt
# Input:
.\search_item.ext -s lvl

# OutPut:
Helmets:   "Dissociation","Aquamarine","Dissonance","Anima-Infused Helmet","Obsidian-Framed Helmet","Keratoconus","Ornate Shadow Cowl","Pisces","Nonexistence","Morph-Stardust"
ChestPlat: "Dondasch","Brilliant Diamond Chestplate","Atakebune","Gaping Cavity","Far Cosmos","Gravity","Boreal-Patterned Aegis","Elysium-Engraved Aegis","Twilight-Gilded Cloak","Empyreal Emberplate","Medeis","Inhibitor֎","Ornate Shadow Garb","Roridula","Wanderlust"
Leggings : "Anxiolytic","Anaerobic","Atomizer","Writhing Growth","Abyss-Imbued Leggings","Chaos-Woven Greaves","Hephaestus-Forged Greaves","Pyrrhic Respite","Ornate Shadow Cover","Neutrino","Aleph Null"
Boots:     "Capricorn","Curador Boots","Skidbladnir","Expedition's End","Fermion","Gaea-Hewn Boots","Hephaestus-Forged Sabatons","Kickback","Cytotoxic Striders","Revenant","Ornate Shadow Cloud","Wasteland Azalea"
Ring:      "Acid","Facile","Intensity","Azeotrope","Prism","Dispersion","Obstinance","Forbearance","Ingress","Tranquility"
Bracelet:  "Privateer","Enmity","Prowess","Knucklebones","Gravitron","Misalignment","Anya's Penumbra","Nebulous","Pandemonium","Compliance","Succession","Breakthrough","Detachment"
Necklace:  "Xebec","Ambivalence","Grafted Eyestalk","Contrast","Legendary Medallion","Planet Healer","Abrasion","Recalcitrance","Exhibition","Simulacrum","Reborn"
```
