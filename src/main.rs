mod go_board;
use go_board::*;

// mod stone_group;
// use stone_group::*;

mod gen_move;
use gen_move::*;

fn automatic_move() {
    let mut move_tree = MoveTree::new();
    move_tree.board.show();
    println!("Press [Enter] to gen_move; [g] to show stone groups; [q] to quit.");

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
        if input == "g" {
            move_tree.board.show_stone_groups();
        }
        println!("Press [Enter] to gen_move; [g] to show stone groups; [q] to quit.");
    }
}

fn main() {
    automatic_move();
    // let test = vec![1,2,3];
    // let filtered_test = test.into_iter().filter(|&x| x>1).collect::<Vec<i32>>();
    // // dbg!("{:?}", iter.next());
    // // dbg!("{:?}", iter.next());
    // dbg!(filtered_test);
}
