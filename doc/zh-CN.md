# WynnCraft 构建工具

[中文版简介](./doc/zh-CN.md)

## 简介

该工具是一个针对 Minecraft 服务器 [WynnCraft](https://wynncraft.com/) 的第三方工具，旨在在游戏中批量生成构建组合，以发现最适合的构建和最佳属性。

目前的工具包包括两个主要组件：构建批量生成工具和装备筛选工具。

## 功能

- [x] 如果缺少 `items.json`，自动获取最新版本。
- [x] 如果不存在 `config.toml`，自动获取。
- [x] 根据提供的装备列表遍历并组合构建，识别有效构建并将其存储在数据库中。
- [x] 设置有效性检查的条件，例如最低属性要求，以筛选不合格的构建。
- [x] 计算基本属性（hp、ehp、hpr、元素防御、属性点、法力偷取、法力恢复、生命偷取、生命恢复、行走速度）。
- [x] 将构建转换为 [WynnBuilder](https://hppeng-wynn.github.io/builder) 的 URL。
- [ ] 其他属性计算。
  - [x] dam_pct
  - [x] exp_bonus
  - [ ] cost
- [x] 对 Hive 装备进行合法性检查。
- [ ] 伤害计算。
  - [ ] 技能计算。
  - [ ] Tomes 计算。
  - [ ] Powders 计算。
- [x] 装备筛选工具，可从所有 WynnCraft 装备中提取合适的装备，便于创建装备列表。
- [x] 基于过去十次速度的移动平均时间来计算剩余时间。
- [x] 剩余需要处理的组合数。

## 分步设置指南 (Windows)

1. 从 [releases 页面](https://github.com/WynnBuilderTools/WynnBuilderTools/releases) 下载最新发布的版本。
2. 将其内容解压到您喜欢的位置，但请确保可以通过终端轻松访问该目录！
3. 一切就绪，打开终端并运行二进制文件！记得使用示例 `config.toml`，如果遇到 `search_item` 可执行文件的问题，请运行 `search_item -h` 以获得帮助！

## 分步设置指南 (Linux)

1. 从 [releases 页面](https://github.com/WynnBuilderTools/WynnBuilderTools/releases) 下载最新发布的版本。
2. 将其内容解压到您喜欢的位置，但请确保可以通过终端轻松访问该目录！
3. 一切就绪，打开终端并运行二进制文件！记得使用示例 `config.toml`，如果遇到 `search_item` 可执行文件的问题，请运行 `./search_item -h` 以获得帮助！

如果这些说明对您不起作用，可以随时 [创建新问题](https://github.com/WynnBuilderTools/WynnBuilderTools/issues/new/choose)，或者如果您认为分步指南需要更改，可以随时发起 pull request！

## 批量生成工具用户指南

批量生成工具包括构建生成器（`builder.exe`）、配置文件（`config.toml`）在 `config` 文件夹中、WynnCraft 物品数据文件（`items.json`）也在 `config` 文件夹中，以及数据库文件（`data.db`）在 `db` 文件夹中。
物品数据文件来自 [hppeng-wynn](https://github.com/hppeng-wynn/hppeng-wynn.github.io/tree/dev/data)。

最终的目录结构应如下所示：

binaries/

├── builder.exe

├── migrations/

  │   └── 01_create_builds_table.sql

├── assets/

  │   └── id_map.json

├── config/

  │   ├── config.toml

  │   └── items.json (从 v1.0.0 开始自动获取)

└── db/ (从 v0.5.0 开始自动生成)

  └── data.db (如果根目录有 `migrations` 文件夹，从 v0.5.0 开始自动生成)

所有交互都通过配置文件进行，如下所示：

```toml
# 配置文件主要用于两个功能：
# 1. 设置装备列表。
# 2. 定义筛选条件。
# 工具将遍历所有可能的装备组合，检查其属性点是否有效，并将有效的构建存储在数据库中。
# 如果生成过程较慢或结果中包含大量属性不合格的构建，可以应用筛选条件。
#
# 示例使用了 RawFish 的 [Idol 构建](https://hppeng-wynn.github.io/builder?v=4#8_0Au0K70r50Qr0OK0K20K40OH0Qf160e2I1S0e1g00010039I1004fI18180H0I0I0E0o--hOsKbv3)

[player] # 玩家属性设置
lvl = 106 # 等级
available_point = 200 # 可用属性点；这里不与等级挂钩，以兼容 tomes 提供的属性点
base_hp = 500 # 基础生命值；通常为 500

[hppeng] # hppeng 相关设置
url_prefix = "https://hppeng-wynn.github.io/builder/?v=8#"  # 生成的 URL 前缀
url_suffix = "00001004fI0z0z0+0+0+0+0---hOsKbv3"            # 生成的 URL 后缀；包括粉末、tomes 和技能；一旦支持这些计算就不再需要
db_path = "db/data.db"                                      # 数据库路径
migrations_path = "migrations"                              # 数据库迁移路径
log_builds = true                                           # 是否将构建日志输出到控制台；对调试有用
log_db_errors = true                                        # 是否将数据库错误日志输出到控制台；对调试有用
db_retry_count = 10                                         # 数据库操作的重试次数
# 运行此配置时生成的最终 URL 将如下所示：https://hppeng-wynn.github.io/builder/?v=4#8_0Au0K70r50Qr0OK0K20K40OH0Qf0P0e2I1Q0e1g00001004fI0z0z0+0+0+0+0---hOsKbv3

[threshold_first] # 第一级筛选条件；此处的属性首先计算，大多数构建可在此处筛选以提高速度
# 此文件中的可选配置项已注释掉；取消注释以应用
# min_hp = 0

[threshold_second] # 第二级筛选条件
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

[items] # 装备列表；除武器外，所有槽位均可指定多个物品
helmets = ["Blue Mask"]
chest_plates = ["Soulflare"]
leggings = ["Vaward"]
boots = ["Resurgence"]
rings = ["Diamond Hydro Ring", "Moon Pool Circlet"]
bracelets = ["Prowess"]
necklaces = ["Diamond Fusion Necklace"]
weapon = "Idol"

illegal_combinations = [ # 非法装备组合检查，用于检查不可同时存在的装备组合，如 Hive
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

设置好配置文件后，运行可执行程序即可生成build，期间会打印输出合法的build，类似下面这样：

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

build生成后会立即放入数据库，之后可以通过数据库直接进行筛选。
如果对数据库不熟悉，可以使用[DB Browser for SQLite (DB4S)](https://github.com/sqlitebrowser/sqlitebrowser)进行浏览与筛选。

## 装备筛选工具使用指南

装备筛选工具是一个命令行工具，主要功能是筛选出某些属性排序靠前的装备并打印出来，可以将打印结果复制到批量生成工具的配置文件中使用。

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

使用示例：

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
