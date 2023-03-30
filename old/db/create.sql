CREATE TABLE IF NOT EXISTS races (
    race_id INTEGER PRIMARY KEY,
    race_date TEXT NOT NULL,
    racecourse TEXT NOT NULL,
    race_num INTEGER NOT NULL,
    post_time TEXT,

    change TEXT,
    race_type TEXT,
    race_name TEXT,
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

CREATE TRIGGER IF NOT EXISTS trigger_races_updated_at AFTER UPDATE ON races
BEGIN
    UPDATE races SET updated_at = DATETIME('now', 'localtime') WHERE rowid == NEW.rowid;
END;

CREATE TABLE IF NOT EXISTS race_horses (
    race_horse_id INTEGER PRIMARY KEY,
    race_id INTEGER NOT NULL,
    horse_num INTEGER NOT NULL,
    bracket_num INTEGER,
    arrival INTEGER,
    horse_id INTEGER,
    horse_sex TEXT,
    horse_age INTEGER,
    weight_to_carry INTEGER,
    jockey_id INTEGER,
    trainer_id INTEGER,
    horse_weight INTEGER,
    horse_weight_delta INTEGER,
    finish_time REAL,
    margin_time REAL,
    margin TEXT,
    last_3f REAL,
    win_fav INTEGER,
    win_odds REAL,
    place_odds_min REAL,
    place_odds_max REAL,
    prize INTEGER,
    created_at TEXT NOT NULL DEFAULT (DATETIME('now', 'localtime')),
    updated_at TEXT NOT NULL DEFAULT (DATETIME('now', 'localtime'))
);

CREATE TRIGGER IF NOT EXISTS trigger_race_horses_updated_at AFTER UPDATE ON race_horses
BEGIN
    UPDATE race_horses SET updated_at = DATETIME('now', 'localtime') WHERE rowid == NEW.rowid;
END;

CREATE TABLE IF NOT EXISTS horses (
    horse_id INTEGER PRIMARY KEY,
    horse_name TEXT,
    sire_name TEXT,
    dam_name TEXT,
    sires_sire_name TEXT,
    sires_dam_name TEXT,
    dams_sire_name TEXT,
    dams_dam_name TEXT,
    breeder TEXT,
    birthplace TEXT,
    created_at TEXT NOT NULL DEFAULT (DATETIME('now', 'localtime')),
    updated_at TEXT NOT NULL DEFAULT (DATETIME('now', 'localtime'))
);

CREATE TRIGGER IF NOT EXISTS trigger_horses_updated_at AFTER UPDATE ON horses
BEGIN
    UPDATE horses SET updated_at = DATETIME('now', 'localtime') WHERE rowid == NEW.rowid;
END;

CREATE TABLE IF NOT EXISTS jockeys (
    jockey_id INTEGER PRIMARY KEY,
    jockey_name TEXT,
    jockey_sex TEXT,
    jockey_status TEXT,
    jockey_affiliation TEXT,
    created_at TEXT NOT NULL DEFAULT (DATETIME('now', 'localtime')),
    updated_at TEXT NOT NULL DEFAULT (DATETIME('now', 'localtime'))
);

CREATE TRIGGER IF NOT EXISTS trigger_jockeys_updated_at AFTER UPDATE ON jockeys
BEGIN
    UPDATE jockeys SET updated_at = DATETIME('now', 'localtime') WHERE rowid == NEW.rowid;
END;

CREATE TABLE IF NOT EXISTS trainers (
    trainer_id INTEGER PRIMARY KEY,
    trainer_name TEXT,
    trainer_sex TEXT,
    trainer_status TEXT,
    trainer_affiliation TEXT,
    created_at TEXT NOT NULL DEFAULT (DATETIME('now', 'localtime')),
    updated_at TEXT NOT NULL DEFAULT (DATETIME('now', 'localtime'))
);

CREATE TRIGGER IF NOT EXISTS trigger_trainers_updated_at AFTER UPDATE ON trainers
BEGIN
    UPDATE trainers SET updated_at = DATETIME('now', 'localtime') WHERE rowid == NEW.rowid;
END;