
CREATE TABLE
	IF NOT EXISTS damage (
        name TEXT NOT NULL,
        normal INTEGER NOT NULL,
        crit INTEGER NOT NULL,
        avg INTEGER NOT NULL,
        build_id INTEGER NOT NULL,
        FOREIGN KEY (build_id) REFERENCES build(row_id)
    );