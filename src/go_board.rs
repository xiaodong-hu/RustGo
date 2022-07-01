use std::{collections::HashMap, fmt};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    Black,
    White
}
impl Player {
    pub fn next(self)->Self {
        if self == Player::Black { Player::White } else { Player::Black }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Position {
    pub row:u8,
    pub col:u8   
}
impl Position {
    fn show(&self, color:Player) {
        use colored::*;
        if color == Player::Black {
            print!("({},{})", self.row.to_string().blue().bold(), self.col.to_string().blue().bold());
        } else {
            print!("({},{})", self.row.to_string().yellow(), self.col.to_string().yellow());
        }
    }
}

pub struct GoBoard {
    pub state:HashMap<Position, Player>, // hashmap of all stones on the board
    pub groups:Vec<HashMap<Position, Player>>, // hashmap for each group of stones; maybe better to split into struct of StoneGroup
    pub size:(u8,u8),
}
impl GoBoard {
    pub fn new()->Self {
        let empty_hash = HashMap::new(); // no stone: empty board and empty group
        GoBoard { state: empty_hash, groups: vec![], size: (19,19) }
    }

    // check if the move is in board
    fn in_board(&self, pos:Position)->bool {
        let x_max = self.size.0;
        let y_max = self.size.1;
        1 <= pos.row && pos.row <= x_max && 1 <= pos.col && pos.col <= y_max
    }
    
    // check if the position is occupied or not
    fn is_empty(&self, pos:Position)->bool {
        !self.state.contains_key(&pos)
    }

    // check if the move is legal (in board && empty && NOT instant komi)
    pub fn move_is_legal(&self, pos:Position)->bool {
        let in_board = self.in_board(pos);
        let is_empty = self.is_empty(pos);
        in_board && is_empty
    }

    pub fn play(&mut self, pos:Position, color:Player) {
        if self.move_is_legal(pos) {
            //let neighbors_hash = neighbors_for_current_move(self, pos);
            let mut current_move = HashMap::new();
            current_move.insert(pos, color);

            //println!("{:?}", neighbors_hash);
            if self.groups.len() == 0 {
                self.groups.push(current_move)
            } else {
                let mut indicator = 0; // indicator to see if there are groups contain such move
                for group_hash in &mut self.groups { // the only correct way to mutate the vec while iteration!
                    let move_is_contained_or_not: bool = {
                        let pos_right = Position{row:pos.row+1, col:pos.col};
                        let pos_left = Position{row:pos.row-1, col:pos.col};
                        let pos_up = Position{row:pos.row, col:pos.col+1};
                        let pos_down = Position{row:pos.row, col:pos.col-1};
                        group_hash.contains_key(&pos_right) || group_hash.contains_key(&pos_left) || group_hash.contains_key(&pos_up) || group_hash.contains_key(&pos_down)
                    };
                    if move_is_contained_or_not {
                        group_hash.insert(pos, color);
                        indicator += 1;
                        break;
                    }
                }
                if indicator == 0 {
                    self.groups.push(current_move)
                }
            }
            self.state.insert(pos, color);
        } // do nothing for illegal moves
    }

    pub fn show(&self)->std::fmt::Result {
        let x_max = self.size.0;
        let y_max = self.size.1;

        print!("\x1B[2J"); // a magic print to clear the terminal

        use colored::*;
        println!("{}", "\tCurrent board state:\n".bold());
        for y in (1..=y_max).rev() {
            let col_label = y;
            print!("\t{}\t", col_label.to_string().bold().green());
            for x in 1..=x_max {
                let current_move = Position {row:x, col:y};
                if self.state.contains_key(&current_move) {
                    match self.state.get(&current_move) {
                        Some(Player::Black) => {
                            if x<x_max {
                                print!(" {}","●".bold().blue())
                            } else {
                                println!(" {}","●".bold().blue()) // break line for stones on the boundary
                            }
                        },
                        Some(Player::White) => {
                            if x<x_max {
                                print!(" {}","●".bold().yellow())
                            } else {
                                println!(" {}","●".bold().yellow()) // break line for stones on the boundary
                            }
                        },
                        None => {}
                    }
                } else if x != x_max { print!(" +") } else { print!(" +\n") }
            }
        }
        println!();
        print!(" \t\t");
        for row_char in b'a'..=b's' { // print the row label as alphabet by the integer representation of utf-8 symbols
            print!(" {}", (row_char as char).to_string().bold().green());
        }
        println!("\n");
        println!("\t{} Groups of Stones Found on board:", self.groups.len());
        for group_hash in &self.groups {
            print!("\t - [");
            for (k,v) in group_hash {
                k.show(*v); 
            }
            println!("]");
        }
        println!();
        Ok(())
    }
}

// fn neighbors_for_current_move(board: &GoBoard, pos:Position)->HashMap<Position, Option<Player>> {
//     let x_max = board.size.0;
//     let y_max = board.size.1;

//     let mut possible_neighbors = HashMap::new();

//     if pos.row+1 <= x_max {
//         possible_neighbors.insert(Position{row:pos.row+1, col:pos.col}, None); // we do not need to specify the player for the neighbors
//     }
//     if pos.row-1 >= 1 {
//         possible_neighbors.insert(Position{row:pos.row-1, col:pos.col}, None);
//     }
//     if pos.col+1 <= y_max {
//         possible_neighbors.insert(Position{row:pos.row, col:pos.col+1}, None);
//     }
//     if pos.col-1 >= 1 {
//         possible_neighbors.insert(Position{row:pos.row, col:pos.col-1}, None);
//     }
//     possible_neighbors
// }