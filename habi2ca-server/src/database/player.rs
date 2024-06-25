use super::Database;
use anyhow::{bail, Context, Result};
use habi2ca_common::player::{Player, PlayerData, PlayerId};
use tokio_rusqlite::{params, OptionalExtension};

impl Database {
    pub async fn create_player(&self, player_name: impl Into<String>) -> Result<PlayerId> {
        const CREATE_PLAYER: &str = "INSERT INTO player (name, xp) VALUES (?1, 0)";

        let player_name = player_name.into();

        self.connection
            .call_unwrap(move |connection| {
                let mut statement = connection.prepare(CREATE_PLAYER)?;

                statement
                    .insert([player_name.as_str()])
                    .with_context(move || {
                        format!(
                            "Failed to insert row for new player with name {}.",
                            player_name
                        )
                    })
                    .map(PlayerId)
            })
            .await
    }

    pub async fn get_player(&self, player_id: PlayerId) -> Result<Option<Player>> {
        const GET_PLAYER: &str = "SELECT name, xp FROM player WHERE id = ?1";

        self.connection
            .call_unwrap(move |connection| {
                let mut statement = connection.prepare(GET_PLAYER)?;
                statement
                    .query_row([player_id], |row| {
                        Ok(Player {
                            id: player_id,
                            data: PlayerData {
                                name: row.get(0)?,
                                xp: row.get(1)?,
                            },
                        })
                    })
                    .optional()
                    .with_context(|| {
                        format!("SQLite call to get player with id {} failed.", player_id)
                    })
            })
            .await
    }

    pub async fn add_xp(&self, player_id: PlayerId, xp_delta: f32) -> Result<()> {
        const ADD_XP: &str = "UPDATE player SET xp = xp + ?2 WHERE id = ?1";

        self.connection
            .call_unwrap(move |connection| {
                let mut statement = connection.prepare(ADD_XP)?;
                let num_rows_changed = statement
                    .execute(params![player_id, xp_delta])
                    .with_context(|| {
                        format!("Failed to add {xp_delta} XP to player with id {player_id}.",)
                    })?;
                match num_rows_changed {
                    1 => Ok(()),
                    0 => bail!("No player with id {player_id} exists."),
                    _ => bail!(
                        "Expected 1 row to be changed, but {num_rows_changed} rows were changed."
                    ),
                }
            })
            .await
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn create_player() {
        let database = Database::create_in_memory().await.unwrap();
        let player_id = database.create_player("Alice").await.unwrap();
        assert_eq!(player_id.0, 1);

        let player = database.get_player(player_id).await.unwrap().unwrap();
        assert_eq!(player.id.0, 1);
        assert_eq!(player.data.name, "Alice");
        assert_eq!(player.data.xp, 0.0);
    }

    #[tokio::test]
    async fn add_xp() {
        let database = Database::create_in_memory().await.unwrap();
        let player_id = database.create_player("Alice").await.unwrap();

        database.add_xp(player_id, 10.0).await.unwrap();
        let player = database.get_player(player_id).await.unwrap().unwrap();
        assert_eq!(player.data.xp, 10.0);

        database.add_xp(player_id, 0.0).await.unwrap();
        let player = database.get_player(player_id).await.unwrap().unwrap();
        assert_eq!(player.data.xp, 10.0);

        database.add_xp(player_id, 5.0).await.unwrap();
        let player = database.get_player(player_id).await.unwrap().unwrap();
        assert_eq!(player.data.xp, 15.0);
    }
}
