use habi2ca_common::{
    player::Player,
    task::{Task, TaskId},
};
use iced::{
    executor,
    futures::FutureExt,
    widget::{button, column, row, text, Column, Text},
    Application, Command,
};
use reqwest::{Client, Response, Url};
use serde_json::json;

async fn unwrap_response(response: Response) -> Response {
    if response.status().is_success() {
        response
    } else {
        panic!("Request failed: {}", response.text().await.unwrap())
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    UpdatePlayer(Player),
    AddXp,
    CreateTask,
    CompleteTask(TaskId),
    UpdateTasks(Vec<Task>),
}

pub struct Config {
    pub server_url: Url,
}

pub struct App {
    server_url: Url,
    player: Option<Player>,
    tasks: Vec<Task>,
    client: Client,
}

impl Application for App {
    type Message = Message;

    type Executor = executor::Default;

    type Theme = iced::Theme;

    type Flags = Config;

    fn new(config: Config) -> (Self, Command<Message>) {
        let client = Client::new();
        let Config { server_url } = config;
        let update_player = client
            .get(server_url.join("api/players/1").unwrap())
            .send()
            .then(|response| response.unwrap().json())
            .map(|player| Message::UpdatePlayer(player.unwrap()));
        let tasks_url = server_url.join("api/tasks").unwrap();
        let update_tasks = client
            .get(tasks_url)
            .send()
            .then(|response| response.unwrap().error_for_status().unwrap().json())
            .map(|tasks| Message::UpdateTasks(tasks.unwrap()));
        (
            Self {
                server_url,
                player: None,
                tasks: vec![],
                client,
            },
            Command::batch([
                Command::perform(update_player, |message| message),
                Command::perform(update_tasks, |message| message),
            ]),
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

                let add_xp_url = self.server_url.join("api/players/1/add_xp").unwrap();

                let update_xp = client
                    .clone()
                    .patch(add_xp_url)
                    .query(&json!({
                        "xp": xp_delta,
                    }))
                    .send()
                    .then(move |response| response.unwrap().error_for_status().unwrap().json())
                    .map(|player| Message::UpdatePlayer(player.unwrap()));
                Command::perform(update_xp, |message| message)
            }
            Message::UpdatePlayer(player) => {
                self.player = Some(player);
                Command::none()
            }
            Message::CreateTask => {
                let client = self.client.clone();
                let tasks_url = self.server_url.join("api/tasks").unwrap();
                let create_task = client
                    .clone()
                    .post(tasks_url.as_ref())
                    .json(&json!({
                        "player": 1,
                        "name": "Task1",
                        "description": "Description1",
                        "completed": false,
                    }))
                    .send()
                    .then(|response| unwrap_response(response.unwrap()))
                    .then(|response| response.json::<Task>())
                    .then(move |_task| client.get(tasks_url).send())
                    .then(|response| response.unwrap().error_for_status().unwrap().json())
                    .map(|tasks| Message::UpdateTasks(tasks.unwrap()));
                Command::perform(create_task, |message| message)
            }
            Message::CompleteTask(task_id) => {
                let client = self.client.clone();
                let complete_task_url = self
                    .server_url
                    .join(&format!("api/tasks/{task_id}/complete"))
                    .unwrap();
                let tasks_url = self.server_url.join("api/tasks").unwrap();
                let complete_task = client
                    .clone()
                    .patch(complete_task_url.as_ref())
                    .send()
                    .then(|response| unwrap_response(response.unwrap()))
                    .then(|response| response.json::<Task>())
                    .then(move |_task| client.get(tasks_url).send())
                    .then(|response| response.unwrap().error_for_status().unwrap().json())
                    .map(|tasks| Message::UpdateTasks(tasks.unwrap()));
                Command::perform(complete_task, |message| message)
            }
            Message::UpdateTasks(tasks) => {
                self.tasks = tasks;
                Command::none()
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        if let Some(player) = &self.player {
            let tasks = Column::with_children(
                self.tasks
                    .iter()
                    .filter(|task| !task.data.completed())
                    .flat_map(|task| {
                        [
                            text(format!("{}", task.data.name)).into(),
                            text(format!("    {}", task.data.description)).into(),
                            button("Complete")
                                .on_press(Message::CompleteTask(task.id))
                                .into(),
                        ]
                    }),
            );

            row![
                column![
                    text(format!("id: {}", player.id)),
                    text(format!("Name: {}", player.data.name)),
                    row![
                        text(format!("XP: {}", player.data.xp)),
                        button("Add XP").on_press(Message::AddXp)
                    ],
                    tasks,
                ],
                button("Create Task").on_press(Message::CreateTask)
            ]
            .into()
        } else {
            iced::Element::new(Text::new("Loading..."))
        }
    }
}

pub fn main() -> iced::Result {
    App::run(iced::Settings::with_flags(Config {
        server_url: Url::parse("http://127.0.0.1:8080").unwrap(),
    }))
}
