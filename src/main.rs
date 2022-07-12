use yagoll::Board;

use std::{
    io::{self, Write},
    thread, time,
};

/// Return the ratio of alive to dead cells
fn board_alive_to_dead_ratio<WIDTH, HEIGHT>(board: &Board) -> f32 {
    todo!();
}

/// Return the "activity level" of the board (number of updates between cycles).
fn calc_board_activity(board: &Board) -> f32 {
    // Copy board
    let mut count = 0;
    let width = board.width;
    let height = board.height;
    let mut next = board.clone();
    next.advance_cycle();
    for x in 0..width {
        for y in 0..height {
            count += if board.get(x, y) != next.get(x, y) {
                1
            } else {
                0
            };
        }
    }

    count as f32 / (width as f32 * height as f32)
}

pub(crate) fn main() -> io::Result<()> {
    let mut board = Board::new_from_file("./input.txt");

    println!("{}", board);

    loop {
        let activity = calc_board_activity(&board);
        println!("Activity: {}", activity);
        board.advance_cycle();
        println!("{}", board);
        io::stdout().flush()?;
        thread::sleep(time::Duration::from_millis(1000));
    }
}
