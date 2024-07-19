mod application;

use anyhow::Result;
use application::{App, Config};
use clap::Parser;
use reqwest::Url;
use yew::Renderer;

#[derive(Parser, Clone, Debug)]
pub struct Cli {
    backend_url: Option<String>,
}

pub fn fallback_backend_url() -> Url {
    #[cfg(feature = "web")]
    {
        use web_sys;

        let backend_url = Url::parse(
            &web_sys::window()
                .expect("no window. Probably running outside of a browser.")
                .location()
                .origin()
                .expect("Failed to get url origin."),
        )
        .expect("Failed to parse origin as URL.");
        backend_url
    }
    #[cfg(not(feature = "web"))]
    {
        Url::parse("http://127.0.0.1:8080").expect("Failed to parse URL.")
    }
}

pub fn main() -> Result<()> {
    let cli = Cli::parse();
    let backend_url = cli
        .backend_url
        .map(|url| Url::parse(&url).expect("Failed to parse URL."))
        .unwrap_or_else(fallback_backend_url);
    Renderer::<App>::with_props(Config {
        server_url: backend_url,
    })
    .render();
    Ok(())
}
