use crate::go_board::*;
use rand::prelude::*;

pub struct MoveTree {
    step: i32,
    current_pos: Option<Position>,
    current_color: Option<Player>,
    pub board: GoBoard
}
impl MoveTree {
    pub fn new()->Self {
        MoveTree { step: 0, current_pos: None, current_color: None, board: GoBoard::new()}
    }

    pub fn gen_move_randomly(&self)->Position {
        let mut rng = thread_rng();
        let board_size = self.board.size;
       
        loop {
            let pos = Position { row: rng.gen_range(1..=board_size.0), col: rng.gen_range(1..=board_size.1)};
            if self.board.move_is_legal(pos) { // low-efficiency for almost-occupied board
                return pos;
            }
        }
    }

    pub fn place_a_move(&mut self, color:Player) {
        let pos = self.gen_move_randomly();
        self.board.play(pos, color);

        self.step += 1;
        self.current_pos = Some(pos);
        self.current_color = Some(color);
    }
}


pub fn automatic_play() {
    let mut move_tree = MoveTree::new();
    move_tree.board.show().ok();
    println!("Press [Enter] to gen_move; [g] to show stone groups; [q] to quit.");

    let mut color = Player::Black; // black first
    move_tree.place_a_move(color);
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).ok();
        if input == "\n" {
            color = color.next();
            move_tree.place_a_move(color);
            move_tree.board.show().ok();
        } else if input == "g\n" {
            move_tree.board.show_stone_groups();
        } else if input == "q\n" {
            std::process::exit(1);
        }
        println!("Press [Enter] to gen_move; [g] to show stone groups; [q] to quit.");
    }
}
