mod charset;
mod dictionary;
mod guess;
mod state;
mod templates;
mod word;

use axum::{
    extract::State,
    routing::{get, post},
    Form, Router,
};
use maud::{html, Markup};
use serde::Deserialize;
use state::{GameState, Input};
use std::sync::{Arc, RwLock};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

type SharedState = Arc<RwLock<GameState>>;

async fn page(State(state): State<SharedState>) -> Markup {
    let state = state.read().unwrap();
    return templates::page(
        "Wordle",
        html! {
            form id="form" method="post" hx-post="/api/input" hx-target="#game" hx-swap="outerHTML" {
                input type="hidden" name="key" id="key";
            }
            h1 { "Wordlx"}
            (templates::game_board(&state))

            div.panel {
                button hx-get="/cheat" hx-target="#cheat"  { "Cheat" }
            }
            div #cheat {}

            script src="assets/wordle.js" {}
        },
    );
}

async fn reset(State(state): State<SharedState>) -> Markup {
    let mut state = state.write().unwrap();
    *state = GameState::new_random();
    templates::game_board(&state)
}

#[derive(Deserialize)]
struct InputParams {
    key: String,
}
async fn input(State(state): State<SharedState>, Form(param): Form<InputParams>) -> Markup {
    let mut state = state.write().unwrap();
    if param.key == "enter" {
        state.input(Input::Enter);
    } else if param.key == "backspace" {
        state.input(Input::Backspace);
    } else {
        state.input(Input::Character(param.key.chars().next().unwrap()));
    }
    templates::game_board(&state)
}

async fn cheat(State(state): State<SharedState>) -> Markup {
    let state = state.read().unwrap();

    if state.phase != state::Phase::Playing {
        return html! {};
    }

    let mut filter = guess::WordFilter::new(state.answer);
    for guess in &state.guesses {
        filter.apply(*guess);
    }

    let choices = dictionary::WORDS
        .iter()
        .filter(|w| filter.matches(w))
        .collect::<Vec<_>>();

    html! {
        h2 { (choices.len()) " choices" }
        (templates::guess_table(html! {
            @for word in choices {
                (templates::guess_row(word.clone(), filter.correct, filter.required, false))
            }
        }))
    }
}

#[tokio::main]
async fn main() {
    let shared_state: SharedState = Arc::new(RwLock::new(GameState::new_random()));

    let app = Router::new()
        .route("/", get(page))
        .route("/cheat", get(cheat))
        .route("/api/input", post(input))
        .route("/api/reset", post(reset))
        .nest_service("/assets", ServeDir::new("assets"))
        .with_state(shared_state);

    let port = 8080;
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
