use maud::{html, Markup};

use crate::{
    state::{GameState, Phase},
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
                link rel="stylesheet" href="assets/style.css" {}
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
            table cellspacing="5" {
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
            @if state.phase == Phase::Won {
                p { "You won!" }
            }
            @if state.phase == Phase::Lost {
                p { "You lost!" }
                p { "The answer was " (state.answer) }
            }
            @if state.phase == Phase::Playing {
                p { "Enter a guess" }
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
