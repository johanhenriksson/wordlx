mod charset;
mod dictionary;
mod state;
mod stats;
mod templates;
mod word;

use axum::{
    routing::{get, post},
    Form, Router,
};
use maud::{html, Markup};
use serde::Deserialize;
use state::{GameState, Input};
use std::time::Instant;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tower_sessions::{cookie::time::Duration, Expiry, MemoryStore, Session, SessionManagerLayer};

const STATE_KEY: &str = "game";

async fn page(session: Session) -> Markup {
    let state = session.get(STATE_KEY).await.unwrap().unwrap_or_default();

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

async fn reset(session: Session) -> Markup {
    let state = GameState::new_random();
    session.insert(STATE_KEY, state.clone()).await.unwrap();
    templates::game_board(&state)
}

#[derive(Deserialize)]
struct InputParams {
    key: String,
}
async fn input(session: Session, Form(param): Form<InputParams>) -> Markup {
    let mut state: GameState = session.get(STATE_KEY).await.unwrap().unwrap_or_default();
    if param.key == "enter" {
        state.input(Input::Enter);
    } else if param.key == "backspace" {
        state.input(Input::Backspace);
    } else {
        state.input(Input::Character(param.key.chars().next().unwrap()));
    }
    session.insert(STATE_KEY, state.clone()).await.unwrap();
    templates::game_board(&state)
}

async fn cheat(session: Session) -> Markup {
    let state: GameState = session.get(STATE_KEY).await.unwrap().unwrap_or_default();

    if state.phase != state::Phase::Playing {
        return html! {};
    }

    let mut filter = stats::WordFilter::new(state.answer);
    for guess in &state.guesses {
        filter.apply(*guess);
    }

    let start_match = Instant::now();
    let choices = dictionary::WORDS
        .iter()
        .filter(|w| filter.matches(**w))
        .collect::<Vec<_>>();
    println!("match took {:?}", start_match.elapsed());

    // find the choice which minimizes the number of remaining possibilities
    let start_score = Instant::now();
    let mut scored = choices
        .iter()
        .map(|choice| {
            let mut filter = filter.clone();
            filter.reject(**choice);
            let count = choices.iter().filter(|w| filter.matches(***w)).count();
            return (**choice, count);
        })
        .collect::<Vec<_>>();

    println!("score took {:?}", start_score.elapsed());
    println!("cheat took {:?}", start_match.elapsed());

    scored.sort_by(|a, b| a.1.cmp(&b.1));

    html! {
        h2 { (choices.len()) " choices" }
        (templates::guess_table(html! {
            @for (word, score) in scored.iter() {
                tr { td { (score) } }
                (templates::guess_row(word.clone(), filter.correct, filter.required, false))
            }
        }))
    }
}

#[tokio::main]
async fn main() {
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::seconds(3600)));

    let app = Router::new()
        .route("/", get(page))
        .route("/cheat", get(cheat))
        .route("/api/input", post(input))
        .route("/api/reset", post(reset))
        .nest_service("/assets", ServeDir::new("assets"))
        .layer(session_layer);

    let port = 8080;
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
