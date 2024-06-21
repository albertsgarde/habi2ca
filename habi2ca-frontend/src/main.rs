use habi2ca_common::player::Player;
use iced::{
    widget::{button, column, row, text},
    Sandbox,
};
use serde_json::json;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    AddXp,
}

pub struct PlayerWidget {
    player: Player,
    client: reqwest::blocking::Client,
}

impl Sandbox for PlayerWidget {
    type Message = Message;

    fn new() -> Self {
        let client = reqwest::blocking::Client::new();
        let player: Player = client
            .get("http://localhost:8080/api/player/1")
            .send()
            .unwrap()
            .json()
            .unwrap();
        Self { player, client }
    }

    fn title(&self) -> String {
        "Habi2ca".to_owned()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::AddXp => {
                let xp_delta = 1.0;
                self.client
                    .post("http://localhost:8080/api/player/1/add_xp")
                    .query(&json!({
                        "xp": xp_delta,
                    }))
                    .send()
                    .unwrap()
                    .error_for_status()
                    .unwrap();
                self.player = self
                    .client
                    .get("http://localhost:8080/api/player/1")
                    .send()
                    .unwrap()
                    .json()
                    .unwrap();
                println!("{}", self.player.xp());
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        column![
            text(format!("id: {}", self.player.id)),
            text(format!("Name: {}", self.player.data.name)),
            row![
                text(format!("XP: {}", self.player.data.xp)),
                button("Add XP").on_press(Message::AddXp)
            ]
        ]
        .into()
    }
}

pub fn main() -> iced::Result {
    PlayerWidget::run(iced::Settings::default())
}
