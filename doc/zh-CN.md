# WynnCraft build 工具包

## 简介

本工具是MC服务器[WynnCraft](https://wynncraft.com/)的第三方辅助工具，用于批量生成游戏中的build组合，以找到属性最合适的build。

当前的工具包包含两部分：build批量生成工具与装备筛选工具。

## 功能

- [x] 根据设置的装备列表遍历组合出所有build，找出所有合法的build储存至数据库
- [x] 设置合法性检查的条件，例如最低属性要求，对不合格的build进行过滤
- [x] 常规属性(hp、ehp、hpr、元素防御、属性点、魔力偷取、魔力恢复、生命偷取、生命恢复、走路速度)计算
- [x] 将build转换为[WynnBuilder](https://hppeng-wynn.github.io/builder) URL
- [ ] 更多的属性计算
  - [x] dam_pct
  - [ ] cost
- [ ] 添加Hive装备合法性检查
- [ ] 伤害计算
  - [ ] 技能计算
  - [ ] tomes计算
  - [ ] powders计算
- [x] 装备筛选工具，从wynn所有装备中筛选出符合要求的装备，方便编写装备列表

## 批量生成工具使用指南

批量生成工具包含一个build生成器(builder.exe)、一份配置文件(config.toml)、一份WynnCraft物品数据文件(items.json)、一份数据库文件(data.db)；
物品数据文件来自于[hppeng-wynn](https://github.com/hppeng-wynn/hppeng-wynn.github.io/tree/dev/data);
交互全部依靠配置文件完成，配置文件示例如下：

```toml
# 配置文件主要提供了两部分功能：
# 1. 设置装备列表
# 2. 设置筛选条件
# 工具会遍历组合装备列表中的所有可能组合，然后计算各个属性点是否合法，合法就储存到数据库中；
# 如果生成的太慢，或者有大量属性不符合要求的build被生成，就可以设置筛选条件。
#
# 示例中的装备使用了RawFish的[Idol build](https://hppeng-wynn.github.io/builder?v=4#8_0Au0K70r50Qr0OK0K20K40OH0Qf160e2I1S0e1g00010039I1004fI18180H0I0I0E0o--hOsKbv3)


[player] # 玩家属性设置
lvl = 106 # 等级
available_point = 200 # 可用的属性点；这里为了兼容tome增加的属性点，没有和等级挂钩
base_hp = 500 # 基础生命值；一般都是500

[hppeng] # hppeng相关设置
url_refix = "https://hppeng-wynn.github.io/builder/?v=4#" # 生成的URL前缀
url_suffix = "00001004fI0z0z0+0+0+0+0---hOsKbv3"          # 生成的URL后缀；包含了powders、tomes 和技能，以后如果支持了这些部分的计算，就不再需要人工填写了
# 运行此配置文件最终生成的URL会是这样: https://hppeng-wynn.github.io/builder/?v=4#8_0Au0K70r50Qr0OK0K20K40OH0Qf0P0e2I1Q0e1g00001004fI0z0z0+0+0+0+0---hOsKbv3

[threshold_first] # 第一道筛选条件；这里的属性会最先被计算，在这里把大部分build都筛选掉可以提升计算速度
# 本配置文件中注释掉的配置项都是可选的，取消注释即可应用
# min_hp = 0

[threshold_second] # 第二道筛选条件
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

[items] # 装备列表；除了武器外都可以指定多个
helmets = ["Cumulonimbus"]
chest_plates = ["Soulflare"]
leggings = ["Vaward"]
boots = ["Resurgence"]
rings = ["Diamond Hydro Ring", "Moon Pool Circlet"]
bracelets = ["Prowess"]
necklaces = ["Diamond Fusion Necklace"]
weapon = "Idol"
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

build生成后会立即放入数据库，之后可以通过数据库直接进行筛选。
如果对数据库不熟悉，可以使用[DB Browser for SQLite (DB4S)](https://github.com/sqlitebrowser/sqlitebrowser)进行浏览与筛选。

## 装备筛选工具使用指南

装备筛选工具是一个命令行工具，主要功能是筛选出某些属性排序靠前的装备并打印出来，可以将打印结果复制到批量生成工具的配置文件中使用。

--help:

``` txt
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

使用示例：

```txt
# InPut:
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
