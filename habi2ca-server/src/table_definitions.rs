pub const TABLES: &[&str] = &[PLAYER_TABLE, TASK_TABLE];

pub const PLAYER_TABLE: &str = r#"
CREATE TABLE player (
    id      INTEGER PRIMARY KEY AUTOINCREMENT,
    name    TEXT NOT NULL,
    xp      REAL NOT NULL,
    CHECK (xp >= 0.0)
) STRICT;
"#;

pub const TASK_TABLE: &str = r#"
CREATE TABLE task (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    player_id   INTEGER NOT NULL,
    name        TEXT NOT NULL,
    description TEXT NOT NULL,
    completed   INTEGER NOT NULL,
    CHECK (completed IN (0, 1)),
    FOREIGN KEY(player_id) REFERENCES player(id) ON DELETE CASCADE
) STRICT;
"#;
