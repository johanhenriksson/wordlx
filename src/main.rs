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
use axum_macros::debug_handler;
use maud::{html, Markup};
use serde::Deserialize;
use state::{GameState, Input};
use std::sync::{Arc, RwLock};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

use crate::{
    dictionary::WORDS,
    guess::{Charset, WordSpace},
};

type SharedState = Arc<RwLock<GameState>>;

async fn page(State(state): State<SharedState>) -> Markup {
    let state = state.read().unwrap();
    return templates::page(
        "Wordle",
        html! {
            form x-ref="form" method="post" hx-post="/api/input" hx-target="#game" hx-swap="outerHTML" {
                input type="hidden" name="key" x-ref="key";
            }
            h1 { "Wordlx"}
            (templates::game_board(&state))

            div.panel {
                button hx-get="/stats" hx-target="#cheat"  { "Cheat" }
            }
            div #cheat {}
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
#[debug_handler]
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

    let mut mask = Charset::all();
    let mut required = Charset::none();
    let mut space = WordSpace::new();
    let mut correct = vec![' ', ' ', ' ', ' ', ' '];
    let mut exists = vec![];

    for guess in &state.guesses {
        // update mask
        for (i, c) in guess.into_iter().enumerate() {
            if c == state.answer.at(i) {
                // correct character in correct position
                exists.push(c);
                correct[i] = c;
                required.include(c);
                space.only(i, c);
            } else if state.answer.contains(c) {
                // correct character in wrong position
                exists.push(c);
                required.include(c);
                space.exclude(i, c);
            } else {
                // incorrect character
                mask.exclude(c);
                for i in 0..5 {
                    space.exclude(i, c);
                }
            }
        }
    }

    let choices = WORDS
        .iter()
        .filter(|w| {
            let wm = w.charset();

            // ensure we dont have any rejected characters
            if !mask.contains(wm) {
                return false;
            }

            // ensure we have all required characters
            if required.intersects(wm.inverse()) {
                return false;
            }

            // ensure the word has no characters in known wrong positions
            space.matches(w)
        })
        .collect::<Vec<_>>();

    html! {
        h2 { (choices.len()) " choices" }
        (templates::guess_table(html! {
            @for word in choices {
                tr .guess {
                    @for (i,c) in word.into_iter().enumerate() {
                        @let exists = exists.contains(&c);
                        @let correct = c == correct[i];
                        (templates::guess_cell(c, false, exists, correct))
                    }
                }
            }
        }))
    }
}

#[tokio::main]
async fn main() {
    let shared_state: SharedState = Arc::new(RwLock::new(GameState::new_random()));

    let app = Router::new()
        .route("/", get(page))
        .route("/stats", get(cheat))
        .route("/api/input", post(input))
        .route("/api/reset", post(reset))
        .nest_service("/assets", ServeDir::new("assets"))
        .with_state(shared_state);

    let port = 8080;
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
