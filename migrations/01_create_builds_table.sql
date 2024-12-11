CREATE TABLE
	IF NOT EXISTS build (
		row_id INTEGER PRIMARY KEY AUTOINCREMENT,
		url TEXT NOT NULL UNIQUE,
		helmet TEXT NOT NULL,
		chest_plate TEXT NOT NULL,
		leggings TEXT NOT NULL,
		boots TEXT NOT NULL,
		ring_1 TEXT NOT NULL,
		ring_2 TEXT NOT NULL,
		bracelet TEXT NOT NULL,
		necklace TEXT NOT NULL,
		earth_assign INTEGER NOT NULL,
		thunder_assign INTEGER NOT NULL,
		water_assign INTEGER NOT NULL,
		fire_assign INTEGER NOT NULL,
		air_assign INTEGER NOT NULL,
		earth_original INTEGER NOT NULL,
		thunder_original INTEGER NOT NULL,
		water_original INTEGER NOT NULL,
		fire_original INTEGER NOT NULL,
		ari_original INTEGER NOT NULL,
		earth_def INTEGER NOT NULL,
		thunder_def INTEGER NOT NULL,
		water_def INTEGER NOT NULL,
		fire_def INTEGER NOT NULL,
		air_def INTEGER NOT NULL,
		max_mr INTEGER NOT NULL,
		max_ms INTEGER NOT NULL,
		max_spd INTEGER NOT NULL,
		max_ls INTEGER NOT NULL,
		max_hpr_raw INTEGER NOT NULL,
		max_hpr_pct INTEGER NOT NULL,
		max_sd_raw INTEGER NOT NULL,
		max_sd_pct INTEGER NOT NULL,
		max_ehp INTEGER NOT NULL,
		max_hp INTEGER NOT NULL,
		max_hpr INTEGER NOT NULL,
		max_neutral_dam_pct INTEGER NOT NULL,
		max_earth_dam_pct INTEGER NOT NULL,
		max_thunder_dam_pct INTEGER NOT NULL,
		max_water_dam_pct INTEGER NOT NULL,
		max_fire_dam_pct INTEGER NOT NULL,
		max_air_dam_pct INTEGER NOT NULL,
		max_exp_bonus INTEGER NOT NULL,
		max_loot_bonus INTEGER NOT NULL
	);