use crate::go_board::*;
#[derive(Debug, Clone)]
pub struct StoneGroup {
    pub chain: Vec<Position>,
    pub color: Player,
    pub liberty: u8,
}
impl StoneGroup {
    pub fn new_single_move_group(pos: Position, color: Player)->Self {
        StoneGroup{chain:vec![pos], color:color, liberty:4}
    }
}

use std::fmt;
impl fmt::Display for StoneGroup {
    fn fmt(&self, io: &mut fmt::Formatter)->fmt::Result {
        let mut s = String::new();
        for i in 0..self.chain.len() {
            s.push('(');
            s.push_str(&self.chain[i].row.to_string());
            s.push(',');
            s.push_str(&self.chain[i].col.to_string());
            s.push(')');
        }
        use colored::*;
        if self.color == Player::Black {
            write!(io, "{}", s.blue())
        } else {
            write!(io, "{}", s.yellow())
        }
    }
}

// pub fn gen_group(board: GoBoard)->StoneGroup {
//     for (k,v) in board.state.into_iter() {

//     }
// }