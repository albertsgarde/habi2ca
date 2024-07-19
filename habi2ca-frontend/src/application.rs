use habi2ca_common::player::Player;
use reqwest::{Client, Url};
use yew::{function_component, html, Callback, Html, Properties};

#[derive(Properties, PartialEq, Clone)]
struct PlayerStatsProps {
    player: Option<Player>,
    on_add_xp: Callback<()>,
}

#[function_component(PlayerStats)]
fn player_stats(props: &PlayerStatsProps) -> Html {
    let PlayerStatsProps { player, on_add_xp } = props.clone();
    let on_add_xp = Callback::from(move |_| on_add_xp.emit(()));

    html! {
        <div>
            {player.map(|player| html!{
                <div>
                    <div>{format!("ID: {}", player.id)}</div>
                    <div>{format!("Name: {}", player.data.name)}</div>
                    <div>
                        {format!("XP: {}", player.data.xp)}
                        <button onclick={on_add_xp}>{"Add XP"}</button>
                    </div>
                </div>
            }).unwrap_or_else(|| html!{"Loading..."})}
        </div>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct Config {
    pub server_url: Url,
}

#[function_component(App)]
pub fn app(config: &Config) -> Html {
    let client = Client::new();
    let Config { server_url } = config;

    let player = yew::use_state_eq(|| None);
    {
        let player = player.clone();
        let client = client.clone();
        let server_url = server_url.clone();
        yew::use_effect_with((), move |_| {
            let player = player.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_player: Player = client
                    .get(server_url.join("api/players/1").unwrap())
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                player.set(Some(fetched_player));
            })
        })
    }

    let on_add_xp = {
        let player = player.clone();
        let server_url = server_url.clone();
        let client = client.clone();
        Callback::from(move |_| {
            let player = player.clone();
            let client = client.clone();

            let server_url = server_url.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let xp_delta = 1.0;
                let updated_player: Player = client
                    .patch(server_url.join("api/players/1/add_xp").unwrap())
                    .query(&[("xp", xp_delta)])
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                player.set(Some(updated_player));
            })
        })
    };

    html! {
    <>
        <PlayerStats player={(*player).clone()} on_add_xp={on_add_xp} />
    </>}
}

/*
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
                let task_data = TaskData {
                    player: self.player.as_ref().unwrap().id,
                    name: "Task1".to_owned(),
                    description: "Description1".to_owned(),
                    completed: false,
                };
                let create_task = client
                    .clone()
                    .post(tasks_url.as_ref())
                    .json(&task_data)
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
}*/
