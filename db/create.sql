CREATE TABLE race (
    race_id INTEGER PRIMARY KEY,
    race_date TEXT,
    racecource TEXT,
    posttime TEXT,
    change TEXT,
    race_type TEXT,
    race_name TEXT,
    class TEXT,
    corse TEXT,
    weather TEXT,
    going TEXT,
    horse_count TEXT,
    created_at TEXT NOT NULL DEFAULT (DATETIME('now', 'localtime')),
    updated_at TEXT NOT NULL DEFAULT (DATETIME('now', 'localtime'))
);

CREATE TRIGGER trigger_test_updated_at AFTER UPDATE ON race
BEGIN
    UPDATE test SET updated_at = DATETIME('now', 'localtime') WHERE rowid == NEW.rowid;
END;