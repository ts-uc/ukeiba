CREATE TABLE race (
    race_id INTEGER PRIMARY KEY,
    race_date TEXT NOT NULL,
    racecource TEXT NOT NULL,
    race_num INTEGER NOT NULL,
    posttime TEXT,
    change TEXT,
    race_type TEXT,
    race_name TEXT,
    class TEXT,
    surface TEXT,
    direction TEXT,
    distance INTEGER,
    weather TEXT,
    going TEXT,
    moisture REAL,
    horse_count INTEGER,
    created_at TEXT NOT NULL DEFAULT (DATETIME('now', 'localtime')),
    updated_at TEXT NOT NULL DEFAULT (DATETIME('now', 'localtime'))
);

CREATE TRIGGER trigger_race_updated_at AFTER UPDATE ON race
BEGIN
    UPDATE race SET updated_at = DATETIME('now', 'localtime') WHERE rowid == NEW.rowid;
END;

CREATE TABLE result (
    race_horse_id INTEGER PRIMARY KEY,
    race_id INTEGER NOT NULL,
    horse_num INTEGER NOT NULL,
    bracket_num INTEGER,
    arrival INTEGER,
    horse_name TEXT,
    horse_id INTEGER,
    horse_affiliation TEXT,
    horse_sex TEXT,
    horse_age INTEGER,
    weight_to_carry INTEGER,
    jockey TEXT,
    jockey_id INTEGER,
    trainer TEXT,
    trainer_id INTEGER,
    horse_weight INTEGER,
    horse_weight_delta INTEGER,
    finish REAL,
    margin TEXT,
    three_furlongs REAL,
    win_fav INTEGER,
    created_at TEXT NOT NULL DEFAULT (DATETIME('now', 'localtime')),
    updated_at TEXT NOT NULL DEFAULT (DATETIME('now', 'localtime'))
);

CREATE TRIGGER trigger_result_updated_at AFTER UPDATE ON result
BEGIN
    UPDATE result SET updated_at = DATETIME('now', 'localtime') WHERE rowid == NEW.rowid;
END;