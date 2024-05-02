#![allow(dead_code)]

use std::{cmp::min, cmp::max, i32::MAX, i32::MIN};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Board {
    cells: [[i32; 3]; 3]
}

impl Board {
    pub fn new() -> Self {
        Board { cells:[[0; 3]; 3] }
    }

    pub fn get(&self, i: usize, j: usize) -> i32 {
        self.cells[i][j]
    }

    pub fn play(&mut self, x: usize, y: usize, player: i32) -> bool {
        if x > 2 || y > 2 { return false; }
        if player != 1 && player != -1 { return false; }
        if self.cells[x][y] != 0 {return false;}

        self.cells[x][y] = player;

        true
    }

    pub fn check(&self) -> i32 {
        
        if self.cells[0][0] != 0 && self.cells[0][0] == self.cells[1][1] && self.cells[1][1] == self.cells[2][2] {
            return self.cells[0][0];
        } 
        
        if self.cells[0][2] != 0 && self.cells[0][2] == self.cells[1][1] && self.cells[1][1] == self.cells[2][0] {
            return self.cells[0][2];
        }
        
        for i in 0..3 {
            if self.cells[i][0] != 0 && self.cells[i][0] == self.cells[i][1] && self.cells[i][1] == self.cells[i][2] {
                return self.cells[i][0];
            }
        }

        for i in 0..3 {
            if self.cells[0][i] != 0 && self.cells[0][i] == self.cells[1][i] && self.cells[1][i] == self.cells[2][i] {
                return self.cells[0][i];
            }
        }

        0
    }

    pub fn available( &self ) -> Vec<(usize, usize)>{
        let mut v : Vec<(usize, usize)> = Vec::new();
        for i in 0..3 {
            for j in 0..3 {
                if self.cells[i][j] == 0 {v.push((i,j));}
            }
        }
        v
    }
}

impl ToString for Board {
    fn to_string(&self) -> String {
        let mut out = String::new();
        let sep = "-------------";
        out = format!("{}{}\n", out, sep);
        println!();
        for  i in 0..3 {
            out = format!(
                "{}\n| {} | {} | {} |\n{}", out, 
                if self.cells[i][0] == 1 {"X"} else if self.cells[i][0] == -1 {"O"} else {" "}, 
                if self.cells[i][1] == 1 {"X"} else if self.cells[i][1] == -1 {"O"} else {" "}, 
                if self.cells[i][2] == 1 {"X"} else if self.cells[i][2] == -1 {"O"} else {" "},
                sep
            );
        }
        out
    }
}

pub fn minmax(board: Board, max_player: bool, alpha: &mut i32, beta: &mut i32, depth: i32, level: i32, pos: &mut (i32, i32)) -> i32 {
    if depth == 0 {
        return 0;
    }
    let res = board.check();
    if res != 0 {
        return res;
    } else {
        let next = board.available();
        if next.len() == 0 { return 0; }
        if max_player {
            let mut value = MIN;
            for (i, j) in next {
                let mut new_board = board.clone();
                new_board.cells[i][j] = 1;
                let nvalue = minmax(new_board, !max_player, alpha, beta, depth-1, level+1, pos);
                if value < nvalue {
                    if level == 0 { *pos = (i as i32, j as i32); }
                    value = nvalue;
                }
                if value > *beta { break; }
                *alpha = max(*alpha, value);
            }
            return value;
        } else {
            let mut value = MAX;
            for (i, j) in next {
                let mut new_board = board.clone();
                new_board.cells[i][j] = -1;
                let nvalue = minmax(new_board, !max_player, alpha, beta, depth-1, level+1, pos);
                if value > nvalue {
                    if level == 0 {*pos = (i as i32, j as i32);}
                    value = nvalue;
                }
                if level == 0 {println!("{:?} {}", pos, value);}
                if value < *alpha {break;}
                *beta = min(*beta, value);
            }
            return value;
        }
    }
}