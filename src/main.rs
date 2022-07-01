mod go_board;
use go_board::*;

mod stone_group;
use stone_group::*;

mod gen_move;
use gen_move::*;

fn automatic_move() {
    let mut move_tree = MoveTree::new();
    move_tree.board.show();
    println!("Press enter to continue...");

    let mut color = Player::Black; // black first
    move_tree.place_a_move(color);
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input);
        if input == "\n" {
            color = color.next();
            move_tree.place_a_move(color);
            move_tree.board.show();
        }
    }
}

fn main() {
    automatic_move();
}
