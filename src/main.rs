#![allow(non_snake_case)]

use std::i32::{MAX, MIN};

use dioxus::prelude::*;
use tracing::Level;

mod game;
use crate::game::*;

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Home {}
    }
}

#[derive(PartialEq, Props, Clone)]
struct CellProps {
    i: usize,
    j: usize,
}
#[derive(Clone, Copy)]
struct GameBoard {
    board: Board,
    state: i32,
    player: i32,
}

fn end(board: &mut Signal<GameBoard>) -> bool {
    if board.read().board.check() != 0 {
        board.write().state = 1;
        return true;
    } else {
        if board.read().board.available().len() == 0 {
            board.write().state = -1;
            return true;
        }
    }
    return false;
}

fn play(board: &mut Signal<GameBoard>, props: &CellProps) {

    let mut player = board.read().player;
    
    if board.write().board.play(props.i, props.j, player) {
        player *= -1;
    }
    
    if end(board) {return;}

    board.write().player = player;


    let (mut alpha, mut beta) = (MIN, MAX);
    let mut pos: (i32, i32) = (0, 0);

    minmax(board.write().board, false, &mut alpha, &mut beta, MAX, 0, &mut pos);
    board.write().board.play(pos.0 as usize, pos.1 as usize, player);
    
    player *= -1;

    if end(board) {return;}

    board.write().player = player;

}

#[component]
fn Cell(props: CellProps) -> Element {

    let mut board = use_context::<Signal<GameBoard>>();

    rsx! {
        button { 
            class: "w-24 h-24 text-center bg-orange-200 text-3xl",
            onclick: move |_| {
                play(&mut board, &props)
            },
            match board.read().board.get(props.i, props.j) {
                1 => {"X"},
                -1 => {"O"},
                _ =>  {""}
            },
        }
    }
}

#[component]
fn Game() -> Element {

    let board = use_context::<Signal<GameBoard>>();

    rsx! {
        if board.read().state == 0 {
            div {
                class: "grid grid-cols-3 gap-2",
                for i in 0..3 {
                    for j in 0..3 {
                        Cell { i, j }
                    }
                }
            }
        } else if board.read().state == 1 {
            h1 { "Player ", {if board.read().player == 1 {"1"} else {"2"}}, " wins!" }
        } else {
            h1 {"It's a draw!"}
        }
    }
}

#[component]
fn Home() -> Element {

    let mut board = use_context_provider(|| 
        Signal::new(GameBoard{
            board: Board::new(),
            state: 0,
            player: 1,
        })
    );

    rsx! {
        div {
            class: "flex flex-col items-center justify-evenly h-screen w-screen",
            Game {}
            button {
                class: "w-24 h-18 bg-orange-300",
                onclick: move |_| {
                    board.write().board = Board::new();
                    board.write().state = 0;
                    board.write().player = 1;
                },
                "Reset"
            }
        }
    }
}
