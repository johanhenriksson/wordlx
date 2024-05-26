use maud::{html, Markup};

use crate::{
    charset::Charset,
    state::{Error, GameState, Phase},
    word::Word,
};

pub fn page(title: &str, content: Markup) -> Markup {
    html! {
        (maud::DOCTYPE)
        html lang="en" {
            head {
                meta charset="UTF-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                title { (title) }
                script src="https://unpkg.com/htmx.org@1.9.12" {}
                link rel="stylesheet" href="assets/style.css";
                link rel="preconnect" href="https://fonts.googleapis.com";
                link rel="preconnect" href="https://fonts.gstatic.com" crossorigin;
                link href="https://fonts.googleapis.com/css2?family=Montserrat:wght@400..700&display=swap" rel="stylesheet";
            }
            body {
                div.content {
                    (content)
                }
            }
        }
    }
}

pub fn game_board(state: &GameState) -> Markup {
    let required = state.answer.charset();
    html! {
        div id="game" {
            (guess_table(html! {
                @for guess in &state.guesses {
                    (guess_row(guess.clone(), state.answer, required, true))
                }
                @if !state.full() {
                    (guess_row(state.guess.clone().into(), Word::empty(), Charset::none(), false))
                    @for _ in 0..5 - state.guesses.len() {
                        (guess_row(Word::empty(), Word::empty(), Charset::none(), false))
                    }
                }
            }))
            div.panel {
                @match state.error {
                    Error::None => {},
                    Error::InvalidGuess => p.message.error { "Invalid guess" },
                }
                @match state.phase {
                    Phase::Won => {
                        p.message { "You won!" }
                    },
                    Phase::Lost => {
                        p.message { "You lost!" }
                        p.message.small { "The answer was " span.word { (state.answer) } }
                    },
                    _ => {},
                }
                @if state.phase != Phase::Playing {
                    button hx-post="/api/reset" hx-target="#game" hx-swap="outerHTML" { "Play again" }
                }
            }
        }
    }
}

pub fn guess_table(content: Markup) -> Markup {
    html! {
        table.game {
            (content)
        }
    }
}

pub fn guess_row(guess: Word, correct: Word, required: Charset, fixed: bool) -> Markup {
    html! {
        tr .guess {
            @for (i,c) in guess.into_iter().enumerate() {
                @let exists = required.includes(c);
                @let correct = c == correct.at(i) && c != ' ';
                (guess_cell(c, fixed, exists, correct))
            }
        }
    }
}

fn guess_cell(char: char, fixed: bool, exists: bool, correct: bool) -> Markup {
    html! {
        td .fixed[fixed] .exists[exists] .correct[correct] valign="middle" {
            (char)
        }
    }
}
