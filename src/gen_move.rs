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
            let mut pos = Position { row: rng.gen_range(1..=board_size.0), col: rng.gen_range(1..=board_size.1)};
            if self.board.move_is_legal(pos) {
                return pos;
            } else {
                pos = Position { row: rng.gen_range(1..=board_size.0), col: rng.gen_range(1..=board_size.1)};
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
