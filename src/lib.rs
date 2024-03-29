pub use crate::config::*;

pub mod config;

use yagoll::{Board, Cell};

use std::error::Error;
use std::io::Write;
use std::{io, thread, time};

use config::Config;

/// The main process loop
pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let mut board = Board::new_from_file(&config.input_file);
    loop {
        let activity = calc_board_activity(&board);
        let ratio = board_alive_to_dead_ratio(&board);
        println!("Activity: {}\nRatio: {}", activity, ratio);
        println!("{}", board);
        board.advance_cycle();
        io::stdout().flush()?;
        thread::sleep(time::Duration::from_millis(1000));
    }
}

/// Return the density/ratio of alive to dead cells for each quadrant
fn population_density_by_quad(board: &Board) -> (f32, f32, f32, f32) {
    // Iterate through each quadrant
    // Calculate alive to dead ratio for each

    (0.0, 0.0, 0.0, 0.0)
}

/// Return the ratio of alive to dead cells
fn board_alive_to_dead_ratio(board: &Board) -> f32 {
    let mut alive_ct = 0;

    for x in 0..board.width {
        for y in 0..board.height {
            alive_ct += if board.get(x, y) == Cell::Alive { 1 } else { 0 }
        }
    }

    alive_ct as f32 / (board.width * board.height) as f32
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
