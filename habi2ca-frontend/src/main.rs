use habi2ca_common::player::Player;
use iced::{
    executor,
    futures::FutureExt,
    widget::{button, column, row, text, Text},
    Application, Command,
};
use reqwest::Client;
use serde_json::json;

#[derive(Debug, Clone)]
pub enum Message {
    ShowPlayer(Player),
    AddXp,
}

pub struct App {
    player: Option<Player>,
    client: Client,
}

impl Application for App {
    type Message = Message;

    type Executor = executor::Default;

    type Theme = iced::Theme;

    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let client = Client::new();
        let get_player = client
            .get("http://localhost:8080/api/players/1")
            .send()
            .then(|response| response.unwrap().json())
            .map(|player| Message::ShowPlayer(player.unwrap()));
        (
            Self {
                player: None,
                client,
            },
            Command::perform(get_player, |message| message),
        )
    }

    fn title(&self) -> String {
        "Habi2ca".to_owned()
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::AddXp => {
                let xp_delta = 1.0;
                let client = self.client.clone();
                let update_xp = client
                    .clone()
                    .post("http://localhost:8080/api/players/1/add_xp")
                    .query(&json!({
                        "xp": xp_delta,
                    }))
                    .send()
                    .then(move |response| {
                        response.unwrap().error_for_status().unwrap();
                        client.get("http://localhost:8080/api/players/1").send()
                    })
                    .then(|response| response.unwrap().json())
                    .map(|player| Message::ShowPlayer(player.unwrap()));
                Command::perform(update_xp, |message| message)
            }
            Message::ShowPlayer(player) => {
                self.player = Some(player);
                Command::none()
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        if let Some(player) = &self.player {
            column![
                text(format!("id: {}", player.id)),
                text(format!("Name: {}", player.data.name)),
                row![
                    text(format!("XP: {}", player.data.xp)),
                    button("Add XP").on_press(Message::AddXp)
                ]
            ]
            .into()
        } else {
            iced::Element::new(Text::new("Loading..."))
        }
    }
}

pub fn main() -> iced::Result {
    App::run(iced::Settings::default())
}
