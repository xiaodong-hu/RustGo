use std::{collections::HashMap, fmt, slice::SliceIndex};

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
        GoBoard { state: empty_hash, groups: vec![], size: (9,9) } // set the default size of board here
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

    // get the neighbor positions for the given move: boundary effect is taken into
    fn get_neighbors(&self, pos:Position)->Vec<Position> {
        let mut neighbors = vec![];
        let x_max = self.size.0;
        let y_max = self.size.1;

        let pos_right = Position{row:pos.row+1, col:pos.col};
        let pos_left = Position{row:pos.row-1, col:pos.col};
        let pos_up = Position{row:pos.row, col:pos.col+1};
        let pos_down = Position{row:pos.row, col:pos.col-1};

        if pos.row < x_max && pos.row != 1 && pos.col < y_max && pos.col != 1 { // middle
            neighbors.push(pos_right);
            neighbors.push(pos_left);
            neighbors.push(pos_up);
            neighbors.push(pos_down);
        } else if pos.row == x_max && pos.col < y_max && pos.col != 1 { // right edge
            neighbors.push(pos_left);
            neighbors.push(pos_up);
            neighbors.push(pos_down);
        }  else if pos.row < x_max && pos.row != 1 && pos.col == y_max { // upper edge
            neighbors.push(pos_right);
            neighbors.push(pos_left);
            neighbors.push(pos_down);
        }  else if pos.row == 1 && pos.col < y_max && pos.col != 1 { // left edge
            neighbors.push(pos_right);
            neighbors.push(pos_down);
            neighbors.push(pos_up);
        }  else if pos.row < x_max && pos.row != 1 && pos.col == 1 { // lower edge
            neighbors.push(pos_right);
            neighbors.push(pos_left);
            neighbors.push(pos_up);
        } else if pos.row == 1 && pos.col == 1 { // down left corner
            neighbors.push(pos_right);
            neighbors.push(pos_up);
        } else if pos.row == x_max && pos.col == 1 { // down right corner
            neighbors.push(pos_left);
            neighbors.push(pos_up);
        } else if pos.row == x_max && pos.col == y_max { // upper right corner
            neighbors.push(pos_left);
            neighbors.push(pos_down);
        } else if pos.row == 1 && pos.col == y_max { // upper left corner
            neighbors.push(pos_right);
            neighbors.push(pos_down);
        }
        neighbors
    }

    fn refresh_groups(&mut self, pos:Position, color:Player) {
        let neighbors = self.get_neighbors(pos);

        let newly_connected_groups_indexes = self.groups.iter().
            enumerate().
            filter_map(|(index, group_hash)| // construct the vector of indexes of groups that touches the given move
                if {
                    let mut touched_or_not = false;
                    for p in &neighbors {
                        if group_hash.contains_key(&p) && group_hash.get(&p).unwrap() == &color {
                            touched_or_not = true;
                            break;
                        }
                    }
                    touched_or_not
                } {
                    Some(index)
                } else {
                    None
                }
            ).collect::<Vec<_>>();

        if newly_connected_groups_indexes.len() == 0 { // simply add a new single-stone group
            let mut single_stone_group = HashMap::new();
            single_stone_group.insert(pos, color);
            self.groups.push(single_stone_group)
        } else if newly_connected_groups_indexes.len() == 1 { // add to such group without any group merge
            self.groups[newly_connected_groups_indexes[0]].insert(pos, color);
        } else { // group merge occurs
            self.groups[newly_connected_groups_indexes[0]].insert(pos, color); // first, add to the first touching group the given move
            
            let old_group_hash_vec = self.groups.clone(); // closure only support single access to self.groups (see below); so an extra duplication is needed here
            for i in 0..newly_connected_groups_indexes.len() { // second, merge all (k,v) of other connected groups to the first touching group
                old_group_hash_vec[newly_connected_groups_indexes[i]].iter().
                    map(|(&k,&v)| self.groups[newly_connected_groups_indexes[0]].insert(k,v));
                
                self.groups.remove(newly_connected_groups_indexes[i]); // finally, remove the deeply copied groups from the Vec<HashMap<_>> 
            }
        }
    }

    pub fn play(&mut self, pos:Position, color:Player) {
        if self.move_is_legal(pos) {
            let mut single_stone_group = HashMap::new();
            single_stone_group.insert(pos, color);

            //println!("{:?}", neighbors_hash);
            if self.groups.len() == 0 {
                self.groups.push(single_stone_group);
            } else {
                self.refresh_groups(pos, color);
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

        self.show_stone_groups();
        Ok(())
    }
    
    pub fn show_stone_groups(&self) {
        println!("\t{} Groups of Stones Found on board:", self.groups.len());
        for group_hash in &self.groups {
            print!("\t - [");
            for (k,v) in group_hash {
                k.show(*v); 
            }
            println!("]");
        }
        println!();
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