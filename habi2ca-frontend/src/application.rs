use anyhow::{Context, Result};
use dioxus::prelude::*;
use habi2ca_common::player::Player;
use reqwest::Url;

fn get_origin() -> Url {
    Url::parse(
        &web_sys::window()
            .expect("no window. Probably running outside of a browser.")
            .location()
            .origin()
            .expect("Failed to get url origin."),
    )
    .expect("Failed to parse origin as URL.")
}

#[derive(Clone, Debug, Routable, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
}

async fn fetch_player() -> Result<Player> {
    let url = get_origin().join("/api/players/1")?;
    let response = reqwest::get(url.as_ref()).await?;
    let player = response
        .json::<Player>()
        .await
        .with_context(|| format!("URL: {url:?}"))?;
    Ok(player)
}

#[component]
fn Player() -> Element {
    let player = use_resource(fetch_player);

    rsx! {
        match &*player.read() {
            Some(Ok(player)) => rsx!{
                div {
                    p { "Player" }
                    p { "Name: {player.name()}" }
                    p { "XP: {player.xp()}" }
                    button { "Add XP"}
                }
            },
            Some(Err(err)) => rsx!{
                div {
                    h1 { "Error" },
                    p { "Failed to fetch player: {err:?}" }
                }
            },
            None => rsx!{
                div {
                    h1 { "Loading..." }
                }
            }
        }
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        Player {}
    }
}

pub fn app() -> Element {
    rsx! {
        Router::<Route> {}
    }
}
