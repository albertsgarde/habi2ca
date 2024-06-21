pub const PLAYER_TABLE: &str = r#"
CREATE TABLE player (
    id      INTEGER PRIMARY KEY AUTOINCREMENT,
    name    TEXT NOT NULL,
    xp      REAL NOT NULL
    CHECK (xp >= 0.0)
) STRICT;
"#;
