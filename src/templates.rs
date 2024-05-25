use maud::{html, Markup};

use crate::{
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
                script defer src="https://cdn.jsdelivr.net/npm/alpinejs@3.x.x/dist/cdn.min.js" {}
                script src="assets/main.js" {}
                link rel="stylesheet" href="assets/style.css";
                link rel="preconnect" href="https://fonts.googleapis.com";
                link rel="preconnect" href="https://fonts.gstatic.com" crossorigin;
                link href="https://fonts.googleapis.com/css2?family=Montserrat:wght@400..700&display=swap" rel="stylesheet";
            }
            body x-data x-on:keydown="submitKey($event, $refs.key, $refs.form)" {
                (content)
            }
        }
    }
}

pub fn game_board(state: &GameState) -> Markup {
    html! {
        div id="game" {
            table {
                @for guess in &state.guesses {
                    (guess_row(guess.clone(), state.answer, true))
                }
                @if !state.full() {
                    (guess_row(state.guess, Word::invalid(), false))
                    @for _ in 0..5 - state.guesses.len() {
                        (guess_row(Word::empty(),Word::invalid(), false))
                    }
                }
            }
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

fn guess_row(guess: Word, answer: Word, fixed: bool) -> Markup {
    html! {
        tr .guess {
            @for (i,c) in guess.into_iter().enumerate() {
                (guess_cell(i, c, answer, fixed))
            }
        }
    }
}

fn guess_cell(index: usize, char: char, answer: Word, fixed: bool) -> Markup {
    let exists = answer.contains(char);
    let correct = char == answer.at(index);
    html! {
        td .fixed[fixed] .exists[exists] .correct[correct] valign="middle" {
            (char)
        }
    }
}
