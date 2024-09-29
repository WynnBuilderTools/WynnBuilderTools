# WynnCraft Build Tools

[中文版简介](./doc/zh-CN.md)

## Introduction

This tool is a third-party utility for the Minecraft server [WynnCraft](https://wynncraft.com/), designed to generate build combinations in bulk within the game,
aiming to discover the most suitable builds with optimal attributes.

The current toolkit consists of two main components: the build batch generation tool and the equipment filtering tool.

## Features

- [x] Traverse and combine builds based on the provided equipment list, identifying valid builds and storing them in a database.
- [x] Set conditions for validity checks, such as minimum attribute requirements, to filter out ineligible builds.
- [x]  Calculate basic attributes (hp, ehp, hpr, elemental defenses, attribute points, mana steal, mana regen, life steal, life regen, walk speed).
- [x] Convert builds into [WynnBuilder](https://hppeng-wynn.github.io/builder) URLs.
- [ ] Additional attribute calculations.
  - [x] dam_pct
  - [x] exp_bonus
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

├── migrations/

  │   └── 01_create_builds_table.sql

├── config/
  
  │   ├── config.toml

  │   └── items.json

└── db/ (automatically generated from v0.5.0 onwards)

  └── data.db (automatically generated from v0.5.0 onwards given a migrations folder at root)

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
url_prefix = "https://hppeng-wynn.github.io/builder/?v=8#"  # Prefix for generated URLs
url_suffix = "00001004fI0z0z0+0+0+0+0---hOsKbv3"            # Suffix for generated URLs; includes powders, tomes, and skills; not needed once these calculations are supported
db_path = "db/data.db"                                      # Database path
migrations_path = "migrations"                           # Database migration path
log_builds = true                                           # Whether to log builds to the console; useful for debugging
log_db_errors = true                                        # Whether to log database errors to the console; useful for debugging
db_retry_count = 10                                         # Number of retries for database operations
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
# min_exp_bonus = 0

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
https://hppeng-wynn.github.io/builder/?v=8#8_0690K70r50Qr0OK0OK0K40OH0Qf0Q0Q351Y0Q1g00001004fI0z0z0+0+0+0+0---hOsKbv3
max_stat:(mr:74, ms:18, spd:-10, ls:440, hpr_raw:507, hpr_pct:20, sd_raw:343, sd_pct:-5)
max_hpr:608
max_hp:12751
max_ehp:34600
skill_point:
assign:         earth:0 thunder:0       water:100       fire:62 air:0
original:       earth:26        thunder:26      water:197       fire:98 air:26
max_def:        earth:15        thunder:-82     water:72        fire:290        air:15
max_dam_pct:    earth:0 thunder:0       water:20        fire:0  air:0   neutral:0
max_exp_bonus:  0
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

      --min-lvl <MIN_LVL>
          Minimum level
          
          [default: 1]

      --max-lvl <MAX_LVL>
          Maximum level
          
          [default: 106]

  -o, --order-by <ORDER_BY>
          Order the results in ascending or descending order
          
          [default: desc]

          Possible values:
          - asc:  Sort the results in ascending order, arrange them from smallest to largest
          - desc: Sort the results in descending order, arrange them from largest to smallest

  -s, --sort-by <SORT_BY>
          Sort the results by a specific field

          Possible values:
          - lvl:     Level
          - hp:      Hp
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
          - expb:    Exp bonus(max)

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

examples:

```txt
# Input:
.\search_item.ext -s lvl

# OutPut:
Helmets:        "Nonexistence","Fool's Errand","Nuclear Emesis","Inconceivably Deranged Paper Mask of Legendary Victory","Mesmerizing Madness","Transplanted Psyche","Outlandish Replica Face Mask of Legendary Victory","Ornate Shadow Cowl","Dissonance","Treasured Diamond Mask of Legendary Victory"
Chestplate:     "Null Plating","Roridula","Empyreal Emberplate","Ornate Shadow Garb","Wanderlust","Schadenfreude","Medeis","Dondasch","Twilight-Gilded Cloak","Elysium-Engraved Aegis","Atakebune","Brilliant Diamond Chestplate","Boreal-Patterned Aegis","Gravity","Far Cosmos","Gaping Cavity"
Leggings:       "Aleph Null","Anaerobic","Ornate Shadow Cover","Atomizer","Pyrrhic Respite","Anxiolytic","Chaos-Woven Greaves","Hephaestus-Forged Greaves","Writhing Growth","Neutrino","Abyss-Imbued Leggings"
Boots:          "Delusion","Ornate Shadow Cloud","Withstand","Acidosis","Expedition's End","Wasteland Azalea","Hephaestus-Forged Sabatons","Fermion","Gaea-Hewn Boots","Skidbladnir"
Ring:           "Ingress","Forbearance","Azeotrope","Tranquility","Obstinance","Dispersion","Prism","Intensity","Acid","Facile"
Bracelet:       "Detachment","Breakthrough","Misalignment","Black Space","Gravitron","Succession","Enmity","Anya's Penumbra","Compliance","Prowess"
Necklace:       "Simulacrum","Exhibition","Swindler's Charm","Grafted Eyestalk","Contrast","Planet Healer","Legendary Medallion","Abrasion","Xebec","Recalcitrance","Ambivalence","Reborn"
```

```txt
# Input:
.\search_item.ext -t ring -s expb

# Output:
Rings:  "Summa","Bronze Basic Ring","Detective's Ring","Ring of Generosity","Draoi Fair","Lodestone","Living Slime","Precious","Decoder Ring","Law of the Jungle","Rarity"
```

```txt
# Input:
.\search_item.ext -s hpr-raw

# Output:
Helmets:        "Morph-Stardust","Aquamarine","Cancer֎","Ophiolite","Skyfloat","Snail Helm","Azure Halo","Phoenix Prince's Crown","Sano's Care","Grillface"
Chestplate:     "Elysium-Engraved Aegis","Gravity","Sparkling Plate","Nether's Reach","Leo","Keeper of Souls","Darkiron Aegis","Dreamcloud","About-Face","Pristine Antiquity"
Leggings:       "Hephaestus-Forged Greaves","Ration","Philophilia","The Golem","Mycelium Plating","Black Lily","Anti-Causality","Elder Oak Roots","Horizon","Greaves of the Veneer"
Boots:          "Delusion","Withstand","Gaea-Hewn Boots","Curador Boots","Burnout","Crater Print","Boreal","Resurgence","Scorpio","Sempiternel"
Ring:           "Iron Will","Diamond Solar Ring","Silver Solar Ring","Gold Solar Ring","Bloodborne","Archaic","Cacophony","Fuse","Recovery","Vital","Draoi Fair"
Bracelet:       "Succession","Hamsey's Brilliance","Flashfire Gauntlet","Sacramentalia","Auric","Siwel's Guilt","Lazarus' Brace","Lasting","Great Brace","Ra"
Necklace:       "Contrast","Antim","Gigabyte","Ambiguity","Pulse Starter","Golemlus Core","Alkali","Mech Core","Adder Stone","Amulet of Rejuvenation"
```

```txt
# Input:
.\search_item.ext -s lvl --min-lvl 50 --max-lvl 80

# Output:
Helmets:        "Cosmic Visor","Dragon Horned Helmet","Green Helmet","Gale's Sight","Centipede","Sparkling Visor","Tephra","Hollow Virtue","Toxin","Rust","Rinkaku","Breakbeat"
Chestplate:     "Changeling's Chestplate","Bete Noire","Cosmic Vest","Traumerei","Reinforced Iron Chestplate","Screech","Future Shock Plating","Eleventh Hour","Pristine Antiquity","Endurance","Aura of Element","Marshmallow"
Leggings:       "Air Sanctuary","Xyloid","Earth Sanctuary","Ringlets","Water Sanctuary","Cosmic Ward","Fire Sanctuary","Thunder Sanctuary","Rainbow Sanctuary","The Prisoner","Leggings of Haste","Reinforced Iron Leggings"
Boots:          "Centennial","Ventus Tail","Missing","Cosmic Walkers","Dragulj Boots","Earthsky Eclipse","Sturdy","Black Sheep","Slipstream","Gert Boots","Reinforced Iron Boots","Determination","Scale of Sieryu","Corrupted Nii Mukluk","Lerteco"
Ring:           "Athanasia","Ring of Power","Time Ring","Cacophony","Ring of Wisdom","Rainbow","Clockwork Ring","Puff","Soldier","Rubber","Ghost","Martyr"
Bracelet:       "Auric","Veneration","Provenance","Tight Clamp","Flexing Chain","Lecade's Rank","Double Vision","Broken Gauntlet","Momentum","Example","Panic Attack"
Necklace:       "Dancer","Altum Spatium","Adder Stone","Asbestos","Metamorphosis","Amulet of Rejuvenation","Sterling Silver","Rough Diamond","Reckoning","Tenuto"
```

```txt
# Input:
.\search_item.ext -s expb

# Output:
Helmets:        "Sano's Care","Facedown","Speaker","Cosmic Visor","Green Helmet","Gale's Sight","Sparkling Visor","Restored Ancient Helmet","Penance","Clearsight Spectacles","Upgraded Orc Mask","Blueberry","Illuminite","Venison","Hero's Mask","Bloodied Wreath","Aeolus","Faded","Santa Hat","Sound of Silence"
Chestplate:     "Roridula","Diamond Dust","Tisaun's Valor","Dragon Hide Plate","Cosmic Vest","Aura of Element","Papyrus","The Jingling Jester","Matryoshka Shell","Geis"
Leggings:       "Trench Scourer","Bantisu's Approach","Ringlets","Cosmic Ward","Helios Lux","Egression","The Oblivious","Greaves of Honor","Bridge of the Divide","Moisture","Chained Pixels"
Boots:          "Memento","Ensa's Ideals","Cosmic Walkers","Champion Boots","Sodeta Boots","Bad Wolf","Durum's Journey","Seven-League Boots","Galloping Spurs","Prologue","Santa Boots","Snowtread Boots","Silken Slippers"
Ring:           "Summa","Bronze Basic Ring","Detective's Ring","Ring of Generosity","Draoi Fair","Lodestone","Living Slime","Precious","Decoder Ring","Law of the Jungle","Rarity"
Bracelet:       "Knucklebones","Follow The Wind","Synchro Core","Kayde","Jiandan Handwraps","Double Vision","Vanguard","Back-up Plan","Shackle of Shade","Rayshyroth's Knowledge","Dragon's Eye Bracelet","Homeorhesis","Binding Brace"
Necklace:       "Ominous Wind","Overload Core","Altum Spatium","Adder Stone","Gospel","Ensa's Faith","Trainer's Pendant","Renda Langit","Hexed Amulet","Criistal","Constrict Collar","Durum's Serenity"
```